// SPDX-License-Identifier: MPL-2.0
pub mod dispatcher;
pub mod number;
pub mod pick;
pub mod prob;
pub mod string;

use aneri_core::ByondSlotKey;
use dispatcher::RngDispatcher;
use meowtonin::{ByondResult, ByondValue};
use parking_lot::Mutex;
use slotmap::SlotMap;
use std::sync::OnceLock;

static INSTANCES: OnceLock<Mutex<SlotMap<ByondSlotKey, RngDispatcher>>> = OnceLock::new();

#[inline]
pub(crate) fn instances() -> &'static Mutex<SlotMap<ByondSlotKey, RngDispatcher>> {
	INSTANCES.get_or_init(Mutex::default)
}

pub(crate) fn free_instances() {
	if let Some(instances) = INSTANCES.get() {
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
	instances().lock().insert(rng).save(&mut src)
}

#[byond_fn]
pub fn rng_del(src: ByondSlotKey) -> bool {
	instances().lock().remove(src).is_some()
}
