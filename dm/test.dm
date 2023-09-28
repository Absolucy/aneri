/world/New()
	ANERI_CALL("test_callback", "https://catfact.ninja/fact", "http_callback")
	sleep(50)

/proc/http_callback(status, resp)
	world.log << "http response (status: [status])\n[resp]"