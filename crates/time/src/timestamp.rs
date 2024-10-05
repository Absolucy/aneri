// SPDX-License-Identifier: MPL-2.0
use chrono::Local;
use std::{
	fmt::Write,
	time::{SystemTime, UNIX_EPOCH},
};

#[byond_fn]
pub fn unix_timestamp() -> String {
	format!(
		"{:.6}",
		SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_secs_f64()
	)
}

#[byond_fn]
pub fn human_readable_timestamp(millis_precision: usize) -> String {
	let now = Local::now();
	let mut result = now.format("%Y-%m-%d %H:%M:%S").to_string();

	if millis_precision > 0 {
		let millis = now.timestamp_subsec_millis();
		let _ = write!(result, ".{:0width$}", millis, width = millis_precision);
	}

	result
}
