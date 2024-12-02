// SPDX-License-Identifier: MPL-2.0
use super::global;
use crate::shared;
use meowtonin::byond_fn;

#[byond_fn]
pub fn prob(probability: f64, secure: Option<bool>) -> bool {
	shared::prob(&mut global(secure), probability)
}

#[byond_fn]
pub fn prob_ratio(numerator: u32, denominator: u32, secure: Option<bool>) -> bool {
	shared::prob_ratio(&mut global(secure), numerator, denominator)
}
