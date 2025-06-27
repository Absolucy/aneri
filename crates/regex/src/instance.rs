// SPDX-License-Identifier: MPL-2.0
use crate::shared;
use aneri_core::ByondSlotKey;
use meowtonin::{ByondError, ByondResult, ByondValue, byond_fn};
use parking_lot::RwLock;
use regex::Regex;
use slotmap::SlotMap;
use std::sync::LazyLock;

const DEFAULT_CAPACITY: usize = 16;

static INSTANCES: LazyLock<RwLock<SlotMap<ByondSlotKey, Regex>>> =
	LazyLock::new(|| RwLock::new(SlotMap::with_capacity_and_key(DEFAULT_CAPACITY)));

pub(crate) fn free_instances() {
	let mut instances = INSTANCES.write();
	if instances.capacity() > DEFAULT_CAPACITY {
		// Don't use clear(), so we reclaim memory.
		*instances = SlotMap::with_capacity_and_key(DEFAULT_CAPACITY);
	} else {
		// If we're at the default capacity, it's a waste of time to reallocate.
		instances.clear();
	}
}

#[byond_fn]
pub fn regex_new(mut src: ByondValue, regex: String) -> ByondResult<()> {
	let mut instances = INSTANCES.write();
	let regex = Regex::new(&regex).map_err(ByondError::boxed)?;
	instances
		.insert(regex)
		.save(&mut src)
		.map_err(ByondError::boxed)
}

#[byond_fn]
pub fn regex_del(src: ByondSlotKey) -> bool {
	INSTANCES.write().remove(src).is_some()
}

#[byond_fn]
pub fn instanced_regex_is_match(src: ByondSlotKey, haystack: String) -> Option<bool> {
	INSTANCES
		.read()
		.get(src)
		.map(|regex| regex.is_match(&haystack))
}

#[byond_fn]
pub fn instanced_regex_find(src: ByondSlotKey, haystack: String) -> ByondResult<ByondValue> {
	let instances = INSTANCES.read();
	let regex = match instances.get(src) {
		Some(regex) => regex,
		None => return Ok(ByondValue::NULL),
	};
	shared::regex_find(regex, &haystack)
}

#[byond_fn]
pub fn instanced_regex_split(src: ByondSlotKey, haystack: String) -> ByondResult<ByondValue> {
	let instances = INSTANCES.read();
	let regex = match instances.get(src) {
		Some(regex) => regex,
		None => return Ok(ByondValue::NULL),
	};
	shared::regex_split(regex, &haystack)
}

#[byond_fn]
pub fn instanced_regex_splitn(
	src: ByondSlotKey,
	haystack: String,
	limit: usize,
) -> ByondResult<ByondValue> {
	let instances = INSTANCES.read();
	let regex = match instances.get(src) {
		Some(regex) => regex,
		None => return Ok(ByondValue::NULL),
	};
	shared::regex_splitn(regex, &haystack, limit)
}

#[byond_fn]
pub fn instanced_regex_replace(
	src: ByondSlotKey,
	haystack: String,
	with: String,
) -> ByondResult<ByondValue> {
	let instances = INSTANCES.read();
	let regex = match instances.get(src) {
		Some(regex) => regex,
		None => return Ok(ByondValue::NULL),
	};
	shared::regex_replace(regex, &haystack, &with)
}

#[byond_fn]
pub fn instanced_regex_replace_all(
	src: ByondSlotKey,
	haystack: String,
	with: String,
) -> ByondResult<ByondValue> {
	let instances = INSTANCES.read();
	let regex = match instances.get(src) {
		Some(regex) => regex,
		None => return Ok(ByondValue::NULL),
	};
	shared::regex_replace_all(regex, &haystack, &with)
}
