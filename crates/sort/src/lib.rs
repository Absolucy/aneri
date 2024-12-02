// SPDX-License-Identifier: MPL-2.0
use meowtonin::{byond_fn, ByondResult, ByondValue};
use std::cmp::Ordering;

fn reassemble_list(list: Vec<[ByondValue; 2]>) -> ByondResult<ByondValue> {
	let mut sorted_list = ByondValue::new_list()?;
	for [key, value] in list {
		sorted_list.write_list_index(key, value)?;
	}
	Ok(sorted_list)
}

fn assoc_a_b<'a, 'b>(
	[a_key, a_val]: &'a [ByondValue; 2],
	[b_key, b_val]: &'b [ByondValue; 2],
	associative: bool,
) -> (&'a ByondValue, &'b ByondValue) {
	if associative {
		(a_val, b_val)
	} else {
		(a_key, b_key)
	}
}

#[byond_fn]
pub fn sort_with_proc(
	list: ByondValue,
	proc_name: String,
	associative: Option<bool>,
) -> ByondResult<ByondValue> {
	let associative = associative.unwrap_or(false);
	let mut list = list.read_assoc_list()?;
	list.sort_by(|a, b| {
		let (a, b) = assoc_a_b(a, b, associative);
		match meowtonin::call_global::<_, _, _, Option<isize>>(&proc_name, [a, b])
			.expect("sort proc failed")
		{
			Some(ret) => ret.cmp(&0),
			None => Ordering::Equal,
		}
	});
	reassemble_list(list)
}

#[byond_fn]
pub fn sort_by_number(
	list: ByondValue,
	descending: Option<bool>,
	associative: Option<bool>,
) -> ByondResult<ByondValue> {
	let descending = descending.unwrap_or(false);
	let associative = associative.unwrap_or(false);
	let list = list.read_assoc_list()?;

	// Pre-convert to tuples of (original_pair, number)
	let mut converted: Vec<_> = list
		.iter()
		.map(|pair| {
			let value = if associative { &pair[1] } else { &pair[0] };
			(pair.clone(), value.get_number().unwrap_or(0.0))
		})
		.collect();

	// Sort using pre-converted values
	converted.sort_by(|&(_, a), &(_, b)| {
		let mut a = a;
		let mut b = b;
		if descending {
			std::mem::swap(&mut a, &mut b);
		}
		a.total_cmp(&b)
	});

	// Rebuild list from sorted pairs
	reassemble_list(converted.into_iter().map(|(pair, _)| pair).collect())
}

#[byond_fn]
pub fn sort_by_number_var(
	list: ByondValue,
	var: String,
	descending: Option<bool>,
	associative: Option<bool>,
) -> ByondResult<ByondValue> {
	let descending = descending.unwrap_or(false);
	let associative = associative.unwrap_or(false);
	let list = list.read_assoc_list()?;

	// Pre-convert to tuples of (original_pair, number)
	let mut converted: Vec<_> = list
		.iter()
		.map(|pair| -> ByondResult<_> {
			let value = if associative { &pair[1] } else { &pair[0] };
			Ok((pair.clone(), value.read_var::<_, f32>(&var).unwrap_or(0.0)))
		})
		.collect::<ByondResult<_>>()?;

	// Sort using pre-converted values
	converted.sort_by(|&(_, a), &(_, b)| {
		let mut a = a;
		let mut b = b;
		if descending {
			std::mem::swap(&mut a, &mut b);
		}
		a.total_cmp(&b)
	});

	// Rebuild list from sorted pairs
	reassemble_list(converted.into_iter().map(|(pair, _)| pair).collect())
}

#[byond_fn]
pub fn sort_by_string(
	list: ByondValue,
	descending: Option<bool>,
	ignore_case: Option<bool>,
	associative: Option<bool>,
) -> ByondResult<ByondValue> {
	let ignore_case = ignore_case.unwrap_or(false);
	let descending = descending.unwrap_or(false);
	let associative = associative.unwrap_or(false);
	let list = list.read_assoc_list()?;

	// Pre-convert to tuples of (original_pair, string)
	let mut converted: Vec<_> = list
		.iter()
		.map(|pair| {
			let value = if associative { &pair[1] } else { &pair[0] };
			let mut string = value.get_string().unwrap_or_default();
			if ignore_case {
				string = string.to_lowercase();
			}
			(pair.clone(), string)
		})
		.collect();

	// Sort using pre-converted values
	converted.sort_by(|(_, a), (_, b)| {
		let mut a = a;
		let mut b = b;
		if descending {
			std::mem::swap(&mut a, &mut b);
		}
		a.cmp(b)
	});

	// Rebuild list from sorted pairs
	reassemble_list(converted.into_iter().map(|(pair, _)| pair).collect())
}

#[byond_fn]
pub fn sort_by_string_var(
	list: ByondValue,
	var: String,
	descending: Option<bool>,
	ignore_case: Option<bool>,
	associative: Option<bool>,
) -> ByondResult<ByondValue> {
	let ignore_case = ignore_case.unwrap_or(false);
	let descending = descending.unwrap_or(false);
	let associative = associative.unwrap_or(false);
	let list = list.read_assoc_list()?;

	// Pre-convert to tuples of (original_pair, string)
	let mut converted: Vec<_> = list
		.iter()
		.map(|pair| -> ByondResult<_> {
			let value = if associative { &pair[1] } else { &pair[0] };
			let mut string = value.read_var::<_, String>(&var).unwrap_or_default();
			if ignore_case {
				string = string.to_lowercase();
			}
			Ok((pair.clone(), string))
		})
		.collect::<ByondResult<_>>()?;

	// Sort using pre-converted values
	converted.sort_by(|(_, a), (_, b)| {
		let mut a = a;
		let mut b = b;
		if descending {
			std::mem::swap(&mut a, &mut b);
		}
		a.cmp(b)
	});

	// Rebuild list from sorted pairs
	reassemble_list(converted.into_iter().map(|(pair, _)| pair).collect())
}
