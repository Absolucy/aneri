// SPDX-License-Identifier: MPL-2.0
use super::INSTANCES;
use crate::shared;
use aneri_core::ByondSlotKey;
use meowtonin::byond_fn;

/// `aneri_rand_prob` on a `/datum/rng`.
/// Null if the generator was deleted.
#[byond_fn(name = "aneri_raw_instanced_prob")]
pub fn instanced_prob(src: ByondSlotKey, probability: f64) -> Option<bool> {
	INSTANCES.with_mut(src, |rng| shared::prob(rng, probability))
}

/// `aneri_rand_prob_ratio` on a `/datum/rng`.
/// Null if the generator was deleted.
#[byond_fn(name = "aneri_raw_instanced_prob_ratio")]
pub fn instanced_prob_ratio(src: ByondSlotKey, numerator: u32, denominator: u32) -> Option<bool> {
	INSTANCES.with_mut(src, |rng| shared::prob_ratio(rng, numerator, denominator))
}
