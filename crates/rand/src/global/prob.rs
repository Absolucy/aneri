// SPDX-License-Identifier: MPL-2.0
use super::global;
use rand::Rng;

#[byond_fn]
pub fn prob(probability: f64, secure: Option<bool>) -> bool {
	global(secure).gen_bool(probability / 100.0)
}

#[byond_fn]
pub fn prob_ratio(numerator: u32, denominator: u32, secure: Option<bool>) -> bool {
	global(secure).gen_ratio(numerator, denominator)
}
