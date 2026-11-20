// only aneri.dme includes this. a codebase vendoring dm/ calls aneri_cleanup() from its own world procs
// and already has its own Destroy()

/world/New()
	aneri_cleanup()
	..()

/world/Del()
	aneri_cleanup()
	..()

/datum/Del()
	Destroy()
	return ..()

/datum/proc/Destroy()
