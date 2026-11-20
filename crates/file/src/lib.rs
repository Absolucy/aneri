// SPDX-License-Identifier: MPL-2.0

use meowtonin::byond_fn;
use std::{
	fs::{File, OpenOptions},
	io::{BufRead, BufReader, BufWriter, Read, Write},
	path::PathBuf,
};

/// Whether a file or folder exists at `path`.
#[byond_fn]
pub fn file_exists(path: PathBuf) -> bool {
	path.exists()
}

/// Reads a whole text file.
/// Every `\r` is removed, so Windows line endings come back as `\n`.
/// Returns null if the file can't be read or isn't valid UTF-8.
#[byond_fn]
pub fn file_read(path: PathBuf) -> Option<String> {
	let metadata = std::fs::metadata(&path).ok()?;
	let mut buf = String::with_capacity(metadata.len() as usize);
	let mut file = File::open(path).ok().map(BufReader::new)?;
	file.read_to_string(&mut buf).ok()?;
	Some(buf.replace('\r', ""))
}

/// Writes text to a file, replacing what was there.
/// Creates missing parent folders.
/// Returns false on failure.
#[byond_fn]
pub fn file_write(path: PathBuf, data: String) -> bool {
	let file_write_impl = || -> Option<()> {
		if let Some(parent) = path.parent().filter(|parent| !parent.exists()) {
			std::fs::create_dir_all(parent).ok()?;
		}
		// write next to it and rename over it, so a crash mid-write leaves the old file intact
		let mut tmp_path = path.clone().into_os_string();
		tmp_path.push(".tmp");
		let tmp_path = PathBuf::from(tmp_path);
		let mut file = BufWriter::new(File::create(&tmp_path).ok()?);
		file.write_all(data.as_bytes()).ok()?;
		file.into_inner().ok()?.sync_all().ok()?;
		std::fs::rename(&tmp_path, &path).ok()
	};
	file_write_impl().is_some()
}

/// Adds text to the end of a file, creating it and any missing parent folders.
/// No newline is added.
/// Returns false on failure.
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
		Some(())
	};
	file_append_impl().is_some()
}

/// Counts the `\n` characters in a file,
/// so a last line with no newline after it isn't counted.
/// Returns null if the file can't be opened.
#[byond_fn]
pub fn file_get_line_count(path: PathBuf) -> Option<usize> {
	let mut file = File::open(path).ok().map(BufReader::new)?;
	let mut lines = 0_usize;
	let mut buffer = [0; 8192];
	while let Ok(bytes_read) = file.read(&mut buffer) {
		if bytes_read == 0 {
			break;
		}
		lines += buffer[..bytes_read].iter().filter(|&&b| b == b'\n').count();
	}
	Some(lines)
}

/// Reads one line of a file, without its line ending.
/// It reads from the start of the file every call,
/// so use `aneri_file_read` to go through a whole file.
/// Returns null if the file can't be read or is too short.
#[byond_fn]
pub fn file_seek_line(
	path: PathBuf,
	/// Which line, counting from 0.
	line: usize,
) -> Option<String> {
	let mut file = File::open(path).ok().map(BufReader::new)?;
	for _ in 0..line {
		if file.skip_until(b'\n').ok()? == 0 {
			return None;
		}
	}
	let mut text = String::new();
	if file.read_line(&mut text).ok()? == 0 {
		return None;
	}
	if text.ends_with('\n') {
		text.pop();
		if text.ends_with('\r') {
			text.pop();
		}
	}
	Some(text)
}

/// Deletes a file.
/// Returns false on failure, including when it didn't exist.
#[byond_fn]
pub fn file_delete(path: PathBuf) -> bool {
	std::fs::remove_file(path).is_ok()
}

/// Creates a folder and any missing parents.
/// Returns true if the folder exists afterwards, even if it already did.
#[byond_fn]
pub fn mkdir(path: PathBuf) -> bool {
	path.is_dir() || std::fs::create_dir_all(path).is_ok()
}

/// Deletes a folder and everything inside it.
/// Returns false on failure.
#[byond_fn]
pub fn rmdir(path: PathBuf) -> bool {
	std::fs::remove_dir_all(path).is_ok()
}
