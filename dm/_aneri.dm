/* This comment bypasses grep checks */ /var/__aneri

/proc/__detect_aneri()
	if (world.system_type == UNIX)
		return __aneri = (fexists("./libaneri.so") ? "./libaneri.so" : "libaneri")
	else
		return __aneri = "aneri"

#define ANERI (__aneri || __detect_aneri())
#define ANERI_CALL(name, args...) call_ext(__aneri_##name ||= load_ext(ANERI, "byond:" + #name))(args)

/proc/aneri_cleanup()	return call_ext(ANERI, "byond:cleanup")()

/proc/aneri_version()	return call_ext(ANERI, "byond:aneri_version")()

/proc/aneri_features()	return call_ext(ANERI, "byond:aneri_features")()

/proc/aneri_set_panic_output_folder(path)	return call_ext(ANERI, "byond:set_panic_output_folder")(path)

/datum/aneri
	VAR_FINAL/__aneri_key_low
	VAR_FINAL/__aneri_key_high

/world/New()
	aneri_cleanup()
	..()

/world/Del()
	aneri_cleanup()
	..()

// placeholder
/datum/Del()
	Destroy()
	return ..()

/datum/proc/Destroy()
