// SPDX-License-Identifier: MPL-2.0
use rand::Rng;

pub(crate) fn prob<Gen>(rng: &mut Gen, probability: f64) -> bool
where
	Gen: Rng,
{
	if !probability.is_finite() {
		return true;
	} else if probability <= 0.0 {
		return false;
	}
	let probability = (probability / 100.0).clamp(0.0, 1.0);
	rng.gen_bool(probability)
}

pub(crate) fn prob_ratio<Gen>(rng: &mut Gen, numerator: u32, denominator: u32) -> bool
where
	Gen: Rng,
{
	rng.gen_ratio(numerator, denominator)
}
