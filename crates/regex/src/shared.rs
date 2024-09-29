// SPDX-License-Identifier: MPL-2.0
use meowtonin::{ByondResult, ByondValue, ToByond};
use regex::Regex;

pub(crate) fn regex_find(regex: &Regex, haystack: &str) -> ByondResult<ByondValue> {
	crate::capture::regex_capture(regex, haystack).to_byond()
}

pub(crate) fn regex_split(regex: &Regex, haystack: &str) -> ByondResult<ByondValue> {
	let mut list = ByondValue::new_list()?;
	for matched in regex.split(haystack) {
		list.push_list(matched.to_byond()?)?;
	}
	Ok(list)
}

pub(crate) fn regex_splitn(regex: &Regex, haystack: &str, limit: usize) -> ByondResult<ByondValue> {
	let mut list = ByondValue::new_list()?;
	for matched in regex.splitn(haystack, limit) {
		list.push_list(matched.to_byond()?)?;
	}
	Ok(list)
}

pub(crate) fn regex_replace(regex: &Regex, haystack: &str, with: &str) -> ByondResult<ByondValue> {
	regex.replace(haystack, with).into_owned().to_byond()
}

pub(crate) fn regex_replace_all(
	regex: &Regex,
	haystack: &str,
	with: &str,
) -> ByondResult<ByondValue> {
	regex.replace_all(haystack, with).into_owned().to_byond()
}
