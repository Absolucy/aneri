// SPDX-License-Identifier: MPL-2.0
#[macro_use]
extern crate meowtonin;

use meowtonin::{ByondResult, ByondValue};
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
pub fn sort_by_number(list: Vec<ByondValue>, descending: Option<bool>) -> Vec<f32> {
	let mut list = list
		.into_iter()
		.flat_map(|value| value.get_number().ok())
		.collect::<Vec<f32>>();
	let descending = descending.unwrap_or(false);
	list.sort_by(|a, b| {
		let mut a = a;
		let mut b = b;
		if descending {
			std::mem::swap(&mut a, &mut b);
		}
		a.total_cmp(b)
	});
	list
}

#[byond_fn]
pub fn sort_by_number_var(
	list: Vec<ByondValue>,
	var: String,
	descending: Option<bool>,
) -> ByondResult<Vec<ByondValue>> {
	let descending = descending.unwrap_or(false);
	let mut list = list
		.into_iter()
		.map(|value| -> ByondResult<(ByondValue, f32)> {
			let num = value.read_var::<_, f32>(&var)?;
			Ok((value, num))
		})
		.collect::<ByondResult<Vec<(ByondValue, f32)>>>()?;
	list.sort_by(|&(_, a), &(_, b)| {
		let mut a = a;
		let mut b = b;
		if descending {
			std::mem::swap(&mut a, &mut b);
		}
		a.total_cmp(&b)
	});
	Ok(list.into_iter().map(|(value, _)| value).collect())
}

#[byond_fn]
pub fn sort_by_string(
	mut list: Vec<String>,
	descending: Option<bool>,
	ignore_case: Option<bool>,
) -> Vec<String> {
	let ignore_case = ignore_case.unwrap_or(false);
	let descending = descending.unwrap_or(false);
	list.sort_by(|a, b| {
		let mut a = a;
		let mut b = b;
		if descending {
			std::mem::swap(&mut a, &mut b);
		}
		if ignore_case {
			let a = a.to_lowercase();
			let b = b.to_lowercase();
			a.cmp(&b)
		} else {
			a.cmp(b)
		}
	});
	list
}

#[byond_fn]
pub fn sort_by_string_var(
	list: Vec<ByondValue>,
	var: String,
	descending: Option<bool>,
	ignore_case: Option<bool>,
) -> ByondResult<Vec<ByondValue>> {
	let ignore_case = ignore_case.unwrap_or(false);
	let descending = descending.unwrap_or(false);
	let mut list = list
		.into_iter()
		.map(|value| -> ByondResult<(ByondValue, String)> {
			let mut string = value.read_var::<_, String>(&var)?;
			if ignore_case {
				string = string.to_lowercase();
			}
			Ok((value, string))
		})
		.collect::<ByondResult<Vec<(ByondValue, String)>>>()?;
	list.sort_by(|(_, a), (_, b)| {
		let mut a = a;
		let mut b = b;
		if descending {
			std::mem::swap(&mut a, &mut b);
		}
		a.cmp(b)
	});
	Ok(list.into_iter().map(|(value, _)| value).collect())
}
