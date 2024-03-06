// SPDX-License-Identifier: MPL-2.0
pub mod dispatcher;
pub mod number;
pub mod pick;
pub mod prob;
pub mod string;

use aneri_core::ByondSlotKey;
use dispatcher::RngDispatcher;
use meowtonin::{ByondResult, ByondValue};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use slotmap::SlotMap;

static INSTANCES: Lazy<Mutex<SlotMap<ByondSlotKey, RngDispatcher>>> =
	Lazy::new(|| Mutex::new(SlotMap::with_capacity_and_key(128)));

pub(crate) fn free_instances() {
	if let Some(instances) = Lazy::get(&INSTANCES) {
		instances.lock().clear();
	}
}

#[byond_fn]
pub fn rng_new(mut src: ByondValue, secure: Option<bool>, seed: Option<u32>) -> ByondResult<()> {
	let secure = secure.unwrap_or(false);
	let rng = if secure {
		RngDispatcher::blake3(seed)
	} else {
		RngDispatcher::wyrand(seed)
	};
	INSTANCES.lock().insert(rng).save(&mut src)
}

#[byond_fn]
pub fn rng_del(src: ByondSlotKey) -> bool {
	INSTANCES.lock().remove(src).is_some()
}
