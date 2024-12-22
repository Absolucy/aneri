// SPDX-License-Identifier: MPL-2.0

use dmi::icon::Icon as Dmi;
use image::imageops::FilterType;
use meowtonin::{byond_fn, ByondError, ByondResult};
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
pub fn dmi_read_states(path: PathBuf) -> Option<Vec<String>> {
	let icon = File::open(path)
		.map(BufReader::new)
		.ok()
		.and_then(|reader| Dmi::load(reader).ok())?;
	Some(icon.states.into_iter().map(|state| state.name).collect())
}
