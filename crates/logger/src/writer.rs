// SPDX-License-Identifier: MPL-2.0

use crate::{counter::THREAD_COUNTER, message::LogMessage};
use crossbeam_channel::Receiver;
use std::{
	fs::OpenOptions,
	io::{BufWriter, Write},
	path::PathBuf,
};
use thread_priority::{ThreadBuilderExt, ThreadPriority};

fn log_thread(path: PathBuf, rx: Receiver<LogMessage>) {
	let counter = THREAD_COUNTER.read().clone();
	counter.increment();
	scopeguard::defer! {
		counter.decrement();
	}
	let mut file = OpenOptions::new()
		.append(true)
		.create(true)
		.open(&path)
		.map(BufWriter::new)
		.unwrap_or_else(|err| panic!("failed to open log file at {}: {err:?}", path.display()));
	while let Ok(log) = rx.recv() {
		if log.format {
			let mut lines = log.message.lines();
			if let Some(first) = lines.next() {
				let _ = writeln!(file, "[{}] {}", log.timestamp.format("%F %T%.3f"), first);
				for line in lines {
					let _ = writeln!(file, " - {}", line);
				}
			}
		} else {
			let _ = write!(file, "{}", log.message);
		}
		let _ = file.flush();
	}
	file.into_inner()
		.expect("failed to collapse")
		.sync_all()
		.expect("failed to sync log file");
}

/// Spawns a thread that writes log messages to the specified path.
/// This thread will stop whenever the sender is dropped, after synchronizing
/// the opened log file.
pub(crate) fn spawn_log_thread(path: PathBuf, rx: Receiver<LogMessage>) {
	// Spawned with min priority in order to avoid potential performance impact to
	// the actual server.
	std::thread::Builder::new()
		.name(format!("aneri logger [{}]", path.display()))
		.spawn_with_priority(ThreadPriority::Min, move |_| log_thread(path, rx))
		.expect("failed to spawn logger thread");
}
