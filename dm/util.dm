#define aneri_json_is_valid(json)		(ANERI_CALL("json_is_valid", json))
#define aneri_toml_is_valid(toml)		(ANERI_CALL("toml_is_valid", toml))
#define aneri_toml_file_is_valid(file)	(ANERI_CALL("toml_file_is_valid", "[file]"))

#define aneri_levenshtein(a, b)						(ANERI_CALL("levenshtein", a, b))
#define aneri_damerau_levenshtein(a, b)				(ANERI_CALL("damerau_levenshtein", a, b))
#define aneri_normalized_levenshtein(a, b)			(ANERI_CALL("normalized_levenshtein", a, b))
#define aneri_normalized_damerau_levenshtein(a, b)	(ANERI_CALL("normalized_damerau_levenshtein", a, b))
#define aneri_hamming(a, b)							(ANERI_CALL("hamming", a, b))

/proc/aneri_deunicode(string, placeholder)
	return ANERI_CALL("deunicode", string, placeholder)

/proc/aneri_toml_decode(toml) as /list
	RETURN_TYPE(/list)
	return ANERI_CALL("toml_decode", toml)

/proc/aneri_toml_decode_file(file) as /list
	RETURN_TYPE(/list)
	return ANERI_CALL("toml_decode_file", "[file]")

/proc/aneri_uuid()
	return ANERI_CALL("uuid")

/proc/aneri_cuid2(length = null)
	return ANERI_CALL("cuid2", length)
