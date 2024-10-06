// SPDX-License-Identifier: MPL-2.0
use std::time::{SystemTime, UNIX_EPOCH};
use time::OffsetDateTime;

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
pub fn human_readable_timestamp() -> String {
	let formatter = time::macros::format_description!(
		"[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
	);
	OffsetDateTime::now_utc()
		.format(&formatter)
		.unwrap_or_else(|_| unreachable!("invalid formatter?"))
}
