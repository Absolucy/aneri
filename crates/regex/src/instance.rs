// SPDX-License-Identifier: MPL-2.0
use crate::shared;
use aneri_core::{ByondSlotKey, ByondSlotMap};
use meowtonin::{ByondError, ByondResult, ByondValue, byond_fn};
use regex::Regex;

static INSTANCES: ByondSlotMap<Regex> = ByondSlotMap::new();

pub(crate) fn free_instances() {
	INSTANCES.clear();
}

/// Compiles the pattern behind a `/datum/regex` and stores its key on `src`.
/// Runtimes on an invalid pattern.
#[byond_fn(name = "aneri_raw_regex_new")]
pub fn regex_new(mut src: ByondValue, regex: String) -> ByondResult<()> {
	let regex = Regex::new(&regex).map_err(ByondError::boxed)?;
	INSTANCES
		.insert(regex)
		.save(&mut src)
		.map_err(ByondError::boxed)
}

/// Deletes the compiled pattern behind a `/datum/regex`.
/// Returns false if it was already gone.
#[byond_fn(name = "aneri_raw_regex_del")]
pub fn regex_del(src: ByondSlotKey) -> bool {
	INSTANCES.remove(src).is_some()
}

/// `aneri_regex_is_match` on a `/datum/regex`.
/// Null if the regex was deleted.
#[byond_fn(name = "aneri_raw_instanced_regex_is_match")]
pub fn instanced_regex_is_match(src: ByondSlotKey, haystack: String) -> Option<bool> {
	INSTANCES.with(src, |regex| regex.is_match(&haystack))
}

/// `aneri_regex_find` on a `/datum/regex`.
/// Null if the regex was deleted.
#[byond_fn(name = "aneri_raw_instanced_regex_find")]
pub fn instanced_regex_find(src: ByondSlotKey, haystack: String) -> ByondResult<ByondValue> {
	match INSTANCES.with(src, |regex| shared::regex_find(regex, &haystack)) {
		Some(result) => result,
		None => Ok(ByondValue::NULL),
	}
}

/// `aneri_regex_split` on a `/datum/regex`.
/// Null if the regex was deleted.
#[byond_fn(name = "aneri_raw_instanced_regex_split")]
pub fn instanced_regex_split(src: ByondSlotKey, haystack: String) -> ByondResult<ByondValue> {
	match INSTANCES.with(src, |regex| shared::regex_split(regex, &haystack)) {
		Some(result) => result,
		None => Ok(ByondValue::NULL),
	}
}

/// `aneri_regex_splitn` on a `/datum/regex`.
/// Null if the regex was deleted.
#[byond_fn(name = "aneri_raw_instanced_regex_splitn")]
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

/// `aneri_regex_replace` on a `/datum/regex`.
/// Null if the regex was deleted.
#[byond_fn(name = "aneri_raw_instanced_regex_replace")]
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

/// `aneri_regex_replace_all` on a `/datum/regex`.
/// Null if the regex was deleted.
#[byond_fn(name = "aneri_raw_instanced_regex_replace_all")]
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
