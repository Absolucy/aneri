/var/__aneri

/proc/__detect_aneri()
	if (world.system_type == UNIX)
		__aneri = "libaneri"
	else
		__aneri = "aneri"
	return __aneri

#define ANERI (__aneri || __detect_aneri())
#define ANERI_CALL(name, args...) call_ext(ANERI, "byond:[name]")(args)
//#define iserror(v) (istype(v, /datum/ext_error))

/proc/aneri_version()	return ANERI_CALL("aneri_version")
/proc/aneri_features()	return ANERI_CALL("aneri_features")
/proc/aneri_cleanup()	return ANERI_CALL("cleanup")

/datum/aneri
	var/__aneri_key_low
	var/__aneri_key_high

/world/New()
	ANERI_CALL("cleanup")
	..()
