/proc/arand_alphanumeric_string(length = 8, secure = FALSE)
	return ANERI_CALL("random_string_alphanumeric", length, secure)

/proc/arand_byte(secure = FALSE)
	return ANERI_CALL("random_byte", secure)

/proc/arand_float(secure = FALSE)
	return ANERI_CALL("random_float", secure)

/proc/arand_uint(secure = FALSE)
	return ANERI_CALL("random_int_unsigned", secure)

/proc/arand_range_int_unsigned(min = 0, max = 1, secure = FALSE)
	return ANERI_CALL("random_range_int_unsigned", min, max, secure)

/proc/arand_int(secure = FALSE)
	return ANERI_CALL("random_int_signed", secure)

/proc/arand_range_int_signed(min = 0, max = 1, secure = FALSE)
	return ANERI_CALL("random_range_int_signed", min, max, secure)

/proc/aprob(probability, secure = FALSE)
	return ANERI_CALL("prob", probability, secure)

/proc/aprob_ratio(numerator, denominator, secure = FALSE)
	return ANERI_CALL("prob_ratio", numerator, denominator, secure)

/proc/apick(list/values, secure = FALSE)
	ANERI_CALL("pick", values, secure)

/proc/apick_weight(list/values, secure = FALSE)
	ANERI_CALL("pick_weight", values, secure)
