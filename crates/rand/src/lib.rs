// SPDX-License-Identifier: MPL-2.0

pub mod global;
pub mod instance;
pub mod shared;

pub fn cleanup() {
	global::reseed_global_rng();
	instance::free_instances();
}
