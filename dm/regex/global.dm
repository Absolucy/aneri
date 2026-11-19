/// Splits `haystack` on matches of `regex`, into at most `limit` pieces.
/// The last piece holds the rest of the text, unsplit.
/proc/aneri_regex_splitn(regex, haystack, limit = 1)
	return ANERI_CALL(regex_splitn, regex, haystack, limit)

/// Every match of `regex` in `haystack`, as a list of `/datum/regex_match`.
/// Uses Rust regex syntax, which has no lookaround or backreferences.
/proc/aneri_regex_find(regex, haystack) as /list
	return ANERI_CALL(regex_find, regex, haystack)
