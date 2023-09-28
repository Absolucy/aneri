// SPDX-License-Identifier: MPL-2.0
use super::instances;
use aneri_core::ByondSlotKey;
use meowtonin::ByondValue;
use rand::{
	distributions::{Distribution, WeightedIndex},
	Rng,
};
use std::collections::HashMap;

#[byond_fn]
pub fn instanced_pick(src: ByondSlotKey, options: Vec<ByondValue>) -> Option<ByondValue> {
	if options.is_empty() {
		None
	} else {
		let mut instances = instances().lock();
		let rng = instances.get_mut(src)?;
		let length = options.len();
		options.into_iter().nth(rng.gen_range(0..length))
	}
}

#[byond_fn]
pub fn instanced_pick_weighted(
	src: ByondSlotKey,
	options: HashMap<ByondValue, f32, ahash::RandomState>,
) -> Option<ByondValue> {
	if options.is_empty() {
		None
	} else {
		let mut instances = instances().lock();
		let rng = instances.get_mut(src)?;
		let (values, weights): (Vec<_>, Vec<_>) = options
			.into_iter()
			.filter(|(_, weight)| weight.is_normal() && weight.is_sign_positive())
			.unzip();
		let dist = WeightedIndex::new(weights).ok()?;
		values.into_iter().nth(dist.sample(&mut *rng))
	}
}
