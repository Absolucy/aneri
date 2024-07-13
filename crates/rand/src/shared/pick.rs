// SPDX-License-Identifier: MPL-2.0
use meowtonin::ByondResult;
use rand::{
	distributions::{Distribution, WeightedIndex},
	Rng,
};

pub(crate) fn pick_weighted<Gen>(rng: &mut Gen, options: Vec<f32>) -> ByondResult<Option<usize>>
where
	Gen: Rng,
{
	let length = options.len();
	match length {
		0 => return Ok(None),
		1 => return Ok(Some(1)),
		_ => {}
	}
	let (indexes, weights) = options
		.into_iter()
		.enumerate()
		.filter(|(_, weight)| weight.is_normal() && weight.is_sign_positive())
		.collect::<(Vec<usize>, Vec<f32>)>();
	match weights.len() {
		0 => return Ok(None),
		1 => {
			return Ok(indexes.into_iter().next());
		}
		_ => {}
	}
	let dist = match WeightedIndex::new(weights) {
		Ok(dist) => dist,
		Err(_) => return Ok(None),
	};
	let idx = dist.sample(rng);
	Ok(indexes.into_iter().nth(idx).map(|idx| idx + 1))
}
