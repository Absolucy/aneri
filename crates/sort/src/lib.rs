// SPDX-License-Identifier: MPL-2.0
use meowtonin::{ByondResult, ByondValue, ByondValueType, byond_fn};
use std::cmp::{Ordering, Reverse};

type Pair = (ByondValue, ByondValue);

enum SortKey {
	Number(f32),
	Text(String),
}

impl SortKey {
	fn compare(&self, other: &Self) -> Ordering {
		match (self, other) {
			(Self::Number(a), Self::Number(b)) => a.total_cmp(b),
			(Self::Text(a), Self::Text(b)) => a.cmp(b),
			(Self::Number(_), Self::Text(_)) => Ordering::Less,
			(Self::Text(_), Self::Number(_)) => Ordering::Greater,
		}
	}
}

fn read_pairs(list: &ByondValue) -> ByondResult<Vec<Pair>> {
	if list.get_type() == ByondValueType::List {
		return Ok(list.iter()?.collect());
	}
	Ok(list.values()?.map(|key| (key, ByondValue::NULL)).collect())
}

fn write_pairs(pairs: Vec<Pair>) -> ByondResult<ByondValue> {
	let mut list = ByondValue::new_list()?;
	list.write_list(pairs.iter().map(|(key, _)| key.clone()))?;
	for (key, value) in pairs.iter().filter(|(_, value)| !value.is_null()) {
		list.write_list_index(key, value)?;
	}
	Ok(list)
}

fn picked(pair: &Pair, associative: bool) -> &ByondValue {
	if associative { &pair.1 } else { &pair.0 }
}

/// Sorts a list using a global proc to compare items.
/// Keeps each item's associated value attached to it.
/// Runtimes if the proc fails, and may runtime if the proc gives
/// answers that contradict each other (a before b, but also b before a).
#[byond_fn(name = "aneri_raw_sort_with_proc")]
pub fn sort_with_proc(
	list: ByondValue,
	/// Gets two items and returns a negative number if the first goes first,
	/// a positive number if the second goes first, or 0 (or null) if they're equal.
	proc_name: String,
	/// Compare the associated values instead of the items themselves, default false.
	/// Items with no associated value get null.
	associative: Option<bool>,
) -> ByondResult<ByondValue> {
	let associative = associative.unwrap_or(false);
	let proc_name = proc_name.strip_prefix("/proc/").unwrap_or(&proc_name);
	let mut pairs = read_pairs(&list)?;
	pairs.sort_by(|a, b| {
		let args = [picked(a, associative), picked(b, associative)];
		meowtonin::call_global::<_, _, _, Option<f32>>(proc_name, args)
			.expect("sort proc failed")
			.and_then(|ret| ret.partial_cmp(&0.0))
			.unwrap_or(Ordering::Equal)
	});
	write_pairs(pairs)
}

/// Sorts a list using a global proc that gets called once per item
/// to say what that item sorts by. Much fewer calls into DM than `sort_with_proc`.
/// Keeps each item's associated value attached to it.
/// Numbers sort before text. Runtimes if the proc fails or returns anything else.
#[byond_fn(name = "aneri_raw_sort_by_key_proc")]
pub fn sort_by_key_proc(
	list: ByondValue,
	/// Gets one item and returns a number or text to sort it by.
	proc_name: String,
	/// Biggest first instead, default false.
	descending: Option<bool>,
	/// Sort by the associated values instead of the items themselves, default false.
	/// Items with no associated value get null, which runtimes.
	associative: Option<bool>,
) -> ByondResult<ByondValue> {
	let descending = descending.unwrap_or(false);
	let associative = associative.unwrap_or(false);
	let proc_name = proc_name.strip_prefix("/proc/").unwrap_or(&proc_name);
	let mut keyed = read_pairs(&list)?
		.into_iter()
		.map(|pair| {
			let key = meowtonin::call_global::<_, _, _, ByondValue>(proc_name, [picked(
				&pair,
				associative,
			)])
			.expect("key proc failed");
			let key = match key.get_type() {
				ByondValueType::Number => {
					SortKey::Number(key.get_number().expect("key proc number didn't read"))
				}
				ByondValueType::String => {
					SortKey::Text(key.get_string().expect("key proc text didn't read"))
				}
				other => panic!("key proc returned {}, not a number or text", other.name()),
			};
			(pair, key)
		})
		.collect::<Vec<_>>();
	keyed.sort_by(|(_, a), (_, b)| {
		if descending {
			b.compare(a)
		} else {
			a.compare(b)
		}
	});
	write_pairs(keyed.into_iter().map(|(pair, _)| pair).collect())
}

/// Sorts the numbers in a list, smallest first.
/// Anything that isn't a number is dropped from the result.
/// Keeps each item's associated value attached to it.
#[byond_fn(name = "aneri_raw_sort_by_number")]
pub fn sort_by_number(
	list: ByondValue,
	/// Biggest first instead, default false.
	descending: Option<bool>,
	/// Sort by the associated values instead of the items themselves, default false.
	/// Items whose associated value isn't a number are dropped, and a plain list has none.
	associative: Option<bool>,
) -> ByondResult<ByondValue> {
	let descending = descending.unwrap_or(false);
	let associative = associative.unwrap_or(false);
	let mut numbered = read_pairs(&list)?
		.into_iter()
		.filter_map(|pair| {
			let number = picked(&pair, associative).get_number().ok()?;
			Some((pair, number))
		})
		.collect::<Vec<_>>();
	numbered.sort_by(|(_, a), (_, b)| {
		if descending {
			b.total_cmp(a)
		} else {
			a.total_cmp(b)
		}
	});
	write_pairs(numbered.into_iter().map(|(pair, _)| pair).collect())
}

/// Sorts a list of datums by a number var on each one, smallest first.
/// Keeps each item's associated value attached to it.
/// Runtimes if an item doesn't have the var or it isn't a number.
#[byond_fn(name = "aneri_raw_sort_by_number_var")]
pub fn sort_by_number_var(
	list: ByondValue,
	var: String,
	/// Biggest first instead, default false.
	descending: Option<bool>,
	/// Read the var off the associated values instead of the items themselves, default false.
	/// Items with no associated value get null, which runtimes.
	associative: Option<bool>,
) -> ByondResult<ByondValue> {
	let descending = descending.unwrap_or(false);
	let associative = associative.unwrap_or(false);
	let mut numbered = read_pairs(&list)?
		.into_iter()
		.map(|pair| {
			let var_value = picked(&pair, associative)
				.read_var::<_, ByondValue>(&var)
				.expect("couldn't read sort var");
			let ByondValueType::Number = var_value.get_type() else {
				panic!("sort var is {}, not a number", var_value.get_type().name());
			};
			let num = var_value.get_number().expect("sort var number didn't read");
			(pair, num)
		})
		.collect::<Vec<_>>();
	numbered.sort_by(|(_, a), (_, b)| {
		if descending {
			b.total_cmp(a)
		} else {
			a.total_cmp(b)
		}
	});
	write_pairs(numbered.into_iter().map(|(pair, _)| pair).collect())
}

/// Sorts a list of strings by byte value,
/// so `B` comes before `a` unless `ignore_case` is set.
/// Keeps each item's associated value attached to it.
/// Runtimes if an item isn't a string.
#[byond_fn(name = "aneri_raw_sort_by_string")]
pub fn sort_by_string(
	list: ByondValue,
	/// Reverse order instead, default false.
	descending: Option<bool>,
	/// Compare lowercased copies, default false.
	ignore_case: Option<bool>,
	/// Sort by the associated values instead of the items themselves, default false.
	/// Items with no associated value get null, which runtimes.
	associative: Option<bool>,
) -> ByondResult<ByondValue> {
	let ignore_case = ignore_case.unwrap_or(false);
	let associative = associative.unwrap_or(false);
	let key = |pair: &Pair| {
		let item = picked(pair, associative);
		let ByondValueType::String = item.get_type() else {
			panic!("sort item is {}, not text", item.get_type().name());
		};
		let string = item.get_string().expect("sort item text didn't read");
		if ignore_case {
			string.to_lowercase()
		} else {
			string
		}
	};
	let mut pairs = read_pairs(&list)?;
	if descending.unwrap_or(false) {
		pairs.sort_by_cached_key(|pair| Reverse(key(pair)));
	} else {
		pairs.sort_by_cached_key(key);
	}
	write_pairs(pairs)
}

/// Sorts a list of datums by a text var on each one, like `aneri_sort_by_string`.
/// Keeps each item's associated value attached to it.
/// Runtimes if an item doesn't have the var or it isn't text.
#[byond_fn(name = "aneri_raw_sort_by_string_var")]
pub fn sort_by_string_var(
	list: ByondValue,
	var: String,
	/// Reverse order instead, default false.
	descending: Option<bool>,
	/// Compare lowercased copies, default false.
	ignore_case: Option<bool>,
	/// Read the var off the associated values instead of the items themselves, default false.
	/// Items with no associated value get null, which runtimes.
	associative: Option<bool>,
) -> ByondResult<ByondValue> {
	let ignore_case = ignore_case.unwrap_or(false);
	let associative = associative.unwrap_or(false);
	let key = |pair: &Pair| {
		let var_value = picked(pair, associative)
			.read_var::<_, ByondValue>(&var)
			.expect("couldn't read sort var");
		let ByondValueType::String = var_value.get_type() else {
			panic!("sort var is {}, not text", var_value.get_type().name());
		};
		let string = var_value.get_string().expect("sort var text didn't read");
		if ignore_case {
			string.to_lowercase()
		} else {
			string
		}
	};
	let mut pairs = read_pairs(&list)?;
	if descending.unwrap_or(false) {
		pairs.sort_by_cached_key(|pair| Reverse(key(pair)));
	} else {
		pairs.sort_by_cached_key(key);
	}
	write_pairs(pairs)
}
