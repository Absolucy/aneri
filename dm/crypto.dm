#define aneri_blake3(data)		ANERI_CALL("blake3", "[data]")
#define aneri_blake3_file(file) ANERI_CALL("blake3_file", "[file]")

#define aneri_crc32(data)		ANERI_CALL("crc32", "[data]")
#define aneri_crc32_file(file)	ANERI_CALL("crc32_file", "[file]")

#define aneri_crc32c(data)		ANERI_CALL("crc32c", "[data]")
#define aneri_crc32c_file(file)	ANERI_CALL("crc32c_file", "[file]")

#define aneri_md5(data)			ANERI_CALL("md5", "[data]")
#define aneri_md5_file(file)	ANERI_CALL("md5_file", "[file]")

#define aneri_sha1(data)		ANERI_CALL("sha1", "[data]")
#define aneri_sha1_file(file)	ANERI_CALL("sha1_file", "[file]")

#define aneri_sha2_224(data)			ANERI_CALL("sha2_224", "[data]")
#define aneri_sha2_224_file(file)		ANERI_CALL("sha2_224_file", "[file]")
#define aneri_sha2_256(data)			ANERI_CALL("sha2_256", "[data]")
#define aneri_sha2_256_file(file)		ANERI_CALL("sha2_256_file", "[file]")
#define aneri_sha2_384(data)			ANERI_CALL("sha2_384", "[data]")
#define aneri_sha2_384_file(file)		ANERI_CALL("sha2_384_file", "[file]")
#define aneri_sha2_512(data)			ANERI_CALL("sha2_512", "[data]")
#define aneri_sha2_512_file(file)		ANERI_CALL("sha2_512_file", "[file]")
#define aneri_sha2_512_224(data)		ANERI_CALL("sha2_512_224", "[data]")
#define aneri_sha2_512_224_file(file)	ANERI_CALL("sha2_512_224_file", "[file]")
#define aneri_sha2_512_256(data)		ANERI_CALL("sha2_512_256", "[data]")
#define aneri_sha2_512_256_file(file)	ANERI_CALL("sha2_512_256_file", "[file]")

#define aneri_sha3_224(data)		ANERI_CALL("sha3_224", "[data]")
#define aneri_sha3_224_file(file)	ANERI_CALL("sha3_224_file", "[file]")
#define aneri_sha3_256(data)		ANERI_CALL("sha3_256", "[data]")
#define aneri_sha3_256_file(file)	ANERI_CALL("sha3_256_file", "[file]")
#define aneri_sha3_384(data)		ANERI_CALL("sha3_384", "[data]")
#define aneri_sha3_384_file(file)	ANERI_CALL("sha3_384_file", "[file]")
#define aneri_sha3_512(data)		ANERI_CALL("sha3_512", "[data]")
#define aneri_sha3_512_file(file)	ANERI_CALL("sha3_512_file", "[file]")

/proc/aneri_xxh32(data, seed = null)		return ANERI_CALL("xxh32", "[data]", seed)
/proc/aneri_xxh32_file(file, seed = null)	return ANERI_CALL("xxh32_file", "[file]", seed)
/proc/aneri_xxh64(data, seed = null)		return ANERI_CALL("xxh64", "[data]", seed)
/proc/aneri_xxh64_file(file, seed = null)	return ANERI_CALL("xxh64_file", "[file]", seed)
/proc/aneri_xxh3(data, seed = null)			return ANERI_CALL("xxh3", "[data]", seed)
/proc/aneri_xxh3_file(file, seed = null)	return ANERI_CALL("xxh3_file", "[file]", seed)

/proc/aneri_totp_check_sha1(token, secret, digits = null, skew = null, step_ = null)
	return ANERI_CALL("totp_check_sha1", token, secret, digits, skew, step_)

/proc/aneri_totp_check_sha256(token, secret, digits = null, skew = null, step_ = null)
	return ANERI_CALL("totp_check_sha256", token, secret, digits, skew, step_)

/proc/aneri_totp_check_sha512(token, secret, digits = null, skew = null, step_ = null)
	return ANERI_CALL("totp_check_sha512", token, secret, digits, skew, step_)

/proc/aneri_totp_gen_sha1(secret, digits = null, skew = null, step_ = null)
	return ANERI_CALL("totp_gen_sha1", secret, digits, skew, step_)

/proc/aneri_totp_gen_sha256(secret, digits = null, skew = null, step_ = null)
	return ANERI_CALL("totp_gen_sha256", secret, digits, skew, step_)

/proc/aneri_totp_gen_sha512(secret, digits = null, skew = null, step_ = null)
	return ANERI_CALL("totp_gen_sha512", secret, digits, skew, step_)
