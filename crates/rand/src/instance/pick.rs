// SPDX-License-Identifier: MPL-2.0
use super::INSTANCES;
use crate::shared;
use aneri_core::ByondSlotKey;
use meowtonin::{ByondResult, ByondValue, byond_fn};

/// `aneri_rand_pick` on a `/datum/rng`.
/// Null if the generator was deleted.
#[byond_fn(name = "aneri_raw_instanced_pick")]
pub fn instanced_pick(src: ByondSlotKey, options: ByondValue) -> ByondResult<Option<ByondValue>> {
	INSTANCES
		.with_mut(src, |rng| shared::pick(rng, options))
		.transpose()
}

/// `aneri_rand_pick_weighted` on a `/datum/rng`.
/// Null if the generator was deleted.
#[byond_fn(name = "aneri_raw_instanced_pick_weighted")]
pub fn instanced_pick_weighted(
	src: ByondSlotKey,
	options: ByondValue,
) -> ByondResult<Option<ByondValue>> {
	INSTANCES
		.with_mut(src, |rng| shared::pick_weighted(rng, options))
		.transpose()
}
