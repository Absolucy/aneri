// SPDX-License-Identifier: MPL-2.0
use super::INSTANCES;
use crate::shared;
use aneri_core::ByondSlotKey;
use meowtonin::ByondResult;

#[byond_fn]
pub fn instanced_pick_weighted(src: ByondSlotKey, options: Vec<f32>) -> ByondResult<Option<usize>> {
	INSTANCES
		.lock()
		.get_mut(src)
		.map(|rng| shared::pick_weighted(rng, options))
		.transpose()
		.map(|result| result.flatten())
}
