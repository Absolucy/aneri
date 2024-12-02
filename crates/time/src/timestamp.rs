// SPDX-License-Identifier: MPL-2.0
use meowtonin::byond_fn;
use std::time::{SystemTime, UNIX_EPOCH};
use time::OffsetDateTime;

/**
 * so this weird shit is in fact quite a bit faster than the usual format!()
 * approach. i benchmarked it on my Ryzen 9 5900X (i686-pc-windows-msvc
 * target, to better represent actual use) this function took around 80ns on
 * average, while the usual format!() approach takes nearly 200ns on
 * average - both excluding the SystemTime::now().duration_since(UNIX_EPOCH)
 * stuff, mind you.
 *
 * all this for a microscopic improvement over the much easier
 * `format!("{:.6}", duration.as_secs_f64())`
 * but this is a very hot proc, due to logging stuff, so if it counts
 * anywhere, it'll be here, i guess?
 */
#[byond_fn]
pub fn unix_timestamp() -> String {
	let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
	let secs = duration.as_secs();
	let frac = duration.as_secs_f64().fract();
	let mut micros = (frac * 1_000_000.0).round() as u32;

	let mut buffer = [b'0'; 17];
	let mut pos = if secs == 0 {
		1
	} else {
		let mut s = secs;
		let mut p = 0;
		while s > 0 {
			unsafe { std::hint::assert_unchecked(p < 10) };
			buffer[9 - p] = (s % 10) as u8 + b'0';
			s /= 10;
			p += 1;
		}
		if p < 10 {
			buffer.copy_within(10 - p..10, 0);
		}
		p
	};

	buffer[pos] = b'.';
	pos += 1;

	for i in (0..6).rev() {
		buffer[pos + i] = (micros % 10) as u8 + b'0';
		micros /= 10;
	}
	pos += 6;

	unsafe { String::from_utf8_unchecked(buffer[..pos].to_vec()) }
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
