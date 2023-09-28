// SPDX-License-Identifier: MPL-2.0
use ahash::RandomState;
use lru::LruCache;
use meowtonin::{ByondError, ByondResult, ByondValue};
use parking_lot::Mutex;
use regex::Regex;
use std::{num::NonZeroUsize, sync::OnceLock};

// SAFETY: This is a constant value where we always know it's non-zero.
// If you change this to 0, then that is explicitly a skill issue.
const CACHE_SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(32) };

static REGEX_CACHE: OnceLock<Mutex<LruCache<String, Regex, RandomState>>> = OnceLock::new();

#[inline(always)]
fn cache() -> &'static Mutex<LruCache<String, Regex, RandomState>> {
	REGEX_CACHE
		.get_or_init(|| Mutex::new(LruCache::with_hasher(CACHE_SIZE, RandomState::default())))
}

#[byond_fn]
pub fn regex_is_match(regex: String, haystack: String) -> ByondResult<bool> {
	cache()
		.lock()
		.try_get_or_insert(regex.clone(), || Regex::new(&regex))
		.map(|regex| regex.is_match(&haystack))
		.map_err(ByondError::boxed)
}

#[byond_fn]
pub fn regex_find(regex: String, haystack: String) -> ByondResult<ByondValue> {
	let mut cache = cache().lock();
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
	let mut cache = cache().lock();
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
