// SPDX-License-Identifier: MPL-2.0
use rand::Rng;

pub(crate) fn random_range_int_unsigned<Gen>(rng: &mut Gen, mut min: u32, mut max: u32) -> u32
where
	Gen: Rng,
{
	if min > max {
		std::mem::swap(&mut min, &mut max);
	}
	rng.gen_range(min..=max)
}

pub(crate) fn random_range_int_signed<Gen>(rng: &mut Gen, mut min: i32, mut max: i32) -> i32
where
	Gen: Rng,
{
	if min > max {
		std::mem::swap(&mut min, &mut max);
	}
	rng.gen_range(min..=max)
}
