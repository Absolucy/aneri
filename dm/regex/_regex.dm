/// Represents data for a single capture group in a regex match.
/datum/regex_capture_group
	/// The name of the capture group, if it has one.
	/// Unnamed capture groups will be null.
	var/name
	/// Byte position where the capture starts, counting from 1.
	var/start
	/// Byte position just after the capture, so `copytext(haystack, start, end)` is the capture.
	var/end
	/// The actual string value of the captured text.
	var/value

/datum/regex_capture_group/New(name, start, end, value)
	src.name = name
	src.start = start
	src.end = end
	src.value = value

/// Represents data for a complete regex match, including all capture groups.
/datum/regex_match
	/// The full string that matched the entire regex pattern.
	var/match
	/// Byte position where the match starts, counting from 1.
	var/start
	/// Byte position just after the match, so `copytext(haystack, start, end)` is the match.
	var/end
	/// A list containing data for each capture group in the match.
	/// This includes both named and unnamed captures.
	var/list/datum/regex_capture_group/captures

/datum/regex_match/New(match, start, end, list/datum/regex_capture_group/captures)
	src.match = match
	src.start = start
	src.end = end
	src.captures = captures

/// Finds a specific capture group by name.
/datum/regex_match/proc/find_capture(group) as /datum/regex_capture_group
	RETURN_TYPE(/datum/regex_capture_group)
	for(var/datum/regex_capture_group/capture as anything in captures)
		if(capture.name == group)
			return capture
