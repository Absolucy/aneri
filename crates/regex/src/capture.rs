// SPDX-License-Identifier: MPL-2.0
use meowtonin::{ByondResult, ByondValue, ToByond};
use regex::Regex;

#[derive(Debug)]
/// Represents data for a single capture group in a regex match.
pub(crate) struct CaptureData<'a> {
	/// The name of the capture group, if it has one.
	/// `None` for unnamed capture groups.
	name: Option<&'a str>,
	/// The starting index of the capture in the original string.
	start: usize,
	/// The ending index of the capture in the original string.
	end: usize,
	/// The actual string value of the captured text.
	value: &'a str,
}

impl ToByond for CaptureData<'_> {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		let name = self.name.to_byond()?;
		let start = self.start.to_byond()?;
		let end = self.end.to_byond()?;
		let value = self.value.to_byond()?;
		ByondValue::new("/datum/regex_capture_group", [name, start, end, value])
	}
}

#[derive(Debug)]
pub(crate) struct MatchData<'a> {
	/// The full string that matched the entire regex pattern.
	rmatch: &'a str,
	/// The starting index of the entire match in the original string.
	start: usize,
	/// The ending index of the entire match in the original string.
	end: usize,
	/// A list containing data for each capture group in the match.
	/// This includes both named and unnamed captures.
	captures: Vec<CaptureData<'a>>,
}

impl ToByond for MatchData<'_> {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		let rmatch = self.rmatch.to_byond()?;
		let start = self.start.to_byond()?;
		let end = self.end.to_byond()?;
		let captures = self.captures.to_byond()?;
		ByondValue::new("/datum/regex_match", [rmatch, start, end, captures])
	}
}

pub(crate) fn regex_capture<'a>(regex: &'a Regex, input: &'a str) -> Vec<MatchData<'a>> {
	let names = regex.capture_names().collect::<Vec<Option<&str>>>();
	regex
		.captures_iter(input)
		.filter_map(|captures| -> Option<_> {
			let full_match = captures.get(0)?;
			Some((captures, full_match))
		})
		.map(|(captures, full_match)| {
			let captures = captures
				.iter()
				.enumerate()
				.flat_map(|(idx, capture)| {
					let capture = capture?;
					Some((names[idx], capture))
				})
				.map(|(name, capture)| CaptureData {
					name,
					start: capture.start() + 1,
					end: capture.end() + 1,
					value: capture.as_str(),
				})
				.collect::<Vec<CaptureData>>();
			MatchData {
				rmatch: full_match.as_str(),
				start: full_match.start() + 1,
				end: full_match.end() + 1,
				captures,
			}
		})
		.collect::<Vec<MatchData>>()
}
