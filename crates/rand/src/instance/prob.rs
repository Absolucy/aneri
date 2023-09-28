// SPDX-License-Identifier: MPL-2.0
use super::instances;
use aneri_core::ByondSlotKey;
use rand::Rng;

#[byond_fn]
pub fn instanced_prob(src: ByondSlotKey, probability: f64) -> Option<bool> {
	instances()
		.lock()
		.get_mut(src)
		.map(|rng| rng.gen_bool(probability / 100.0))
}

#[byond_fn]
pub fn instanced_prob_ratio(src: ByondSlotKey, numerator: u32, denominator: u32) -> Option<bool> {
	instances()
		.lock()
		.get_mut(src)
		.map(|rng| rng.gen_ratio(numerator, denominator))
}
