// SPDX-License-Identifier: MPL-2.0
use aneri_core::ByondSlotKey;
use meowtonin::{ByondError, ByondResult, ByondValue};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use slotmap::SlotMap;
use std::time::Instant;

static INSTANCES: Lazy<RwLock<SlotMap<ByondSlotKey, Instant>>> =
	Lazy::new(|| RwLock::new(SlotMap::with_capacity_and_key(128)));

pub(crate) fn free_instances() {
	if let Some(instances) = Lazy::get(&INSTANCES) {
		instances.write().clear();
	}
}

#[byond_fn]
pub fn instant_new(mut src: ByondValue) -> ByondResult<()> {
	let mut instances = INSTANCES.write();
	let instant = Instant::now();
	instances
		.insert(instant)
		.save(&mut src)
		.map_err(ByondError::boxed)
}

#[byond_fn]
pub fn instant_del(src: ByondSlotKey) -> bool {
	INSTANCES.write().remove(src).is_some()
}

#[byond_fn]
pub fn instant_microseconds(src: ByondSlotKey) -> Option<u32> {
	let instances = INSTANCES.read();
	let instant = match instances.get(src) {
		Some(instant) => instant,
		None => return None,
	};
	Some(instant.elapsed().as_micros() as u32)
}

#[byond_fn]
pub fn instant_milliseconds(src: ByondSlotKey) -> Option<u32> {
	let instances = INSTANCES.read();
	let instant = match instances.get(src) {
		Some(instant) => instant,
		None => return None,
	};
	Some(instant.elapsed().as_millis() as u32)
}

#[byond_fn]
pub fn instant_seconds(src: ByondSlotKey) -> Option<f32> {
	let instances = INSTANCES.read();
	let instant = match instances.get(src) {
		Some(instant) => instant,
		None => return None,
	};
	Some(instant.elapsed().as_secs_f32())
}
