// SPDX-License-Identifier: MPL-2.0
pub mod dispatcher;
pub mod number;
pub mod pick;
pub mod prob;
pub mod string;

use aneri_core::{ByondSlotKey, ByondSlotMap};
use dispatcher::RngDispatcher;
use meowtonin::{ByondResult, ByondValue, byond_fn};

static INSTANCES: ByondSlotMap<RngDispatcher> = ByondSlotMap::new();

pub(crate) fn free_instances() {
	INSTANCES.clear();
}

/// Creates the generator behind a `/datum/rng` and stores its key on `src`.
#[byond_fn(name = "aneri_raw_rng_new")]
pub fn rng_new(
	mut src: ByondValue,
	/// Use the slower, cryptographically secure generator, default false.
	secure: Option<bool>,
	/// The same seed always gives the same numbers.
	/// Null picks a random seed.
	seed: Option<u32>,
) -> ByondResult<()> {
	let secure = secure.unwrap_or(false);
	let rng = if secure {
		RngDispatcher::secure(seed)
	} else {
		RngDispatcher::fast(seed)
	};
	INSTANCES.insert(rng).save(&mut src)
}

/// Deletes the generator behind a `/datum/rng`.
/// Returns false if it was already gone.
#[byond_fn(name = "aneri_raw_rng_del")]
pub fn rng_del(src: ByondSlotKey) -> bool {
	INSTANCES.remove(src).is_some()
}
