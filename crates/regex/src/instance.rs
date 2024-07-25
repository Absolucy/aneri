// SPDX-License-Identifier: MPL-2.0
use aneri_core::ByondSlotKey;
use meowtonin::{ByondError, ByondResult, ByondValue, ToByond};
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
		None => return Ok(ByondValue::null()),
	};
	let mut list = ByondValue::new_list()?;
	let substring = byondval!("text");
	let start = byondval!("start");
	let end = byondval!("end");
	if let Some(matched) = regex.find(&haystack) {
		list.write_list_index(&substring, matched.as_str())?;
		list.write_list_index(&start, matched.start() + 1)?;
		list.write_list_index(&end, matched.end() + 1)?;
	}
	Ok(list)
}

#[byond_fn]
pub fn instanced_regex_find_all(src: ByondSlotKey, haystack: String) -> ByondResult<ByondValue> {
	let instances = INSTANCES.read();
	let regex = match instances.get(src) {
		Some(regex) => regex,
		None => return Ok(ByondValue::null()),
	};
	let mut list = ByondValue::new_list()?;
	let substring = byondval!("text");
	let start = byondval!("start");
	let end = byondval!("end");
	for matched in regex.find_iter(&haystack) {
		let mut match_list = ByondValue::new_list()?;
		match_list.write_list_index(&substring, matched.as_str())?;
		match_list.write_list_index(&start, matched.start() + 1)?;
		match_list.write_list_index(&end, matched.end() + 1)?;
		list.push_list(match_list)?;
	}
	Ok(list)
}

#[byond_fn]
pub fn instanced_regex_split(src: ByondSlotKey, haystack: String) -> ByondResult<ByondValue> {
	let instances = INSTANCES.read();
	let regex = match instances.get(src) {
		Some(regex) => regex,
		None => return Ok(ByondValue::null()),
	};
	let mut list = ByondValue::new_list()?;
	for matched in regex.split(&haystack) {
		list.push_list(matched.to_byond()?)?;
	}
	Ok(list)
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
		None => return Ok(ByondValue::null()),
	};
	let mut list = ByondValue::new_list()?;
	for matched in regex.splitn(&haystack, limit) {
		list.push_list(matched.to_byond()?)?;
	}
	Ok(list)
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
		None => return Ok(ByondValue::null()),
	};
	regex.replace(&haystack, with).into_owned().to_byond()
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
		None => return Ok(ByondValue::null()),
	};
	regex.replace_all(&haystack, with).into_owned().to_byond()
}
