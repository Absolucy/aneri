/// A (non-cryptographic) PRNG instance, using rand's `SmallRng`
/// (last I checked, `SmallRng` is `Xoshiro128PlusPlus` - unless you're on 64-bit OpenDream, then it's `Xoshiro256PlusPlus`).
/// Pass a `seed` to New() to get the same numbers every time.
/// Every proc returns null once the rng is deleted.
/datum/rng
	parent_type = /datum/aneri
	var/secure = FALSE

/datum/rng/New(seed = null)
	var/err = ANERI_CALL(rng_new, src, secure, seed)
	// returns null if everything was fine, else returns error info
	if(err)
		CRASH("failed to initialize rng: [err]")

/datum/rng/Destroy()
	if(!ANERI_CALL(rng_del, src))
		CRASH("attempted to delete nonexistent rng instance")
	return ..()

/// A random whole number from 0 to 255.
/datum/rng/proc/byte() as num
	return ANERI_CALL(instanced_random_byte, src)

/// A random number from 0 up to, but not including, 1.
/datum/rng/proc/float() as num
	return ANERI_CALL(instanced_random_float, src)

/// A random whole number from 0 to 4294967295.
/// DM numbers lose precision above 16777216, so big results get rounded.
/datum/rng/proc/uint() as num
	return ANERI_CALL(instanced_random_int_unsigned, src)

/// A random whole number from -2147483648 to 2147483647.
/// DM numbers lose precision past 16777216 either way, so big results get rounded.
/datum/rng/proc/int() as num
	return ANERI_CALL(instanced_random_int_signed, src)

/// A random whole number from `min` to `max`, both included.
/// Runtimes unless both are whole numbers, 0 or more.
/datum/rng/proc/ranged_uint(min = 0, max) as num
	if(!isnum(min) || !isnum(max))
		CRASH("invalid arguments to /datum/rng/proc/ranged_uint")
	return ANERI_CALL(instanced_random_range_int_unsigned, src, min, max)

/// A random whole number from `min` to `max`, both included.
/// Runtimes unless both are whole numbers.
/datum/rng/proc/ranged_int(min = 0, max)
	if(!isnum(min) || !isnum(max))
		CRASH("invalid arguments to /datum/rng/proc/ranged_int")
	return ANERI_CALL(instanced_random_range_int_signed, src, min, max)

/// A random item from `choices`.
/// Returns null for an empty list.
/datum/rng/proc/pick_from(list/choices)
	RETURN_TYPE(choices[_].type)
	if(!islist(choices))
		CRASH("Attempted to do a pick from a non-list")
	return ANERI_CALL(instanced_pick, src, choices)

/// A random key from an assoc list of item to weight.
/// An item with weight 2 is picked twice as often as one with weight 1.
/datum/rng/proc/pick_weighted(list/choices)
	RETURN_TYPE(choices[_].type)
	if(!islist(choices))
		CRASH("Attempted to do a weighted pick from a non-list")
	return ANERI_CALL(instanced_pick_weighted, src, choices)

/// Returns TRUE `percent` percent of the time, like DM's `prob()`.
/datum/rng/proc/chance(percent) // "prob" is a reserved word, so we do "chance" instead
	return ANERI_CALL(instanced_prob, src, percent)

/// Returns TRUE `numerator` in `denominator` times.
/// Runtimes if `denominator` is 0 or smaller than `numerator`.
/datum/rng/proc/prob_ratio(numerator, denominator)
	return ANERI_CALL(instanced_prob_ratio, src, numerator, denominator)

/// A random string of `length` letters and digits.
/datum/rng/proc/string(length = 16) as text
	return ANERI_CALL(instanced_random_string_alphanumeric, src, length)

/// Replaces each character of `input` with `replacement`, each with a `probability` percent chance.
/datum/rng/proc/replace_chars_prob(input, replacement, probability = 25, skip_whitespace = FALSE) as text
	return ANERI_CALL(instanced_replace_chars_prob, src, input, replacement, probability, skip_whitespace)

/// A cryptographic PRNG instance, using ChaCha12.
/datum/rng/secure
	secure = TRUE
