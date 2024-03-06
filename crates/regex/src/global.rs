// SPDX-License-Identifier: MPL-2.0
use ahash::RandomState;
use lru::LruCache;
use meowtonin::{ByondError, ByondResult, ByondValue, ToByond};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use regex::Regex;
use std::num::NonZeroUsize;

// SAFETY: This is a constant value where we always know it's non-zero.
// If you change this to 0, then that is explicitly a skill issue.
const CACHE_SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(32) };

static REGEX_CACHE: Lazy<Mutex<LruCache<String, Regex, RandomState>>> =
	Lazy::new(|| Mutex::new(LruCache::with_hasher(CACHE_SIZE, RandomState::default())));

pub fn clear_cache() {
	if let Some(cache) = Lazy::get(&REGEX_CACHE) {
		cache.lock().clear();
	}
}

#[byond_fn]
pub fn regex_is_match(regex: String, haystack: String) -> ByondResult<bool> {
	REGEX_CACHE
		.lock()
		.try_get_or_insert(regex.clone(), || Regex::new(&regex))
		.map(|regex| regex.is_match(&haystack))
		.map_err(ByondError::boxed)
}

#[byond_fn]
pub fn regex_find(regex: String, haystack: String) -> ByondResult<ByondValue> {
	let mut cache = REGEX_CACHE.lock();
	let regex = cache
		.try_get_or_insert(regex.clone(), || Regex::new(&regex))
		.map_err(ByondError::boxed)?;
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
pub fn regex_find_all(regex: String, haystack: String) -> ByondResult<ByondValue> {
	let mut cache = REGEX_CACHE.lock();
	let regex = cache
		.try_get_or_insert(regex.clone(), || Regex::new(&regex))
		.map_err(ByondError::boxed)?;
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

#[byond_fn]
pub fn regex_split(regex: String, haystack: String) -> ByondResult<ByondValue> {
	let mut cache = REGEX_CACHE.lock();
	let regex = cache
		.try_get_or_insert(regex.clone(), || Regex::new(&regex))
		.map_err(ByondError::boxed)?;
	let mut list = ByondValue::new_list()?;
	for matched in regex.split(&haystack) {
		list.push_list(matched.to_byond()?)?;
	}
	Ok(list)
}

#[byond_fn]
pub fn regex_splitn(regex: String, haystack: String, limit: usize) -> ByondResult<ByondValue> {
	let mut cache = REGEX_CACHE.lock();
	let regex = cache
		.try_get_or_insert(regex.clone(), || Regex::new(&regex))
		.map_err(ByondError::boxed)?;
	let mut list = ByondValue::new_list()?;
	for matched in regex.splitn(&haystack, limit) {
		list.push_list(matched.to_byond()?)?;
	}
	Ok(list)
}

#[byond_fn]
pub fn regex_replace(regex: String, haystack: String, with: String) -> ByondResult<ByondValue> {
	let mut cache = REGEX_CACHE.lock();
	let regex = cache
		.try_get_or_insert(regex.clone(), || Regex::new(&regex))
		.map_err(ByondError::boxed)?;
	regex.replace(&haystack, with).into_owned().to_byond()
}

#[byond_fn]
pub fn regex_replace_all(regex: String, haystack: String, with: String) -> ByondResult<ByondValue> {
	let mut cache = REGEX_CACHE.lock();
	let regex = cache
		.try_get_or_insert(regex.clone(), || Regex::new(&regex))
		.map_err(ByondError::boxed)?;
	regex.replace_all(&haystack, with).into_owned().to_byond()
}
