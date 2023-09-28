#define aneri_json_is_valid(text) ANERI_CALL("json_is_valid", text)

/proc/aneri_hex_encode(input, upper = 0) return ANERI_CALL("hex_encode", input, upper)
#define aneri_hex_decode(input) ANERI_CALL("hex_decode", input)


#define aneri_url_encode(input) ANERI_CALL("url_encode", input)
#define aneri_url_decode(input) ANERI_CALL("url_decode", input)

#define aneri_base64_encode(input)		ANERI_CALL("base64_encode", input)
#define aneri_base64_decode(input)		ANERI_CALL("base64_decode", input)
#define aneri_base64url_encode(input)	ANERI_CALL("base64url_encode", input)
#define aneri_base64url_decode(input)	ANERI_CALL("base64url_decode", input)

/proc/aneri_uuid()				return ANERI_CALL("uuid")
/proc/aneri_unix_timestamp()	return ANERI_CALL("unix_timestamp")
