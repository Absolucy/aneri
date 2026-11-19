/proc/aneri_regex_splitn(regex, haystack, limit = 1)
	return ANERI_CALL(regex_splitn, regex, haystack, limit)

/proc/aneri_regex_find(regex, haystack) as /list
	return ANERI_CALL(regex_find, regex, haystack)
