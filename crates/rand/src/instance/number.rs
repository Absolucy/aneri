// SPDX-License-Identifier: MPL-2.0
use super::INSTANCES;
use aneri_core::ByondSlotKey;
use rand::{
	distributions::{
		uniform::{SampleRange, SampleUniform},
		Distribution, Standard,
	},
	Rng,
};

fn rand_impl<Output>(src: ByondSlotKey) -> Option<Output>
where
	Standard: Distribution<Output>,
{
	INSTANCES.lock().get_mut(src).map(|rng| rng.gen())
}

fn range_impl<Output, Range>(src: ByondSlotKey, range: Range) -> Option<Output>
where
	Output: SampleUniform,
	Range: SampleRange<Output>,
{
	INSTANCES
		.lock()
		.get_mut(src)
		.map(|rng| rng.gen_range(range))
}

#[byond_fn]
pub fn instanced_random_byte(src: ByondSlotKey) -> Option<u8> {
	rand_impl(src)
}

#[byond_fn]
pub fn instanced_random_float(src: ByondSlotKey) -> Option<f32> {
	rand_impl(src)
}

#[byond_fn]
pub fn instanced_random_int_unsigned(src: ByondSlotKey) -> Option<u32> {
	rand_impl(src)
}

#[byond_fn]
pub fn instanced_random_int_signed(src: ByondSlotKey) -> Option<i32> {
	rand_impl(src)
}

#[byond_fn]
pub fn instanced_random_range_int_unsigned(
	src: ByondSlotKey,
	mut min: u32,
	mut max: u32,
) -> Option<u32> {
	if min > max {
		std::mem::swap(&mut min, &mut max);
	}
	range_impl(src, min..=max)
}

#[byond_fn]
pub fn instanced_random_range_int_signed(
	src: ByondSlotKey,
	mut min: i32,
	mut max: i32,
) -> Option<i32> {
	if min > max {
		std::mem::swap(&mut min, &mut max);
	}
	range_impl(src, min..=max)
}
