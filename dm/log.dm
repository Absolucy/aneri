#define aneri_log(fname, text)	ANERI_CALL("log_write", "[fname]", text)
/proc/aneri_log_close_all()		return ANERI_CALL("log_close_all")
