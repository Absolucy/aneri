// SPDX-License-Identifier: MPL-2.0
use crate::message::LogMessage;
use crossbeam_channel::Receiver;
use std::{
	fs::OpenOptions,
	io::{BufWriter, Write},
	path::PathBuf,
};
use thread_counter::Ticket;
use thread_priority::{ThreadBuilderExt, ThreadPriority};
use time::format_description::BorrowedFormatItem;

/// Default stack size for logger threads (1 MiB)
pub const DEFAULT_STACK_SIZE: usize = 1024 * 1024;

static TIMESTAMP_FORMATTER: &[BorrowedFormatItem] = time::macros::format_description!(
	"[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
);

fn log_thread(path: PathBuf, rx: Receiver<LogMessage>, _ticket: Ticket) {
	let mut file = OpenOptions::new()
		.append(true)
		.create(true)
		.open(&path)
		.map(BufWriter::new)
		.unwrap_or_else(|err| panic!("failed to open log file at {}: {err:?}", path.display()));
	while let Ok(log) = rx.recv() {
		write_message(&mut file, &log);
		while let Ok(log) = rx.try_recv() {
			write_message(&mut file, &log);
		}
		let _ = file.flush();
	}
	file.into_inner()
		.expect("failed to collapse")
		.sync_all()
		.expect("failed to sync log file");
}

fn write_message(file: &mut impl Write, log: &LogMessage) {
	if log.format {
		let mut lines = log.message.lines();
		if let Some(first) = lines.next() {
			let _ = writeln!(
				file,
				"[{}] {}",
				log.timestamp
					.format(TIMESTAMP_FORMATTER)
					.unwrap_or_else(|_| unreachable!("invalid formatter somehow??")),
				first
			);
			for line in lines {
				let _ = writeln!(file, " - {line}");
			}
		}
	} else {
		let _ = file.write_all(log.message.as_bytes());
	}
}

/// Spawns a thread that writes log messages to the specified path.
/// This thread will stop whenever the sender is dropped, after synchronizing
/// the opened log file.
pub(crate) fn spawn_log_thread(path: PathBuf, rx: Receiver<LogMessage>) {
	let ticket = crate::counter::take_thread_ticket();
	// Spawned with min priority in order to avoid potential performance impact
	// to the actual server.
	std::thread::Builder::new()
		.name(format!("aneri logger [{}]", path.display()))
		.stack_size(DEFAULT_STACK_SIZE)
		.spawn_with_priority(ThreadPriority::Min, move |_| log_thread(path, rx, ticket))
		.expect("failed to spawn logger thread");
}
