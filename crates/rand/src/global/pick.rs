// SPDX-License-Identifier: MPL-2.0
use super::global;
use meowtonin::ByondValue;
use rand::{
	distributions::{Distribution, WeightedIndex},
	Rng,
};
use std::collections::HashMap;

#[byond_fn]
pub fn pick(options: Vec<ByondValue>, secure: Option<bool>) -> Option<ByondValue> {
	if options.is_empty() {
		None
	} else {
		let length = options.len();
		options.into_iter().nth(global(secure).gen_range(0..length))
	}
}

#[byond_fn]
pub fn pick_weighted(
	options: HashMap<ByondValue, f32, ahash::RandomState>,
	secure: Option<bool>,
) -> Option<ByondValue> {
	if options.is_empty() {
		None
	} else {
		let weights = options.values().copied().collect::<Vec<f32>>();
		let dist = WeightedIndex::new(weights).ok()?;
		options
			.into_iter()
			.nth(dist.sample(&mut global(secure)))
			.map(|(value, _)| value)
	}
}
