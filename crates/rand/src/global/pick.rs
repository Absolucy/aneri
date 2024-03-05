// SPDX-License-Identifier: MPL-2.0
use super::global;
use meowtonin::{ByondResult, ByondValue};
use rand::{
	distributions::{Distribution, WeightedIndex},
	Rng,
};
use std::collections::HashMap;

#[byond_fn]
pub fn pick(options: ByondValue, secure: Option<bool>) -> ByondResult<Option<ByondValue>> {
	if !options.is_list() {
		return Ok(None);
	}
	let length = options.length()?;
	match length {
		0 => return Ok(None),
		1 => return options.read_list_index(&1).map(Some),
		_ => {}
	}
	let idx = global(secure).gen_range::<usize, _>(1..=length);
	options.read_list_index(&idx).map(Some)
}

#[byond_fn]
pub fn pick_weighted(
	options: HashMap<ByondValue, f32, ahash::RandomState>,
	secure: Option<bool>,
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
	let dist = WeightedIndex::new(weights).ok()?;
	values.into_iter().nth(dist.sample(&mut global(secure)))
}
