// SPDX-License-Identifier: MPL-2.0

pub mod capture;
pub mod global;
pub mod instance;
pub mod shared;

use meowtonin::byond_fn;

pub fn cleanup() {
	global::clear_cache();
	instance::free_instances();
}

/// Escapes every special regex character in `pattern`.
/// The result matches `pattern` as plain text.
#[byond_fn]
pub fn regex_escape(pattern: String) -> String {
	regex::escape(&pattern)
}

/// Escapes each string in `patterns` and joins them with `|`.
/// The result matches any one of them as plain text.
/// One call instead of calling `aneri_regex_escape` in a loop.
#[byond_fn]
pub fn regex_combine_list(patterns: Vec<String>) -> String {
	patterns
		.into_iter()
		.map(|pattern| regex::escape(&pattern))
		.collect::<Vec<String>>()
		.join("|")
}
