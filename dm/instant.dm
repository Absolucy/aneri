/datum/instant
	parent_type = /datum/aneri

/datum/instant/New(regex_value, modifiers)
	var/err = ANERI_CALL("instant_new", src)
	// returns null if everything was fine, else returns error info
	if(!isnull(err))
		//log_runtime("failed to create instant: [err]")
		CRASH("failed to create instant: [err]")

/datum/instant/Destroy()
	if(!ANERI_CALL("instant_del", src))
		//log_runtime("attempted to delete nonexistent instant")
		CRASH("attempted to delete nonexistent instant")
	return ..()

/datum/instant/proc/microseconds()
	return ANERI_CALL("instant_microseconds", src)

/datum/instant/proc/milliseconds()
	return ANERI_CALL("instant_milliseconds", src)

/datum/instant/proc/seconds()
	return ANERI_CALL("instant_seconds", src)
