// SPDX-License-Identifier: MPL-2.0
use meowtonin::{ByondResult, ByondValue, FromByond};
use rand::{
	Rng,
	distr::{Distribution, weighted::WeightedIndex},
};

pub(crate) fn pick<Gen>(rng: &mut Gen, options: ByondValue) -> ByondResult<ByondValue>
where
	Gen: Rng,
{
	if !options.is_list() {
		return Ok(ByondValue::NULL);
	}
	let length = options.length()?;
	match length {
		0 => return Ok(ByondValue::NULL),
		1 => return options.read_list_index(&1),
		_ => {}
	}
	let idx = rng.random_range::<usize, _>(1..=length);
	options.read_list_index(&idx)
}

pub(crate) fn pick_weighted<Gen>(
	rng: &mut Gen,
	options: ByondValue,
	weights: ByondValue,
) -> ByondResult<ByondValue>
where
	Gen: Rng,
{
	if !options.is_list() {
		return Ok(ByondValue::NULL);
	}
	let length = options.length()?;
	match length {
		0 => return Ok(ByondValue::NULL),
		1 => return options.read_list_index(&1),
		_ => {}
	}
	let weights = if weights.is_list() {
		let mut weights = Vec::<f32>::from_byond(weights)?;
		if weights.len() != length {
			weights.resize(length, 1.0);
		}
		weights
	} else {
		vec![1.0; length]
	};
	let dist = match WeightedIndex::new(weights) {
		Ok(dist) => dist,
		Err(_) => return Ok(ByondValue::NULL),
	};
	let idx = dist.sample(rng);
	options.read_list_index(&idx)
}
