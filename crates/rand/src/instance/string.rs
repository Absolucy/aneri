// SPDX-License-Identifier: MPL-2.0
use super::INSTANCES;
use crate::shared;
use aneri_core::ByondSlotKey;
use meowtonin::byond_fn;

/// `aneri_rand_string` on a `/datum/rng`.
/// Null if the generator was deleted.
#[byond_fn(name = "aneri_raw_instanced_random_string_alphanumeric")]
pub fn instanced_random_string_alphanumeric(src: ByondSlotKey, length: usize) -> Option<String> {
	INSTANCES.with_mut(src, |rng| shared::random_string_alphanumeric(rng, length))
}

/// `aneri_rand_replace_chars_prob` on a `/datum/rng`.
/// Null if the generator was deleted.
#[byond_fn(name = "aneri_raw_instanced_replace_chars_prob")]
pub fn instanced_replace_chars_prob(
	src: ByondSlotKey,
	input: String,
	replacement: String,
	prob: f32,
	skip_whitespace: Option<bool>,
) -> Option<String> {
	INSTANCES.with_mut(src, |rng| {
		shared::replace_chars_prob(rng, input, replacement, prob, skip_whitespace)
	})
}
