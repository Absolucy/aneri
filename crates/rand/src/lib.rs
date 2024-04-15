// SPDX-License-Identifier: MPL-2.0
#![warn(
	clippy::correctness,
	clippy::suspicious,
	clippy::complexity,
	clippy::perf,
	clippy::style
)]
#[macro_use]
extern crate meowtonin;

pub mod global;
pub mod instance;
pub mod shared;

pub fn cleanup() {
	global::reseed_global_rng();
	instance::free_instances();
}
