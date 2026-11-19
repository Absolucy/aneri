/proc/aneri_rand_ranged_uint(min = 0, max, secure = FALSE) as num
	return ANERI_CALL("random_range_int_unsigned", min, max, secure)

/proc/aneri_rand_ranged_int(min = 0, max, secure = FALSE) as num
	return ANERI_CALL("random_range_int_signed", min, max, secure)

/proc/aneri_rand_pick(list/choices, secure = FALSE)
	RETURN_TYPE(choices[_].type)
	return ANERI_CALL("pick", choices, secure)

/proc/aneri_rand_pick_weighted(list/choices, secure = FALSE)
	RETURN_TYPE(choices[_].type)
	return ANERI_CALL("pick_weighted", choices, secure)

/proc/aneri_rand_string(length = 16, secure = FALSE) as text
	return ANERI_CALL("random_string_alphanumeric", length, secure)

/proc/aneri_rand_replace_chars_prob(input, replacement, probability = 25, skip_whitespace = FALSE, secure = FALSE) as text
	return ANERI_CALL("replace_chars_prob", input, replacement, probability, skip_whitespace, secure)
