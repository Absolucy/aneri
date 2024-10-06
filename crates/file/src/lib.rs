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

use std::{
	fs::{File, OpenOptions},
	io::{BufRead, BufReader, BufWriter, Read, Write},
	path::PathBuf,
};

#[byond_fn]
pub fn file_exists(path: PathBuf) -> bool {
	path.exists()
}

#[byond_fn]
pub fn file_read(path: PathBuf) -> Option<String> {
	let metadata = std::fs::metadata(&path).ok()?;
	let mut buf = String::with_capacity(metadata.len() as usize);
	let mut file = File::open(path).ok().map(BufReader::new)?;
	file.read_to_string(&mut buf).ok()?;
	Some(buf.replace('\r', ""))
}

#[byond_fn]
pub fn file_write(path: PathBuf, data: String) -> bool {
	let file_write_impl = || -> Option<()> {
		if let Some(parent) = path.parent().filter(|parent| !parent.exists()) {
			std::fs::create_dir_all(parent).ok()?;
		}
		let mut file = File::create(path).ok().map(BufWriter::new)?;
		file.write_all(data.as_bytes()).ok()?;
		file.flush().ok()?;
		file.into_inner().ok()?.sync_all().ok()?;
		Some(())
	};
	file_write_impl().is_some()
}

#[byond_fn]
pub fn file_append(path: PathBuf, data: String) -> bool {
	let file_append_impl = || -> Option<()> {
		if let Some(parent) = path.parent().filter(|parent| !parent.exists()) {
			std::fs::create_dir_all(parent).ok()?;
		}
		let mut file = OpenOptions::new()
			.create(true)
			.append(true)
			.open(path)
			.ok()
			.map(BufWriter::new)?;
		file.write_all(data.as_bytes()).ok()?;
		file.flush().ok()?;
		file.into_inner().ok()?.sync_all().ok()?;
		Some(())
	};
	file_append_impl().is_some()
}

#[byond_fn]
pub fn file_get_line_count(path: PathBuf) -> Option<usize> {
	let mut file = File::open(path).ok().map(BufReader::new)?;
	let mut lines = 0_usize;
	let mut temp_string = String::new();
	loop {
		match file.read_line(&mut temp_string) {
			Ok(0) => break,
			Ok(_) => (),
			Err(_) => return None,
		}
		lines += 1;
		temp_string.clear();
	}
	Some(lines)
}

#[byond_fn]
pub fn file_seek_line(path: PathBuf, line: usize) -> Option<String> {
	let file = File::open(path).ok().map(BufReader::new)?;
	file.lines().nth(line).and_then(|inner| inner.ok())
}

#[byond_fn]
pub fn file_delete(path: PathBuf) -> bool {
	std::fs::remove_file(path).is_ok()
}

#[byond_fn]
pub fn mkdir(path: PathBuf) -> bool {
	path.is_dir() || std::fs::create_dir_all(path).is_ok()
}

#[byond_fn]
pub fn rmdir(path: PathBuf) -> bool {
	std::fs::remove_dir_all(path).is_ok()
}
