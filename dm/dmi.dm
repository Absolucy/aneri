#define ANERI_RESIZE_NEAREST	"nearest"
#define ANERI_RESIZE_CATMULL	"catmull"
#define ANERI_RESIZE_GAUSSIAN	"gaussian"
#define ANERI_RESIZE_LANCZOS3	"lanczos3"
#define ANERI_RESIZE_TRIANGLE	"triangle"

/proc/aneri_dmi_resize_png(path, width, height, resize_type = ANERI_RESIZE_NEAREST)
	return ANERI_CALL("dmi_resize_png", path, width, height, resize_type)

/proc/aneri_dmi_read_states(path) as /list
	RETURN_TYPE(/list)
	return ANERI_CALL("dmi_read_states", path)

#define TMP_UPSCALE_PATH "tmp/resize_icon.png"

/// Upscales an icon using aneri.
/// You really shouldn't use this TOO often, as it has to copy the icon to a temporary png file,
/// resize it, fcopy_rsc the resized png, and then create a new /icon from said png.
/// Cache the output where possible.
/proc/resize_icon(icon/icon, width, height, resize_type = ANERI_RESIZE_NEAREST) as /icon
	RETURN_TYPE(/icon)
	SHOULD_BE_PURE(TRUE)

	if(!istype(icon))
		CRASH("Attempted to upscale non-icon")
	if(!isnum(width) || !isnum(height))
		CRASH("Attempted to upscale icon to non-number width/height")
	if(!fcopy(icon, TMP_UPSCALE_PATH))
		CRASH("Failed to create temporary png file to upscale")
	UNLINT(aneri_dmi_resize_png(TMP_UPSCALE_PATH, width, height, resize_type)) // technically impure but in practice its not
	. = icon(fcopy_rsc(TMP_UPSCALE_PATH))
	fdel(TMP_UPSCALE_PATH)

#undef TMP_UPSCALE_PATH
