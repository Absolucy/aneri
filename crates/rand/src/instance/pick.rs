// SPDX-License-Identifier: MPL-2.0
use super::INSTANCES;
use crate::shared;
use aneri_core::ByondSlotKey;
use meowtonin::{ByondResult, ByondValue, byond_fn};

#[byond_fn]
pub fn instanced_pick(src: ByondSlotKey, options: ByondValue) -> ByondResult<Option<ByondValue>> {
	INSTANCES
		.lock()
		.get_mut(src)
		.map(|rng| shared::pick(rng, options))
		.transpose()
}

#[byond_fn]
pub fn instanced_pick_weighted(
	src: ByondSlotKey,
	options: ByondValue,
	weights: ByondValue,
) -> ByondResult<Option<ByondValue>> {
	INSTANCES
		.lock()
		.get_mut(src)
		.map(|rng| shared::pick_weighted(rng, options, weights))
		.transpose()
}
