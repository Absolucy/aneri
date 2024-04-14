// SPDX-License-Identifier: MPL-2.0
use super::global;
use meowtonin::{ByondResult, ByondValue};
use rand::{
	distributions::{Distribution, WeightedIndex},
	Rng,
};

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
pub fn pick_weighted(options: ByondValue, secure: Option<bool>) -> ByondResult<Option<ByondValue>> {
	if !options.is_list() {
		return Ok(None);
	}
	let length = options.length::<usize>()?;
	match length {
		0 => return Ok(None),
		1 => return options.read_list_index(&1).map(Some),
		_ => {}
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
	let idx = dist.sample(&mut global(secure));
	Ok(options.get(idx).and_then(|entry| entry.get(2)).cloned())
}
