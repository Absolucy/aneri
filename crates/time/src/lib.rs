// SPDX-License-Identifier: MPL-2.0
#[macro_use]
extern crate meowtonin;

pub mod instant;
pub mod timestamp;

pub fn cleanup() {
	instant::free_instances();
}
