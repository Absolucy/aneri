// SPDX-License-Identifier: MPL-2.0
use meowtonin::{ByondResult, ByondValue, ToByond};
use regex::Regex;

#[derive(Debug)]
/// Represents data for a single capture group in a regex match.
pub(crate) struct CaptureData {
	/// The name of the capture group, if it has one.
	/// `None` for unnamed capture groups.
	name: Option<String>,
	/// The starting index of the capture in the original string.
	start: usize,
	/// The ending index of the capture in the original string.
	end: usize,
	/// The actual string value of the captured text.
	value: String,
}

impl ToByond for CaptureData {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		let name = self.name.to_byond()?;
		let start = self.start.to_byond()?;
		let end = self.end.to_byond()?;
		let value = self.value.to_byond()?;
		ByondValue::new("/datum/regex_capture_group", [name, start, end, value])
	}
}

#[derive(Debug)]
pub(crate) struct MatchData {
	/// The full string that matched the entire regex pattern.
	rmatch: String,
	/// The starting index of the entire match in the original string.
	start: usize,
	/// The ending index of the entire match in the original string.
	end: usize,
	/// A list containing data for each capture group in the match.
	/// This includes both named and unnamed captures.
	captures: Vec<CaptureData>,
}

impl ToByond for MatchData {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		let rmatch = self.rmatch.to_byond()?;
		let start = self.start.to_byond()?;
		let end = self.end.to_byond()?;
		let captures = self.captures.to_byond()?;
		ByondValue::new("/datum/regex_match", [rmatch, start, end, captures])
	}
}

pub(crate) fn regex_capture(regex: &Regex, input: &str) -> Vec<MatchData> {
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
					let name = regex
						.capture_names()
						.nth(idx)
						.and_then(|n| n.map(|s| s.to_string()));
					Some((name, capture))
				})
				.map(|(name, capture)| CaptureData {
					name,
					start: capture.start() + 1,
					end: capture.end() + 1,
					value: capture.as_str().to_string(),
				})
				.collect::<Vec<CaptureData>>();
			MatchData {
				rmatch: full_match.as_str().to_string(),
				start: full_match.start() + 1,
				end: full_match.end() + 1,
				captures,
			}
		})
		.collect::<Vec<MatchData>>()
}
