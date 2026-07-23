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

#[byond_fn]
pub fn rng_new(mut src: ByondValue, secure: Option<bool>, seed: Option<u32>) -> ByondResult<()> {
	let secure = secure.unwrap_or(false);
	let rng = if secure {
		RngDispatcher::secure(seed)
	} else {
		RngDispatcher::fast(seed)
	};
	INSTANCES.insert(rng).save(&mut src)
}

#[byond_fn]
pub fn rng_del(src: ByondSlotKey) -> bool {
	INSTANCES.remove(src).is_some()
}
