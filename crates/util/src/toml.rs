// SPDX-License-Identifier: MPL-2.0
use meowtonin::{ByondError, ByondResult, ByondValue};
use std::path::PathBuf;

#[byond_fn]
pub fn toml_decode(toml: String) -> ByondResult<ByondValue> {
	toml::from_str::<toml::Table>(&toml)
		.map_err(ByondError::boxed)
		.and_then(|table| meowtonin_serde::serialize(&table).map_err(ByondError::boxed))
}

#[byond_fn]
pub fn toml_decode_file(path: PathBuf) -> ByondResult<ByondValue> {
	std::fs::read_to_string(path)
		.map_err(ByondError::boxed)
		.and_then(|file| toml::from_str::<toml::Table>(&file).map_err(ByondError::boxed))
		.and_then(|table| meowtonin_serde::serialize(&table).map_err(ByondError::boxed))
}
