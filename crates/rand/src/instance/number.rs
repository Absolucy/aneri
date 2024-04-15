// SPDX-License-Identifier: MPL-2.0
use super::INSTANCES;
use crate::shared;
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
pub fn instanced_random_range_int_unsigned(src: ByondSlotKey, min: u32, max: u32) -> Option<u32> {
	INSTANCES
		.lock()
		.get_mut(src)
		.map(|rng| shared::random_range_int_unsigned(rng, min, max))
}

#[byond_fn]
pub fn instanced_random_range_int_signed(src: ByondSlotKey, min: i32, max: i32) -> Option<i32> {
	INSTANCES
		.lock()
		.get_mut(src)
		.map(|rng| shared::random_range_int_signed(rng, min, max))
}
