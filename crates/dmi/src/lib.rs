// SPDX-License-Identifier: MPL-2.0

use image::imageops::FilterType;
use meowtonin::{ByondError, ByondResult, byond_fn};
use std::{fs::File, io::BufReader, path::PathBuf};

#[byond_fn]
pub fn dmi_resize_png(
	path: PathBuf,
	width: u32,
	height: u32,
	filter: Option<String>,
) -> ByondResult<()> {
	let image = image::open(&path).map_err(ByondError::boxed)?;
	let filter = match filter.as_deref() {
		Some("catmull") => FilterType::CatmullRom,
		Some("triangle") => FilterType::Triangle,
		Some("gaussian") => FilterType::Gaussian,
		Some("lanczos3") => FilterType::Lanczos3,
		_ => FilterType::Nearest,
	};
	let image = image.resize(width, height, filter);
	image.save(path).map_err(ByondError::boxed)
}

#[byond_fn]
pub fn dmi_read_states(path: PathBuf) -> ByondResult<Vec<String>> {
	let file = File::open(path)
		.map(BufReader::new)
		.map_err(ByondError::boxed)?;
	let decoder = png::Decoder::new(file);
	let reader = decoder.read_info().map_err(ByondError::boxed)?;
	let info = reader.info();
	let mut states = Vec::<String>::new();
	for ztxt in &info.compressed_latin1_text {
		let text = ztxt.get_text().map_err(ByondError::boxed)?;
		text.lines()
			.take_while(|line| !line.contains("# END DMI"))
			.filter_map(|line| {
				line.trim()
					.strip_prefix("state = \"")
					.and_then(|line| line.strip_suffix('"'))
			})
			.for_each(|state| {
				states.push(state.to_owned());
			});
	}
	Ok(states)
}
