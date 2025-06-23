// SPDX-License-Identifier: MPL-2.0
pub mod dispatcher;
pub mod number;
pub mod pick;
pub mod prob;
pub mod string;

use aneri_core::ByondSlotKey;
use dispatcher::RngDispatcher;
use meowtonin::{ByondResult, ByondValue, byond_fn};
use parking_lot::Mutex;
use slotmap::SlotMap;
use std::sync::LazyLock;

const DEFAULT_CAPACITY: usize = 16;

static INSTANCES: LazyLock<Mutex<SlotMap<ByondSlotKey, RngDispatcher>>> =
	LazyLock::new(|| Mutex::new(SlotMap::with_capacity_and_key(DEFAULT_CAPACITY)));

pub(crate) fn free_instances() {
	let mut instances = INSTANCES.lock();
	if instances.capacity() > DEFAULT_CAPACITY {
		// Don't use clear(), so we reclaim memory.
		*instances = SlotMap::with_capacity_and_key(DEFAULT_CAPACITY);
	} else {
		// If we're at the default capacity, it's a waste of time to reallocate.
		instances.clear();
	}
}

#[byond_fn]
pub fn rng_new(mut src: ByondValue, secure: Option<bool>, seed: Option<u32>) -> ByondResult<()> {
	let secure = secure.unwrap_or(false);
	let rng = if secure {
		RngDispatcher::secure(seed)
	} else {
		RngDispatcher::fast(seed)
	};
	INSTANCES.lock().insert(rng).save(&mut src)
}

#[byond_fn]
pub fn rng_del(src: ByondSlotKey) -> bool {
	INSTANCES.lock().remove(src).is_some()
}
