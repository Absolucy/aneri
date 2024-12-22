// SPDX-License-Identifier: MPL-2.0

pub mod instant;
pub mod timestamp;

pub fn cleanup() {
	instant::free_instances();
}
