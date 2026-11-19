/* This comment bypasses grep checks */ var/static/__aneri_sort_with_proc
/* This comment bypasses grep checks */ var/static/__aneri_sort_by_key_proc
/* This comment bypasses grep checks */ var/static/__aneri_sort_by_number
/* This comment bypasses grep checks */ var/static/__aneri_sort_by_number_var
/* This comment bypasses grep checks */ var/static/__aneri_sort_by_string
/* This comment bypasses grep checks */ var/static/__aneri_sort_by_string_var

/// Returns a sorted copy of `list_to_sort`, using the global proc `cmp` to compare items.
/// `cmp` gets two items and returns a negative number if the first goes first,
/// a positive number if the second goes first, or 0 if they're equal.
/// Associated values stay with their items. With `associative`, `cmp` gets the associated values instead.
/proc/aneri_sort_with_proc(list/list_to_sort, cmp, associative = FALSE) as /list
	RETURN_TYPE(list_to_sort[_].type)
	return ANERI_CALL(sort_with_proc, list_to_sort, "[cmp]", associative)

/// Returns a sorted copy of `list_to_sort`, using the global proc `key_proc` to say what each item sorts by.
/// `key_proc` gets one item and returns a number or text. Numbers sort before text.
/// Calls `key_proc` once per item, so it's much faster than aneri_sort_with_proc() on big lists.
/// Associated values stay with their items. With `associative`, `key_proc` gets the associated values instead.
/proc/aneri_sort_by_key_proc(list/list_to_sort, key_proc, descending = FALSE, associative = FALSE) as /list
	RETURN_TYPE(list_to_sort[_].type)
	return ANERI_CALL(sort_by_key_proc, list_to_sort, "[key_proc]", descending, associative)

/// Returns the numbers in `list_to_sort`, sorted smallest first.
/// Anything that isn't a number is dropped.
/// Associated values stay with their items. With `associative`, sorts by the associated values instead.
/proc/aneri_sort_by_number(list/list_to_sort, descending = FALSE, associative = FALSE) as /list
	RETURN_TYPE(list_to_sort[_].type)
	return ANERI_CALL(sort_by_number, list_to_sort, descending, associative)

/// Returns a copy of a list of datums, sorted by the number var `var_name`, smallest first.
/// Associated values stay with their items. With `associative`, reads the var off the associated values instead.
/proc/aneri_sort_by_number_var(list/list_to_sort, var_name, descending = FALSE, associative = FALSE) as /list
	RETURN_TYPE(list_to_sort[_].type)
	return ANERI_CALL(sort_by_number_var, list_to_sort, var_name, descending, associative)

/// Returns a sorted copy of a list of strings.
/// Sorts by byte value, so `B` comes before `a` unless `ignore_case` is set.
/// Associated values stay with their items. With `associative`, sorts by the associated values instead.
/proc/aneri_sort_by_string(list/list_to_sort, descending = FALSE, ignore_case = FALSE, associative = FALSE) as /list
	RETURN_TYPE(list_to_sort[_].type)
	return ANERI_CALL(sort_by_string, list_to_sort, descending, ignore_case, associative)

/// Returns a copy of a list of datums, sorted by the text var `var_name`, like aneri_sort_by_string().
/// Associated values stay with their items. With `associative`, reads the var off the associated values instead.
/proc/aneri_sort_by_string_var(list/list_to_sort, var_name, descending = FALSE, ignore_case = FALSE, associative = FALSE) as /list
	RETURN_TYPE(list_to_sort[_].type)
	return ANERI_CALL(sort_by_string_var, list_to_sort, var_name, descending, ignore_case, associative)
