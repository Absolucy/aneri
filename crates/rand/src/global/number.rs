// SPDX-License-Identifier: MPL-2.0
use super::global;
use crate::shared;
use rand::Rng;

#[byond_fn]
pub fn random_byte(secure: Option<bool>) -> u8 {
	global(secure).gen()
}

#[byond_fn]
pub fn random_float(secure: Option<bool>) -> f32 {
	global(secure).gen()
}

#[byond_fn]
pub fn random_int_unsigned(secure: Option<bool>) -> u32 {
	global(secure).gen()
}

#[byond_fn]
pub fn random_int_signed(secure: Option<bool>) -> i32 {
	global(secure).gen()
}

#[byond_fn]
pub fn random_range_int_unsigned(min: u32, max: u32, secure: Option<bool>) -> u32 {
	shared::random_range_int_unsigned(&mut global(secure), min, max)
}

#[byond_fn]
pub fn random_range_int_signed(min: i32, max: i32, secure: Option<bool>) -> i32 {
	shared::random_range_int_signed(&mut global(secure), min, max)
}
