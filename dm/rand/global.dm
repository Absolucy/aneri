/// A random whole number from `min` to `max`, both included.
/// Runtimes unless both are whole numbers, 0 or more.
/proc/aneri_rand_ranged_uint(min = 0, max, secure = FALSE) as num
	return ANERI_CALL(random_range_int_unsigned, min, max, secure)

/// A random whole number from `min` to `max`, both included.
/// Runtimes unless both are whole numbers.
/proc/aneri_rand_ranged_int(min = 0, max, secure = FALSE) as num
	return ANERI_CALL(random_range_int_signed, min, max, secure)

/// A random item from `choices`.
/// Returns null for an empty list.
/proc/aneri_rand_pick(list/choices, secure = FALSE)
	RETURN_TYPE(choices[_].type)
	return ANERI_CALL(pick, choices, secure)

/// A random key from an assoc list of item to weight.
/// An item with weight 2 is picked twice as often as one with weight 1.
/// Items whose weight isn't a positive number are skipped.
/proc/aneri_rand_pick_weighted(list/choices, secure = FALSE)
	RETURN_TYPE(choices[_].type)
	return ANERI_CALL(pick_weighted, choices, secure)

/// A random string of `length` letters and digits.
/proc/aneri_rand_string(length = 16, secure = FALSE) as text
	return ANERI_CALL(random_string_alphanumeric, length, secure)

/// Replaces each character of `input` with `replacement`, each with a `probability` percent chance.
/// Useful for garbling text.
/proc/aneri_rand_replace_chars_prob(input, replacement, probability = 25, skip_whitespace = FALSE, secure = FALSE) as text
	return ANERI_CALL(replace_chars_prob, input, replacement, probability, skip_whitespace, secure)
