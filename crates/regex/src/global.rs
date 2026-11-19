// SPDX-License-Identifier: MPL-2.0
use crate::shared;
use lru::LruCache;
use meowtonin::{ByondError, ByondResult, ByondValue, byond_fn};
use parking_lot::Mutex;
use regex::Regex;
use rustc_hash::FxBuildHasher;
use std::{num::NonZeroUsize, sync::LazyLock};

const CACHE_SIZE: NonZeroUsize = NonZeroUsize::new(32).unwrap();

static REGEX_CACHE: LazyLock<Mutex<LruCache<String, Regex, FxBuildHasher>>> =
	LazyLock::new(|| Mutex::new(LruCache::with_hasher(CACHE_SIZE, FxBuildHasher)));

pub fn clear_cache() {
	REGEX_CACHE.lock().clear();
}

/// Whether `regex` matches anywhere in `haystack`.
/// Uses Rust regex syntax, which has no lookaround or backreferences.
/// Runtimes on an invalid pattern.
/// The last 32 patterns stay compiled, so reusing one is cheap.
#[byond_fn]
pub fn regex_is_match(regex: String, haystack: String) -> ByondResult<bool> {
	REGEX_CACHE
		.lock()
		.try_get_or_insert_ref(regex.as_str(), || Regex::new(&regex))
		.map(|regex| regex.is_match(&haystack))
		.map_err(ByondError::boxed)
}

/// Every match of `regex` in `haystack`, as a list of `/datum/regex_match`.
/// Positions are byte offsets, starting at 1.
/// Runtimes on an invalid pattern.
#[byond_fn(name = "aneri_raw_regex_find")]
pub fn regex_find(regex: String, haystack: String) -> ByondResult<ByondValue> {
	let mut cache = REGEX_CACHE.lock();
	let regex = cache
		.try_get_or_insert_ref(regex.as_str(), || Regex::new(&regex))
		.map_err(ByondError::boxed)?;
	shared::regex_find(regex, &haystack)
}

/// Splits `haystack` on every match of `regex` and returns the pieces.
/// Runtimes on an invalid pattern.
#[byond_fn]
pub fn regex_split(regex: String, haystack: String) -> ByondResult<ByondValue> {
	let mut cache = REGEX_CACHE.lock();
	let regex = cache
		.try_get_or_insert_ref(regex.as_str(), || Regex::new(&regex))
		.map_err(ByondError::boxed)?;
	shared::regex_split(regex, &haystack)
}

/// Like `aneri_regex_split`, but returns at most `limit` pieces.
/// The last piece holds the rest of the text, unsplit.
#[byond_fn(name = "aneri_raw_regex_splitn")]
pub fn regex_splitn(regex: String, haystack: String, limit: usize) -> ByondResult<ByondValue> {
	let mut cache = REGEX_CACHE.lock();
	let regex = cache
		.try_get_or_insert_ref(regex.as_str(), || Regex::new(&regex))
		.map_err(ByondError::boxed)?;
	shared::regex_splitn(regex, &haystack, limit)
}

/// Replaces the first match of `regex` in `haystack`.
/// Runtimes on an invalid pattern.
#[byond_fn]
pub fn regex_replace(
	regex: String,
	haystack: String,
	/// The replacement.
	/// `$1` or `$name` inserts a capture group, `$$` is a literal `$`.
	with: String,
) -> ByondResult<ByondValue> {
	let mut cache = REGEX_CACHE.lock();
	let regex = cache
		.try_get_or_insert_ref(regex.as_str(), || Regex::new(&regex))
		.map_err(ByondError::boxed)?;
	shared::regex_replace(regex, &haystack, &with)
}

/// Replaces every match of `regex` in `haystack`.
/// Runtimes on an invalid pattern.
#[byond_fn]
pub fn regex_replace_all(
	regex: String,
	haystack: String,
	/// The replacement.
	/// `$1` or `$name` inserts a capture group, `$$` is a literal `$`.
	with: String,
) -> ByondResult<ByondValue> {
	let mut cache = REGEX_CACHE.lock();
	let regex = cache
		.try_get_or_insert_ref(regex.as_str(), || Regex::new(&regex))
		.map_err(ByondError::boxed)?;
	shared::regex_replace_all(regex, &haystack, &with)
}
