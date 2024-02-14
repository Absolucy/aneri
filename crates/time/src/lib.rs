// SPDX-License-Identifier: MPL-2.0
#[macro_use]
extern crate meowtonin;

use std::time::{SystemTime, UNIX_EPOCH};

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
