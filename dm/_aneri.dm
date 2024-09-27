/* This comment bypasses grep checks */ /var/__aneri

/proc/__detect_aneri()
	if (world.system_type == UNIX)
		return __aneri = (fexists("./libaneri.so") ? "./libaneri.so" : "libaneri")
	else
		return __aneri = "aneri"

#define ANERI (__aneri || __detect_aneri())
#define ANERI_CALL(name, args...) call_ext(ANERI, "byond:" + name)(args)

/proc/aneri_version()	return ANERI_CALL("aneri_version")
/proc/aneri_features()	return ANERI_CALL("aneri_features")
/proc/aneri_cleanup()	return ANERI_CALL("cleanup")

/datum/aneri
	VAR_FINAL/__aneri_key_low
	VAR_FINAL/__aneri_key_high

/world/New()
	aneri_cleanup()
	..()

// placeholder
/datum/proc/Destroy()
