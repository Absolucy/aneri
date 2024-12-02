// SPDX-License-Identifier: MPL-2.0
use super::INSTANCES;
use crate::shared;
use aneri_core::ByondSlotKey;
use meowtonin::byond_fn;

#[byond_fn]
pub fn instanced_random_string_alphanumeric(src: ByondSlotKey, length: usize) -> Option<String> {
	INSTANCES
		.lock()
		.get_mut(src)
		.map(|rng| shared::random_string_alphanumeric(rng, length))
}

#[byond_fn]
pub fn instanced_replace_chars_prob(
	src: ByondSlotKey,
	input: String,
	replacement: String,
	prob: f32,
	skip_whitespace: Option<bool>,
) -> Option<String> {
	INSTANCES
		.lock()
		.get_mut(src)
		.map(|rng| shared::replace_chars_prob(rng, input, replacement, prob, skip_whitespace))
}
