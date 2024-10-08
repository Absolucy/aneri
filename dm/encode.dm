#define aneri_url_encode(data)			(ANERI_CALL("url_encode", data))
#define aneri_url_decode(data)			(ANERI_CALL("url_decode", data))
#define aneri_hex_decode(data)			(ANERI_CALL("hex_decode", data))
#define aneri_base64_encode(data)		(ANERI_CALL("base64_encode", data))
#define aneri_base64_decode(data)		(ANERI_CALL("base64_decode", data))
#define aneri_base64url_encode(data)	(ANERI_CALL("base64url_encode", data))
#define aneri_base64url_decode(data)	(ANERI_CALL("base64url_decode", data))

/proc/aneri_hex_encode(data, upper = FALSE)
	return ANERI_CALL("hex_encode", data, upper)
