/* This comment bypasses grep checks */ /var/__aneri

/proc/__detect_aneri()
	if (world.system_type == UNIX)
		if (fexists("./libaneri.so"))
			// No need for LD_LIBRARY_PATH badness.
			return __aneri = "./libaneri.so"
		else if (fexists("./aneri"))
			// Old dumb filename.
			return __aneri = "./aneri"
		else if (fexists("[world.GetConfig("env", "HOME")]/.byond/bin/aneri"))
			// Old dumb filename in `~/.byond/bin`.
			return __aneri = "aneri"
		else
			// It's not in the current directory, so try others
			return __aneri = "libaneri.so"
	else
		return __aneri = "aneri"

#define ANERI (__aneri || __detect_aneri())
#define ANERI_CALL(name, args...) call_ext(ANERI, "byond:[name]")(args)

/proc/aneri_version()	return ANERI_CALL("aneri_version")
/proc/aneri_features()	return ANERI_CALL("aneri_features")
/proc/aneri_cleanup()	return ANERI_CALL("cleanup")

/datum/aneri
	var/__aneri_key_low
	var/__aneri_key_high

/world/New()
	aneri_cleanup()
	..()
