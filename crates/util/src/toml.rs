// SPDX-License-Identifier: MPL-2.0
use meowtonin::{ByondError, ByondResult, ByondValue, byond_fn};
use std::path::PathBuf;

/// Whether `toml` is a valid TOML document.
#[byond_fn]
pub fn toml_is_valid(toml: String) -> bool {
	toml::from_str::<toml::Table>(&toml).is_ok()
}

/// Whether the file at `path` is a valid TOML document.
/// Returns false if it can't be read.
#[byond_fn(name = "aneri_raw_toml_file_is_valid")]
pub fn toml_file_is_valid(path: PathBuf) -> bool {
	std::fs::read_to_string(path)
		.ok()
		.and_then(|file| toml::from_str::<toml::Table>(&file).ok())
		.is_some()
}

/// Decodes a TOML document into an assoc list.
/// Runtimes if it isn't valid TOML.
#[byond_fn]
pub fn toml_decode(toml: String) -> ByondResult<ByondValue> {
	toml::from_str::<toml::Table>(&toml)
		.map_err(ByondError::boxed)
		.and_then(|table| meowtonin_serde::serialize(&table).map_err(ByondError::boxed))
}

/// Decodes a TOML file into an assoc list.
/// Runtimes if it can't be read or isn't valid TOML.
#[byond_fn(name = "aneri_raw_toml_decode_file")]
pub fn toml_decode_file(path: PathBuf) -> ByondResult<ByondValue> {
	std::fs::read_to_string(path)
		.map_err(ByondError::boxed)
		.and_then(|file| toml::from_str::<toml::Table>(&file).map_err(ByondError::boxed))
		.and_then(|table| meowtonin_serde::serialize(&table).map_err(ByondError::boxed))
}
