// SPDX-License-Identifier: MPL-2.0
use crate::shared;
use ahash::RandomState;
use lru::LruCache;
use meowtonin::{byond_fn, ByondError, ByondResult, ByondValue};
use parking_lot::Mutex;
use regex::Regex;
use std::{num::NonZeroUsize, sync::LazyLock};

// SAFETY: This is a constant value where we always know it's non-zero.
// If you change this to 0, then that is explicitly a skill issue.
const CACHE_SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(32) };

static REGEX_CACHE: LazyLock<Mutex<LruCache<String, Regex, RandomState>>> =
	LazyLock::new(|| Mutex::new(LruCache::with_hasher(CACHE_SIZE, RandomState::default())));

pub fn clear_cache() {
	REGEX_CACHE.lock().clear();
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
	shared::regex_find(regex, &haystack)
}

#[byond_fn]
pub fn regex_split(regex: String, haystack: String) -> ByondResult<ByondValue> {
	let mut cache = REGEX_CACHE.lock();
	let regex = cache
		.try_get_or_insert(regex.clone(), || Regex::new(&regex))
		.map_err(ByondError::boxed)?;
	shared::regex_split(regex, &haystack)
}

#[byond_fn]
pub fn regex_splitn(regex: String, haystack: String, limit: usize) -> ByondResult<ByondValue> {
	let mut cache = REGEX_CACHE.lock();
	let regex = cache
		.try_get_or_insert(regex.clone(), || Regex::new(&regex))
		.map_err(ByondError::boxed)?;
	shared::regex_splitn(regex, &haystack, limit)
}

#[byond_fn]
pub fn regex_replace(regex: String, haystack: String, with: String) -> ByondResult<ByondValue> {
	let mut cache = REGEX_CACHE.lock();
	let regex = cache
		.try_get_or_insert(regex.clone(), || Regex::new(&regex))
		.map_err(ByondError::boxed)?;
	shared::regex_replace(regex, &haystack, &with)
}

#[byond_fn]
pub fn regex_replace_all(regex: String, haystack: String, with: String) -> ByondResult<ByondValue> {
	let mut cache = REGEX_CACHE.lock();
	let regex = cache
		.try_get_or_insert(regex.clone(), || Regex::new(&regex))
		.map_err(ByondError::boxed)?;
	shared::regex_replace_all(regex, &haystack, &with)
}
