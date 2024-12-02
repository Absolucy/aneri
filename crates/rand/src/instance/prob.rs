// SPDX-License-Identifier: MPL-2.0
use super::INSTANCES;
use crate::shared;
use aneri_core::ByondSlotKey;
use meowtonin::byond_fn;

#[byond_fn]
pub fn instanced_prob(src: ByondSlotKey, probability: f64) -> Option<bool> {
	INSTANCES
		.lock()
		.get_mut(src)
		.map(|rng| shared::prob(rng, probability))
}

#[byond_fn]
pub fn instanced_prob_ratio(src: ByondSlotKey, numerator: u32, denominator: u32) -> Option<bool> {
	INSTANCES
		.lock()
		.get_mut(src)
		.map(|rng| shared::prob_ratio(rng, numerator, denominator))
}
