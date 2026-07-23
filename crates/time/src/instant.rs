// SPDX-License-Identifier: MPL-2.0
use aneri_core::{ByondSlotKey, ByondSlotMap};
use meowtonin::{ByondError, ByondResult, ByondValue, byond_fn};
use std::time::Instant;

static INSTANCES: ByondSlotMap<Instant> = ByondSlotMap::new();

pub(crate) fn free_instances() {
	INSTANCES.clear();
}

#[byond_fn]
pub fn instant_new(mut src: ByondValue) -> ByondResult<()> {
	let instant = Instant::now();
	INSTANCES
		.insert(instant)
		.save(&mut src)
		.map_err(ByondError::boxed)
}

#[byond_fn]
pub fn instant_del(src: ByondSlotKey) -> bool {
	INSTANCES.remove(src).is_some()
}

#[byond_fn]
pub fn instant_reset(src: ByondSlotKey) -> bool {
	INSTANCES
		.with_mut(src, |instant| {
			*instant = Instant::now();
			true
		})
		.unwrap_or(false)
}

#[byond_fn]
pub fn instant_microseconds(src: ByondSlotKey) -> Option<u32> {
	INSTANCES.with(src, |instant| instant.elapsed().as_micros() as u32)
}

#[byond_fn]
pub fn instant_nanoseconds(src: ByondSlotKey) -> Option<u32> {
	INSTANCES.with(src, |instant| instant.elapsed().as_nanos() as u32)
}

#[byond_fn]
pub fn instant_milliseconds(src: ByondSlotKey) -> Option<u32> {
	INSTANCES.with(src, |instant| instant.elapsed().as_millis() as u32)
}

#[byond_fn]
pub fn instant_seconds(src: ByondSlotKey) -> Option<f32> {
	INSTANCES.with(src, |instant| instant.elapsed().as_secs_f32())
}
