// SPDX-License-Identifier: MPL-2.0
use aneri_core::ByondSlotKey;
use meowtonin::{ByondError, ByondResult, ByondValue};
use parking_lot::RwLock;
use regex::Regex;
use slotmap::SlotMap;
use std::sync::OnceLock;

static INSTANCES: OnceLock<RwLock<SlotMap<ByondSlotKey, Regex>>> = OnceLock::new();

pub(crate) fn free_instances() {
	if let Some(instances) = INSTANCES.get() {
		instances.write().clear();
	}
}

#[inline(always)]
fn instances() -> &'static RwLock<SlotMap<ByondSlotKey, Regex>> {
	INSTANCES.get_or_init(RwLock::default)
}

#[byond_fn]
pub fn regex_new(mut src: ByondValue, regex: String) -> ByondResult<()> {
	let mut instances = instances().write();
	let regex = Regex::new(&regex).map_err(ByondError::boxed)?;
	instances
		.insert(regex)
		.save(&mut src)
		.map_err(ByondError::boxed)
}

#[byond_fn]
pub fn regex_del(src: ByondSlotKey) -> bool {
	instances().write().remove(src).is_some()
}

#[byond_fn]
pub fn instanced_regex_is_match(src: ByondSlotKey, haystack: String) -> Option<bool> {
	instances()
		.read()
		.get(src)
		.map(|regex| regex.is_match(&haystack))
}

#[byond_fn]
pub fn instanced_regex_find(src: ByondSlotKey, haystack: String) -> ByondResult<ByondValue> {
	let instances = instances().read();
	let regex = match instances.get(src) {
		Some(regex) => regex,
		None => return Ok(ByondValue::null()),
	};
	let mut list = ByondValue::new_list()?;
	if let Some(matched) = regex.find(&haystack) {
		let substring = byondval!(const "text");
		let start = byondval!(const "start");
		let end = byondval!(const "end");
		list.write_list_index(substring, matched.as_str())?;
		list.write_list_index(start, matched.start() + 1)?;
		list.write_list_index(end, matched.end() + 1)?;
	}
	Ok(list)
}

#[byond_fn]
pub fn instanced_regex_find_all(src: ByondSlotKey, haystack: String) -> ByondResult<ByondValue> {
	let instances = instances().read();
	let regex = match instances.get(src) {
		Some(regex) => regex,
		None => return Ok(ByondValue::null()),
	};
	let mut list = ByondValue::new_list()?;
	for matched in regex.find_iter(&haystack) {
		let mut match_list = ByondValue::new_list()?;
		let substring = byondval!(const "text");
		let start = byondval!(const "start");
		let end = byondval!(const "end");
		match_list.write_list_index(substring, matched.as_str())?;
		match_list.write_list_index(start, matched.start() + 1)?;
		match_list.write_list_index(end, matched.end() + 1)?;
		list.push_list(match_list)?;
	}
	Ok(list)
}
