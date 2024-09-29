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

pub mod capture;
pub mod global;
pub mod instance;
pub mod shared;

pub fn cleanup() {
	global::clear_cache();
	instance::free_instances();
}

/// Escapes all regular expression meta characters in `pattern`.
/// The string returned may be safely used as a literal in a regular expression.
#[byond_fn]
pub fn regex_escape(pattern: String) -> String {
	regex::escape(&pattern)
}

/// Helper function to escape and combine a list of patterns to match.
/// This is just so we only have to do one call_ext instead of doing it in a
/// loop.
/// The return value will use `|` as a separator.
#[byond_fn]
pub fn regex_combine_list(patterns: Vec<String>) -> String {
	patterns
		.into_iter()
		.map(|pattern| regex::escape(&pattern))
		.collect::<Vec<String>>()
		.join("|")
}
