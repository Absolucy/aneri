// SPDX-License-Identifier: MPL-2.0
use meowtonin::byond_fn;
use std::{ffi::OsStr, fs::File, path::PathBuf, time::Duration};
use symphonia::{
	core::{
		codecs::DecoderOptions,
		formats::FormatOptions,
		io::{MediaSourceStream, MediaSourceStreamOptions},
		meta::MetadataOptions,
		probe::{Hint, ProbeResult},
	},
	default::{get_codecs, get_probe},
};

#[byond_fn]
pub fn audio_length(path: PathBuf) -> f32 {
	// If the path has an extension, include it as a hint.
	let mut hint = Hint::new();
	if let Some(extension) = path.extension().and_then(OsStr::to_str) {
		hint.with_extension(extension);
	}

	let file = File::open(&path)
		.map(Box::new)
		.expect("failed to open audio file");

	let mss = MediaSourceStream::new(file, MediaSourceStreamOptions::default());

	let format_opts = FormatOptions::default();
	let metadata_opts = MetadataOptions::default();

	let probed = get_probe()
		.format(&hint, mss, &format_opts, &metadata_opts)
		.expect("failed to probe audio");

	sound_length_simple(&probed).unwrap_or_else(|| sound_length_decode(probed)) as f32
}

fn sound_length_simple(probed: &ProbeResult) -> Option<f64> {
	let track = probed.format.default_track()?;
	let time_base = track.codec_params.time_base?;
	let n_frames = track.codec_params.n_frames?;

	let time = time_base.calc_time(n_frames);
	let duration = Duration::from_secs(time.seconds) + Duration::from_secs_f64(time.frac);

	Some(duration.as_secs_f64() * 10.0)
}

fn sound_length_decode(probed: ProbeResult) -> f64 {
	let mut format = probed.format;

	let track = format.default_track().expect("could not get default track");

	// Grab the number of frames of the track
	let samples_capacity = track.codec_params.n_frames.unwrap_or(0) as f64;

	// Create a decoder using the provided codec parameters in the track.
	let decoder_opts = DecoderOptions::default();
	let mut decoder = get_codecs()
		.make(&track.codec_params, &decoder_opts)
		.expect("decoder creation error");

	// Try to grab a data packet from the container
	let encoded_packet = format.next_packet().expect("next_packet error");

	// Try to decode the data packet
	let decoded_packet = decoder
		.decode(&encoded_packet)
		.expect("failed to decode packet");

	// Grab the sample rate from the spec of the buffer.
	let sample_rate = decoded_packet.spec().rate as f64;
	// Math!
	samples_capacity / sample_rate * 10.0
}
