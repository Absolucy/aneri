// SPDX-License-Identifier: MPL-2.0
use super::INSTANCES;
use aneri_core::ByondSlotKey;
use rand::Rng;

#[byond_fn]
pub fn instanced_prob(src: ByondSlotKey, probability: f64) -> Option<bool> {
	if !probability.is_finite() {
		return Some(true);
	}
	let probability = (probability / 100.0).clamp(0.0, 1.0);
	INSTANCES
		.lock()
		.get_mut(src)
		.map(|rng| rng.gen_bool(probability))
}

#[byond_fn]
pub fn instanced_prob_ratio(src: ByondSlotKey, numerator: u32, denominator: u32) -> Option<bool> {
	INSTANCES
		.lock()
		.get_mut(src)
		.map(|rng| rng.gen_ratio(numerator, denominator))
}
