// SPDX-License-Identifier: MPL-2.0
use crate::shared;
use aneri_core::{ByondSlotKey, ByondSlotMap};
use meowtonin::{ByondError, ByondResult, ByondValue, byond_fn};
use regex::Regex;

static INSTANCES: ByondSlotMap<Regex> = ByondSlotMap::new();

pub(crate) fn free_instances() {
	INSTANCES.clear();
}

#[byond_fn]
pub fn regex_new(mut src: ByondValue, regex: String) -> ByondResult<()> {
	let regex = Regex::new(&regex).map_err(ByondError::boxed)?;
	INSTANCES
		.insert(regex)
		.save(&mut src)
		.map_err(ByondError::boxed)
}

#[byond_fn]
pub fn regex_del(src: ByondSlotKey) -> bool {
	INSTANCES.remove(src).is_some()
}

#[byond_fn]
pub fn instanced_regex_is_match(src: ByondSlotKey, haystack: String) -> Option<bool> {
	INSTANCES.with(src, |regex| regex.is_match(&haystack))
}

#[byond_fn]
pub fn instanced_regex_find(src: ByondSlotKey, haystack: String) -> ByondResult<ByondValue> {
	match INSTANCES.with(src, |regex| shared::regex_find(regex, &haystack)) {
		Some(result) => result,
		None => Ok(ByondValue::NULL),
	}
}

#[byond_fn]
pub fn instanced_regex_split(src: ByondSlotKey, haystack: String) -> ByondResult<ByondValue> {
	match INSTANCES.with(src, |regex| shared::regex_split(regex, &haystack)) {
		Some(result) => result,
		None => Ok(ByondValue::NULL),
	}
}

#[byond_fn]
pub fn instanced_regex_splitn(
	src: ByondSlotKey,
	haystack: String,
	limit: usize,
) -> ByondResult<ByondValue> {
	match INSTANCES.with(src, |regex| shared::regex_splitn(regex, &haystack, limit)) {
		Some(result) => result,
		None => Ok(ByondValue::NULL),
	}
}

#[byond_fn]
pub fn instanced_regex_replace(
	src: ByondSlotKey,
	haystack: String,
	with: String,
) -> ByondResult<ByondValue> {
	match INSTANCES.with(src, |regex| shared::regex_replace(regex, &haystack, &with)) {
		Some(result) => result,
		None => Ok(ByondValue::NULL),
	}
}

#[byond_fn]
pub fn instanced_regex_replace_all(
	src: ByondSlotKey,
	haystack: String,
	with: String,
) -> ByondResult<ByondValue> {
	match INSTANCES.with(src, |regex| {
		shared::regex_replace_all(regex, &haystack, &with)
	}) {
		Some(result) => result,
		None => Ok(ByondValue::NULL),
	}
}
