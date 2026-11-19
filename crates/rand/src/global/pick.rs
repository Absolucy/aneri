// SPDX-License-Identifier: MPL-2.0
use super::global;
use crate::shared;
use meowtonin::{ByondResult, ByondValue, byond_fn};

/// A random item from a list.
/// Returns null for an empty list or a non-list.
#[byond_fn(name = "aneri_raw_pick")]
pub fn pick(
	options: ByondValue,
	/// Use the slower, cryptographically secure generator, default false.
	secure: Option<bool>,
) -> ByondResult<ByondValue> {
	shared::pick(&mut global(secure), options)
}

/// A random key from an assoc list of item to weight.
/// An item with weight 2 is picked twice as often as one with weight 1.
/// Items whose weight isn't a positive number are skipped.
/// Returns null if nothing is left to pick from.
#[byond_fn(name = "aneri_raw_pick_weighted")]
pub fn pick_weighted(
	options: ByondValue,
	/// Use the slower, cryptographically secure generator, default false.
	secure: Option<bool>,
) -> ByondResult<ByondValue> {
	shared::pick_weighted(&mut global(secure), options)
}
