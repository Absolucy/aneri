// SPDX-License-Identifier: MPL-2.0
#![warn(
	clippy::correctness,
	clippy::suspicious,
	clippy::complexity,
	clippy::perf,
	clippy::style
)]

#[macro_use]
extern crate meowtonin;

pub mod message;
pub mod queue;
pub mod writer;

use std::path::PathBuf;

/// Logs a message to the specified path.
#[byond_fn]
pub fn log_write(path: PathBuf, message: String, format: Option<bool>) -> bool {
	queue::get_queue(&path)
		.send(message::LogMessage::new(message, format.unwrap_or(true)))
		.is_ok()
}

/// Closes all active logfiles.
#[byond_fn]
pub fn log_close_all() {
	queue::clear_log_queue();
}
