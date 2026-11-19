// SPDX-License-Identifier: MPL-2.0
use super::global;
use crate::shared;
use meowtonin::byond_fn;
use rand::RngExt;

/// A random whole number from 0 to 255.
#[byond_fn(name = "aneri_rand_byte")]
pub fn random_byte(
	/// Use the slower, cryptographically secure generator, default false.
	secure: Option<bool>,
) -> u8 {
	global(secure).random()
}

/// A random number from 0 up to, but not including, 1.
#[byond_fn(name = "aneri_rand_float")]
pub fn random_float(
	/// Use the slower, cryptographically secure generator, default false.
	secure: Option<bool>,
) -> f32 {
	global(secure).random()
}

/// A random whole number from 0 to 4294967295.
/// DM numbers lose precision above 16777216, so big results get rounded.
#[byond_fn(name = "aneri_rand_uint")]
pub fn random_int_unsigned(
	/// Use the slower, cryptographically secure generator, default false.
	secure: Option<bool>,
) -> u32 {
	global(secure).random()
}

/// A random whole number from -2147483648 to 2147483647.
/// DM numbers lose precision past 16777216 either way, so big results get rounded.
#[byond_fn(name = "aneri_rand_int")]
pub fn random_int_signed(
	/// Use the slower, cryptographically secure generator, default false.
	secure: Option<bool>,
) -> i32 {
	global(secure).random()
}

/// A random whole number from `min` to `max`, both included.
/// Swaps them if `min` is bigger.
/// Runtimes unless both are whole numbers, 0 or more.
#[byond_fn(name = "aneri_raw_random_range_int_unsigned")]
pub fn random_range_int_unsigned(
	min: u32,
	max: u32,
	/// Use the slower, cryptographically secure generator, default false.
	secure: Option<bool>,
) -> u32 {
	shared::random_range_int_unsigned(&mut global(secure), min, max)
}

/// A random whole number from `min` to `max`, both included.
/// Swaps them if `min` is bigger.
/// Runtimes unless both are whole numbers.
#[byond_fn(name = "aneri_raw_random_range_int_signed")]
pub fn random_range_int_signed(
	min: i32,
	max: i32,
	/// Use the slower, cryptographically secure generator, default false.
	secure: Option<bool>,
) -> i32 {
	shared::random_range_int_signed(&mut global(secure), min, max)
}
