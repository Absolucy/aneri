/datum/aneri/rng/New(secure = FALSE, seed)
	ANERI_CALL("rng_new", src, secure, seed)

/datum/aneri/rng/Del()
	ANERI_CALL("rng_del", src)

/datum/aneri/rng/proc/alphanumeric_string(length = 8)
	return ANERI_CALL("instanced_random_string_alphanumeric", src, length)

/datum/aneri/rng/proc/byte()
	return ANERI_CALL("instanced_random_byte", src)

/datum/aneri/rng/proc/float()
	return ANERI_CALL("instanced_random_float", src)

/datum/aneri/rng/proc/uint()
	return ANERI_CALL("instanced_random_int_unsigned", src)

/datum/aneri/rng/proc/int()
	return ANERI_CALL("instanced_random_int_signed", src)

/datum/aneri/rng/proc/uint_range(min = 0, max = 100)
	return ANERI_CALL("instanced_random_range_int_unsigned", src, min, max)

/datum/aneri/rng/proc/int_range(min = 0, max = 100)
	return ANERI_CALL("instanced_random_range_int_signed", src, min, max)

/datum/aneri/rng/proc/chance(val)
	return ANERI_CALL("instanced_prob", src, val)

/datum/aneri/rng/proc/chance_ratio(numerator, denominator)
	return ANERI_CALL("instanced_prob_ratio", src, numerator, denominator)

/datum/aneri/rng/proc/pick_from(list/values)
	return ANERI_CALL("instanced_pick", src, values)

/datum/aneri/rng/proc/weighted_pick_from(list/values)
	return ANERI_CALL("instanced_pick_weighted", src, values)
