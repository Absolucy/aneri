/// A compiled regex, for patterns used often.
/// Uses Rust regex syntax, which has no lookaround or backreferences.
/// Every proc returns null once the regex is deleted.
/datum/regex
	parent_type = /datum/aneri

/datum/regex/New(pattern)
	var/err = ANERI_CALL(regex_new, src, pattern)
	// returns null if everything was fine, else returns error info
	if(err)
		CRASH("failed to initialize regex: [err]")

/datum/regex/Destroy()
	if(!ANERI_CALL(regex_del, src))
		CRASH("attempted to delete nonexistent regex instance")
	return ..()

/// Whether the regex matches anywhere in `haystack`.
/datum/regex/proc/is_match(haystack)
	return ANERI_CALL(instanced_regex_is_match, src, haystack)

/// Every match in `haystack`, as a list of `/datum/regex_match`.
/datum/regex/proc/find(haystack) as /list
	return ANERI_CALL(instanced_regex_find, src, haystack)

/// Splits `haystack` on every match and returns the pieces.
/datum/regex/proc/split(haystack)
	return ANERI_CALL(instanced_regex_split, src, haystack)

/// Splits `haystack` into at most `limit` pieces.
/// The last piece holds the rest of the text, unsplit.
/datum/regex/proc/splitn(haystack, limit = 1)
	return ANERI_CALL(instanced_regex_splitn, src, haystack, limit)

/// Replaces the first match in `haystack`.
/// `$1` or `$name` in `with` inserts a capture group, `$$` is a literal `$`.
/datum/regex/proc/replace(haystack, with = "")
	return ANERI_CALL(instanced_regex_replace, src, haystack, with)

/// Replaces every match in `haystack`.
/// `$1` or `$name` in `with` inserts a capture group, `$$` is a literal `$`.
/datum/regex/proc/replace_all(haystack, with = "")
	return ANERI_CALL(instanced_regex_replace_all, src, haystack, with)
