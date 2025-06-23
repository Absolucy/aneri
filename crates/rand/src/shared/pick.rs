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
		return Ok(ByondValue::NULL);
	}
	let length = options.length()?;
	match length {
		0 => return Ok(ByondValue::NULL),
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
		return Ok(ByondValue::NULL);
	}
	let length = options.length()?;
	match length {
		0 => return Ok(ByondValue::NULL),
		1 => return options.read_list_index(&1),
		_ => {}
	}
	let (options, weights): (Vec<ByondValue>, Vec<f32>) = options
		.read_assoc_list()?
		.into_iter()
		.filter_map(|[value, weight]| {
			weight
				.get_number()
				.ok()
				.filter(|&weight| weight.is_normal() && weight.is_sign_positive())
				.map(|weight| (value, weight))
		})
		.unzip();
	match weights.len() {
		0 => return Ok(ByondValue::NULL),
		1 => return Ok(options.into_iter().next().unwrap_or_default()),
		_ => {}
	}
	let dist = match WeightedIndex::new(weights) {
		Ok(dist) => dist,
		Err(_) => return Ok(ByondValue::NULL),
	};
	let idx = dist.sample(rng);
	Ok(options.into_iter().nth(idx).unwrap_or_default())
}
