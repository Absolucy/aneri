// SPDX-License-Identifier: MPL-2.0

use image::{ImageFormat, imageops::FilterType};
use meowtonin::{ByondError, ByondResult, byond_fn};
use std::{
	fs::File,
	io::{BufReader, BufWriter},
	path::{Path, PathBuf},
};

const METADATA_KEYWORD: &str = "Description";

/// Resizes a PNG or DMI and saves it over the original file.
/// For a DMI, `width` and `height` are the size of each frame, and the icon states are kept.
/// For a plain PNG, they're the size of the whole image.
/// Runtimes if the file can't be read or written.
#[byond_fn]
pub fn dmi_resize_png(
	path: PathBuf,
	width: u32,
	height: u32,
	/// One of the `ANERI_RESIZE_*` defines:
	/// `catmull`, `triangle`, `gaussian` or `lanczos3`.
	/// Anything else is nearest-neighbor.
	filter: Option<String>,
) -> ByondResult<()> {
	let metadata = read_metadata(&path)?;
	let file = File::open(&path)
		.map(BufReader::new)
		.map_err(ByondError::boxed)?;
	let image = image::load(file, ImageFormat::Png).map_err(ByondError::boxed)?;
	let filter = match filter.as_deref() {
		Some("catmull") => FilterType::CatmullRom,
		Some("triangle") => FilterType::Triangle,
		Some("gaussian") => FilterType::Gaussian,
		Some("lanczos3") => FilterType::Lanczos3,
		_ => FilterType::Nearest,
	};
	let (sheet_width, sheet_height, metadata) = match metadata {
		Some(metadata) => {
			let columns = image.width() / frame_size(&metadata, "width")?;
			let rows = image.height() / frame_size(&metadata, "height")?;
			let metadata = set_frame_size(&metadata, width, height);
			(columns * width, rows * height, Some(metadata))
		}
		None => (width, height, None),
	};
	let image = image
		.resize_exact(sheet_width, sheet_height, filter)
		.into_rgba8();

	let file = File::create(&path)
		.map(BufWriter::new)
		.map_err(ByondError::boxed)?;
	let mut encoder = png::Encoder::new(file, sheet_width, sheet_height);
	encoder.set_color(png::ColorType::Rgba);
	encoder.set_depth(png::BitDepth::Eight);
	if let Some(metadata) = metadata {
		encoder
			.add_ztxt_chunk(METADATA_KEYWORD.to_owned(), metadata)
			.map_err(ByondError::boxed)?;
	}
	let mut writer = encoder.write_header().map_err(ByondError::boxed)?;
	writer.write_image_data(&image).map_err(ByondError::boxed)?;
	writer.finish().map_err(ByondError::boxed)
}

/// Lists the icon state names in a .dmi file without decoding its pixels.
/// Runtimes if the file can't be read or isn't a PNG.
#[byond_fn(name = "aneri_raw_dmi_read_states")]
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

/// The DMI metadata text, or `None` for a plain PNG.
fn read_metadata(path: &Path) -> ByondResult<Option<String>> {
	let file = File::open(path)
		.map(BufReader::new)
		.map_err(ByondError::boxed)?;
	let reader = png::Decoder::new(file)
		.read_info()
		.map_err(ByondError::boxed)?;
	for chunk in &reader.info().compressed_latin1_text {
		if chunk.keyword != METADATA_KEYWORD {
			continue;
		}
		let text = chunk.get_text().map_err(ByondError::boxed)?;
		if text.contains("# BEGIN DMI") {
			return Ok(Some(text));
		}
	}
	Ok(None)
}

fn frame_size(metadata: &str, key: &str) -> ByondResult<u32> {
	metadata
		.lines()
		.find_map(|line| {
			line.trim()
				.strip_prefix(key)?
				.strip_prefix(" = ")?
				.parse()
				.ok()
		})
		.filter(|&size| size > 0)
		.ok_or_else(|| {
			ByondError::Boxed(Box::<dyn std::error::Error + Send + Sync>::from(format!(
				"dmi metadata has no valid {key}"
			)))
		})
}

fn set_frame_size(metadata: &str, width: u32, height: u32) -> String {
	metadata
		.split('\n')
		.map(|line| match line.trim() {
			trimmed if trimmed.starts_with("width = ") => format!("\twidth = {width}"),
			trimmed if trimmed.starts_with("height = ") => format!("\theight = {height}"),
			_ => line.to_owned(),
		})
		.collect::<Vec<_>>()
		.join("\n")
}
