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

use std::{fs::File, path::PathBuf, time::Duration};
use symphonia::core::{
	formats::FormatOptions,
	io::{MediaSourceStream, MediaSourceStreamOptions},
	meta::MetadataOptions,
	probe::Hint,
};

#[byond_fn]
pub fn audio_length(path: PathBuf) -> Option<u32> {
	// Open the file
	let file = File::open(path).ok()?;

	// Create a media source stream
	let mss = MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());

	// Create a hint to help with format detection
	let hint = Hint::new();

	// Use default options for format and metadata
	let format_opts = FormatOptions::default();
	let metadata_opts = MetadataOptions::default();

	// Probe the media source
	let probed = symphonia::default::get_probe()
		.format(&hint, mss, &format_opts, &metadata_opts)
		.ok()?;

	// Get the default track
	let track = probed.format.default_track()?;

	// Calculate the duration
	let time_base = track.codec_params.time_base?;
	let duration = track.codec_params.n_frames.map(|frames| {
		let time = time_base.calc_time(frames);
		Duration::from_secs(time.seconds) + Duration::from_secs_f64(time.frac)
	})?;

	// Convert to deciseconds
	Some((duration.as_secs_f64() * 10.0).ceil() as u32)
}
