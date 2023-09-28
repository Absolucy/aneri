// SPDX-License-Identifier: MPL-2.0
use std::path::PathBuf;

#[byond_fn]
pub fn crc32(data: String) -> String {
	let crc = crc32fast::hash(data.as_bytes());
	faster_hex::hex_string(&crc.to_le_bytes())
}

#[byond_fn]
pub fn crc32_file(path: PathBuf) -> Option<String> {
	std::fs::read(path).ok().map(|contents| {
		let crc = crc32fast::hash(&contents);
		faster_hex::hex_string(&crc.to_le_bytes())
	})
}

#[byond_fn]
pub fn crc32c(data: String) -> String {
	let crc = crc32c::crc32c(data.as_bytes());
	faster_hex::hex_string(&crc.to_le_bytes())
}

#[byond_fn]
pub fn crc32c_file(path: PathBuf) -> Option<String> {
	std::fs::read(path).ok().map(|contents| {
		let crc = crc32c::crc32c(&contents);
		faster_hex::hex_string(&crc.to_le_bytes())
	})
}
