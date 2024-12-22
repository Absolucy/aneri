// SPDX-License-Identifier: MPL-2.0

pub(crate) mod counter;
pub mod message;
pub mod queue;
pub mod writer;

use meowtonin::byond_fn;
use std::path::PathBuf;

/// Logs a message to the specified path.
#[byond_fn]
pub fn log_write(path: PathBuf, message: String, format: Option<bool>) -> bool {
	if let Some(parent) = path.parent().filter(|path| !path.is_dir()) {
		std::fs::create_dir_all(parent).expect("failed to create log directory");
	}
	queue::get_queue(&path)
		.send(message::LogMessage::new(message, format.unwrap_or(true)))
		.is_ok()
}

/// Closes all active logfiles.
#[byond_fn]
pub fn log_close_all() {
	queue::clear_log_queue();
}
