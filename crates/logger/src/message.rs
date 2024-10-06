// SPDX-License-Identifier: MPL-2.0
use time::OffsetDateTime;

/// A log message and a timestamp of when it was logged.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(target_pointer_width = "32", repr(align(64)))]
#[cfg_attr(target_pointer_width = "64", repr(align(128)))]
pub(crate) struct LogMessage {
	/// The time the message was logged.
	pub timestamp: OffsetDateTime,
	/// The log message itself.
	pub message: String,
	/// Whether to format the message or not.
	pub format: bool,
}

impl LogMessage {
	/// Creates a new log message at the current in time.
	pub fn new(message: impl Into<String>, format: bool) -> Self {
		let timestamp = OffsetDateTime::now_utc();
		let message = message.into();
		Self {
			timestamp,
			message,
			format,
		}
	}
}
