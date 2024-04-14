// SPDX-License-Identifier: MPL-2.0
use super::INSTANCES;
use aneri_core::ByondSlotKey;
use meowtonin::{ByondResult, ByondValue};
use rand::{
	distributions::{Distribution, WeightedIndex},
	Rng,
};

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
	let mut instances = INSTANCES.lock();
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
	options: ByondValue,
) -> ByondResult<Option<ByondValue>> {
	if !options.is_list() || options.length::<usize>()? < 1 {
		return Ok(None);
	}
	let options = options.read_assoc_list()?;
	let weights = options
		.iter()
		.map(|[_, weight]| weight.get_number())
		.filter_map(|weight| weight.ok())
		.filter(|weight| weight.is_normal() && weight.is_sign_positive())
		.collect::<Vec<f32>>();
	match weights.len() {
		0 => return Ok(None),
		1 => return Ok(options.get(1).and_then(|entry| entry.get(1)).cloned()),
		_ => {}
	}
	let dist = match WeightedIndex::new(weights) {
		Ok(dist) => dist,
		Err(_) => return Ok(None),
	};
	let mut instances = INSTANCES.lock();
	let rng = match instances.get_mut(src) {
		Some(rng) => rng,
		None => return Ok(None),
	};
	let idx = dist.sample(rng);
	Ok(options.get(idx).and_then(|entry| entry.get(2)).cloned())
}
