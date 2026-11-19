// SPDX-License-Identifier: MPL-2.0

pub(crate) mod counter;
pub mod message;
pub mod queue;
pub mod writer;

use meowtonin::byond_fn;
use std::path::PathBuf;

/// Queues a message to be appended to a log file.
/// A background thread does the writing, so this never waits on the disk.
/// Returns false only if the log file's writer has shut down.
#[byond_fn]
pub fn log_write(
	path: PathBuf,
	message: String,
	/// When true (the default), the first line gets a UTC timestamp,
	/// later lines start with ` - `, and a newline is added.
	/// False writes `message` exactly as given.
	format: Option<bool>,
) -> bool {
	queue::get_queue(&path)
		.send(message::LogMessage::new(message, format.unwrap_or(true)))
		.is_ok()
}

/// Finishes writing every queued message, then closes all log files.
/// Waits up to 10 seconds.
/// The next `aneri_log_write` to a path opens it again.
#[byond_fn]
pub fn log_close_all() {
	queue::clear_log_queue();
}
