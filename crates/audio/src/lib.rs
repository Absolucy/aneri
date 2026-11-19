// SPDX-License-Identifier: MPL-2.0
use meowtonin::{ByondError, ByondResult, ByondValue, ToByond, byond_fn};
use std::{
	ffi::OsStr,
	fs::File,
	num::NonZeroUsize,
	panic::resume_unwind,
	path::{Path, PathBuf},
	thread,
};
use symphonia::{
	core::{
		codecs::CodecParameters,
		errors::Error as SymphoniaError,
		formats::FormatOptions,
		io::{MediaSourceStream, MediaSourceStreamOptions},
		meta::MetadataOptions,
		probe::Hint,
	},
	default::get_probe,
};

/// How long a sound file is, in deciseconds.
/// Reads the length from the file header without decoding the audio.
/// Returns null if the header has no length.
/// Runtimes if the file can't be opened or isn't a format it knows.
#[byond_fn]
pub fn audio_length(path: PathBuf) -> ByondResult<Option<f32>> {
	read_length(&path).map_err(ByondError::boxed)
}

/// How long each sound file in a list is, in deciseconds.
/// Returns `list("successes" = list(path = length), "errors" = list(path = message))`,
/// where every path you passed appears in exactly one of the two.
/// A file whose header has no length goes in "errors".
/// Files are read in parallel. Runtimes if an entry isn't a string.
#[byond_fn(name = "aneri_raw_audio_length_list")]
pub fn audio_length_list(paths: Vec<String>) -> ByondResult<Vec<(&'static str, ByondValue)>> {
	let workers = thread::available_parallelism().map_or(1, NonZeroUsize::get);
	let chunk_size = paths.len().div_ceil(workers).max(1);
	let results = thread::scope(|scope| {
		paths
			.chunks(chunk_size)
			.map(|chunk| {
				scope.spawn(move || {
					chunk
						.iter()
						.map(|path| read_length(Path::new(path)))
						.collect::<Vec<_>>()
				})
			})
			.collect::<Vec<_>>()
			.into_iter()
			.flat_map(|worker| worker.join().unwrap_or_else(|panic| resume_unwind(panic)))
			.collect::<Vec<_>>()
	});

	let mut successes = Vec::new();
	let mut errors = Vec::new();
	for (path, result) in paths.into_iter().zip(results) {
		match result {
			Ok(Some(length)) => successes.push((path, length)),
			Ok(None) => errors.push((path, "no length in file header".to_owned())),
			Err(error) => errors.push((path, error.to_string())),
		}
	}

	Ok(vec![
		("successes", successes.to_byond()?),
		("errors", errors.to_byond()?),
	])
}

fn read_length(path: &Path) -> Result<Option<f32>, SymphoniaError> {
	// If the path has an extension, include it as a hint.
	let mut hint = Hint::new();
	if let Some(extension) = path.extension().and_then(OsStr::to_str) {
		hint.with_extension(extension);
	}

	let file = File::open(path).map(Box::new)?;

	let mss = MediaSourceStream::new(file, MediaSourceStreamOptions::default());

	let format_opts = FormatOptions {
		enable_gapless: true,
		..FormatOptions::default()
	};
	let metadata_opts = MetadataOptions::default();

	let probed = get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;

	Ok(probed
		.format
		.default_track()
		.and_then(|track| length_deciseconds(&track.codec_params)))
}

fn length_deciseconds(params: &CodecParameters) -> Option<f32> {
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
