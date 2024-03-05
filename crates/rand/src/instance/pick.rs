// SPDX-License-Identifier: MPL-2.0
use super::instances;
use aneri_core::ByondSlotKey;
use meowtonin::{ByondResult, ByondValue};
use rand::{
	distributions::{Distribution, WeightedIndex},
	Rng,
};
use std::collections::HashMap;

#[byond_fn]
pub fn instanced_pick(src: ByondSlotKey, options: ByondValue) -> ByondResult<Option<ByondValue>> {
	if !options.is_list() {
		return Ok(None);
	}
	let length = options.length::<usize>()?;
	match length {
		0 => return Ok(None),
		1 => return options.read_list_index(&1).map(Some),
		_ => {}
	}
	let mut instances = instances().lock();
	let rng = match instances.get_mut(src) {
		Some(rng) => rng,
		None => return Ok(None),
	};
	let idx = rng.gen_range::<usize, _>(1..=length);
	options.read_list_index(&idx).map(Some)
}

#[byond_fn]
pub fn instanced_pick_weighted(
	src: ByondSlotKey,
	options: HashMap<ByondValue, f32, ahash::RandomState>,
) -> Option<ByondValue> {
	if options.is_empty() {
		return None;
	}
	let (values, weights): (Vec<_>, Vec<_>) = options
		.into_iter()
		.filter(|(_, weight)| weight.is_normal() && weight.is_sign_positive())
		.unzip();
	match values.len() {
		0 => return None,
		1 => return values.into_iter().next(),
		_ => {}
	}
	let mut instances = instances().lock();
	let rng = instances.get_mut(src)?;
	let dist = WeightedIndex::new(weights).ok()?;
	values.into_iter().nth(dist.sample(&mut *rng))
}
