#define aneri_regex_is_match(regex, haystack) (ANERI_CALL("regex_is_match", regex, haystack))
#define aneri_regex_split(regex, haystack) (ANERI_CALL("regex_split", regex, haystack))
#define aneri_regex_replace(regex, haystack, with) (ANERI_CALL("regex_replace", regex, haystack, with))
#define aneri_regex_replace_all(regex, haystack, with) (ANERI_CALL("regex_replace_all", regex, haystack, with))

/proc/aneri_regex_splitn(regex, haystack, limit = 1)
	return ANERI_CALL("regex_splitn", regex, haystack, limit)

/proc/aneri_regex_find(regex, haystack) as /list
	return ANERI_CALL("regex_replace_find", regex, haystack)
