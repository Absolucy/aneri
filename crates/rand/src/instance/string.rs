// SPDX-License-Identifier: MPL-2.0
use super::INSTANCES;
use aneri_core::ByondSlotKey;
use rand::{distributions::Alphanumeric, Rng};

#[byond_fn]
pub fn instanced_random_string_alphanumeric(src: ByondSlotKey, length: usize) -> Option<String> {
	INSTANCES.lock().get_mut(src).map(|rng| {
		(0..=length)
			.map(|_| rng.sample(Alphanumeric) as char)
			.collect()
	})
}
