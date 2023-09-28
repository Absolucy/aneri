// SPDX-License-Identifier: MPL-2.0
#[macro_use]
extern crate meowtonin;

use std::time::{SystemTime, UNIX_EPOCH};

// Yes, this likely suffers from the year 2038 problem,
// but honestly, if we're still using BYOND in 2038, we fucked up.
#[byond_fn]
pub fn unix_timestamp() -> Option<f32> {
	SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.ok()
		.map(|duration| duration.as_secs_f32())
}
