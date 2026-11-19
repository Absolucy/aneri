// SPDX-License-Identifier: MPL-2.0
use meowtonin::byond_fn;
use std::{ffi::OsStr, fs::File, path::PathBuf};
use symphonia::{
	core::{
		formats::FormatOptions,
		io::{MediaSourceStream, MediaSourceStreamOptions},
		meta::MetadataOptions,
		probe::Hint,
	},
	default::get_probe,
};

#[byond_fn]
pub fn audio_length(path: PathBuf) -> Option<f32> {
	// If the path has an extension, include it as a hint.
	let mut hint = Hint::new();
	if let Some(extension) = path.extension().and_then(OsStr::to_str) {
		hint.with_extension(extension);
	}

	let file = File::open(&path)
		.map(Box::new)
		.expect("failed to open audio file");

	let mss = MediaSourceStream::new(file, MediaSourceStreamOptions::default());

	let format_opts = FormatOptions {
		enable_gapless: true,
		..FormatOptions::default()
	};
	let metadata_opts = MetadataOptions::default();

	let probed = get_probe()
		.format(&hint, mss, &format_opts, &metadata_opts)
		.expect("failed to probe audio");

	let params = &probed.format.default_track()?.codec_params;
	let n_frames = params.n_frames?;
	let seconds = match params.time_base {
		Some(time_base) => {
			let time = time_base.calc_time(n_frames);
			time.seconds as f64 + time.frac
		}
		None => n_frames as f64 / f64::from(params.sample_rate?),
	};

	Some((seconds * 10.0) as f32)
}
