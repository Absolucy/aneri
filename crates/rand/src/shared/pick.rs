// SPDX-License-Identifier: MPL-2.0
use meowtonin::{ByondResult, ByondValue};
use rand::{
	distributions::{Distribution, WeightedIndex},
	Rng,
};

pub(crate) fn pick<Gen>(rng: &mut Gen, options: ByondValue) -> ByondResult<ByondValue>
where
	Gen: Rng,
{
	if !options.is_list() {
		return Ok(ByondValue::null());
	}
	let length = options.length::<usize>()?;
	match length {
		0 => return Ok(ByondValue::null()),
		1 => return options.read_list_index(&1),
		_ => {}
	}
	let idx = rng.gen_range::<usize, _>(1..=length);
	options.read_list_index(&idx)
}

pub(crate) fn pick_weighted<Gen>(rng: &mut Gen, options: ByondValue) -> ByondResult<ByondValue>
where
	Gen: Rng,
{
	if !options.is_list() {
		return Ok(ByondValue::null());
	}
	let length = options.length::<usize>()?;
	match length {
		0 => return Ok(ByondValue::null()),
		1 => return options.read_list_index(&1),
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
		0 => return Ok(ByondValue::null()),
		1 => {
			return Ok(options
				.get(1)
				.and_then(|entry| entry.get(1))
				.cloned()
				.unwrap_or_default())
		}
		_ => {}
	}
	let dist = match WeightedIndex::new(weights) {
		Ok(dist) => dist,
		Err(_) => return Ok(ByondValue::null()),
	};
	let idx = dist.sample(rng);
	Ok(options
		.get(idx)
		.and_then(|entry| entry.get(2))
		.cloned()
		.unwrap_or_default())
}
