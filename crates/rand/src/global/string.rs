// SPDX-License-Identifier: MPL-2.0
use super::global;
use crate::shared;
use meowtonin::byond_fn;

/// A random string of `length` letters and digits (`A-Z`, `a-z`, `0-9`).
#[byond_fn(name = "aneri_raw_random_string_alphanumeric")]
pub fn random_string_alphanumeric(
	length: usize,
	/// Use the slower, cryptographically secure generator, default false.
	secure: Option<bool>,
) -> String {
	shared::random_string_alphanumeric(&mut global(secure), length)
}

/// Replaces each character of `input` with `replacement`, each with a `prob` percent chance.
/// Useful for garbling text.
#[byond_fn(name = "aneri_raw_replace_chars_prob")]
pub fn replace_chars_prob(
	input: String,
	/// Text to put in place of a replaced character.
	/// Can be longer than one character.
	replacement: String,
	/// Chance from 0 to 100 for each character.
	prob: f32,
	/// Leave spaces, tabs and newlines alone, default false.
	skip_whitespace: Option<bool>,
	/// Use the slower, cryptographically secure generator, default false.
	secure: Option<bool>,
) -> String {
	shared::replace_chars_prob(
		&mut global(secure),
		input,
		replacement,
		prob,
		skip_whitespace,
	)
}
