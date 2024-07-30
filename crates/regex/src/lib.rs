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

pub mod global;
pub mod instance;

pub fn cleanup() {
	global::clear_cache();
	instance::free_instances();
}

#[byond_fn]
pub fn regex_escape(pattern: String) -> String {
	regex::escape(&pattern)
}
