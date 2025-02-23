// SPDX-License-Identifier: MPL-2.0
use aneri_core::ByondSlotKey;
use meowtonin::{byond_fn, ByondError, ByondResult, ByondValue};
use parking_lot::Mutex;
use slotmap::SlotMap;
use std::{sync::LazyLock, time::Instant};

const DEFAULT_CAPACITY: usize = 16;

static INSTANCES: LazyLock<Mutex<SlotMap<ByondSlotKey, Instant>>> =
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
pub fn instant_new(mut src: ByondValue) -> ByondResult<()> {
	let mut instances = INSTANCES.lock();
	let instant = Instant::now();
	instances
		.insert(instant)
		.save(&mut src)
		.map_err(ByondError::boxed)
}

#[byond_fn]
pub fn instant_del(src: ByondSlotKey) -> bool {
	INSTANCES.lock().remove(src).is_some()
}

#[byond_fn]
pub fn instant_reset(src: ByondSlotKey) -> bool {
	match INSTANCES.lock().get_mut(src) {
		Some(instant) => {
			*instant = Instant::now();
			true
		}
		None => false,
	}
}

#[byond_fn]
pub fn instant_microseconds(src: ByondSlotKey) -> Option<u32> {
	let instances = INSTANCES.lock();
	let instant = instances.get(src)?;
	Some(instant.elapsed().as_micros() as u32)
}

#[byond_fn]
pub fn instant_nanoseconds(src: ByondSlotKey) -> Option<u32> {
	let instances = INSTANCES.lock();
	let instant = instances.get(src)?;
	Some(instant.elapsed().as_nanos() as u32)
}

#[byond_fn]
pub fn instant_milliseconds(src: ByondSlotKey) -> Option<u32> {
	let instances = INSTANCES.lock();
	let instant = instances.get(src)?;
	Some(instant.elapsed().as_millis() as u32)
}

#[byond_fn]
pub fn instant_seconds(src: ByondSlotKey) -> Option<f32> {
	let instances = INSTANCES.lock();
	let instant = instances.get(src)?;
	Some(instant.elapsed().as_secs_f32())
}
