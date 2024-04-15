// SPDX-License-Identifier: MPL-2.0
use super::global;
use crate::shared;

#[byond_fn]
pub fn random_string_alphanumeric(length: usize, secure: Option<bool>) -> String {
	shared::random_string_alphanumeric(&mut global(secure), length)
}

#[byond_fn]
pub fn replace_chars_prob(
	input: String,
	replacement: String,
	prob: f32,
	skip_whitespace: Option<bool>,
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
