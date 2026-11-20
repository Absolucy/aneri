// SPDX-License-Identifier: MPL-2.0
use aneri_core::{ByondSlotKey, ByondSlotMap};
use meowtonin::{ByondError, ByondResult, ByondValue, byond_fn};
use std::time::Instant;

static INSTANCES: ByondSlotMap<Instant> = ByondSlotMap::new();

pub(crate) fn free_instances() {
	INSTANCES.clear();
}

/// Starts the stopwatch behind a `/datum/instant` and stores its key on `src`.
#[byond_fn(name = "aneri_raw_instant_new")]
pub fn instant_new(mut src: ByondValue) -> ByondResult<()> {
	let instant = Instant::now();
	INSTANCES
		.insert(instant)
		.save(&mut src)
		.map_err(ByondError::boxed)
}

/// Deletes the stopwatch behind a `/datum/instant`.
/// Returns false if it was already gone.
#[byond_fn(name = "aneri_raw_instant_del")]
pub fn instant_del(src: ByondSlotKey) -> bool {
	INSTANCES.remove(src).is_some()
}

/// Restarts the stopwatch from 0.
/// Returns false if it was deleted.
#[byond_fn(name = "aneri_raw_instant_reset")]
pub fn instant_reset(src: ByondSlotKey) -> bool {
	INSTANCES
		.with_mut(src, |instant| {
			*instant = Instant::now();
			true
		})
		.unwrap_or(false)
}

// these return floats because DM numbers are f32: meowtonin refuses any integer past 2^24,
// which microseconds pass after 17 seconds and nanoseconds after 17 milliseconds

/// Microseconds since the stopwatch started or was reset.
/// Null if it was deleted.
#[byond_fn(name = "aneri_raw_instant_microseconds")]
pub fn instant_microseconds(src: ByondSlotKey) -> Option<f64> {
	INSTANCES.with(src, |instant| instant.elapsed().as_secs_f64() * 1_000_000.0)
}

/// Nanoseconds since the stopwatch started or was reset.
/// Null if it was deleted.
#[byond_fn(name = "aneri_raw_instant_nanoseconds")]
pub fn instant_nanoseconds(src: ByondSlotKey) -> Option<f64> {
	INSTANCES.with(src, |instant| {
		instant.elapsed().as_secs_f64() * 1_000_000_000.0
	})
}

/// Milliseconds since the stopwatch started or was reset.
/// Null if it was deleted.
#[byond_fn(name = "aneri_raw_instant_milliseconds")]
pub fn instant_milliseconds(src: ByondSlotKey) -> Option<f64> {
	INSTANCES.with(src, |instant| instant.elapsed().as_secs_f64() * 1_000.0)
}

/// Seconds since the stopwatch started or was reset, with a fraction.
/// Null if it was deleted.
#[byond_fn(name = "aneri_raw_instant_seconds")]
pub fn instant_seconds(src: ByondSlotKey) -> Option<f32> {
	INSTANCES.with(src, |instant| instant.elapsed().as_secs_f32())
}
