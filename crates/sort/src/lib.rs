// SPDX-License-Identifier: MPL-2.0
#[macro_use]
extern crate meowtonin;

use meowtonin::ByondValue;
use std::cmp::Ordering;

#[byond_fn]
pub fn sort_with_proc(mut list: Vec<ByondValue>, proc_name: String) -> Vec<ByondValue> {
	let original_len = list.len();
	glidesort::sort_in_vec_by(&mut list, |a, b| {
		match meowtonin::call_global::<_, _, _, Option<isize>>(&proc_name, [a, b])
			.expect("sort proc failed")
		{
			Some(ret) => ret.cmp(&0),
			None => Ordering::Equal,
		}
	});
	list.truncate(original_len);
	list
}

#[byond_fn]
pub fn sort_by_number(list: Vec<ByondValue>, descending: Option<bool>) -> Vec<f32> {
	let mut list = list
		.into_iter()
		.flat_map(|value| value.get_number().ok())
		.collect::<Vec<f32>>();
	let original_len = list.len();
	let descending = descending.unwrap_or(false);
	glidesort::sort_in_vec_by(&mut list, |a, b| {
		let mut a = a;
		let mut b = b;
		if descending {
			std::mem::swap(&mut a, &mut b);
		}
		a.total_cmp(b)
	});
	list.truncate(original_len);
	list
}

#[byond_fn]
pub fn sort_by_number_var(
	mut list: Vec<ByondValue>,
	var: String,
	descending: Option<bool>,
) -> Vec<ByondValue> {
	let original_len = list.len();
	let descending = descending.unwrap_or(false);
	glidesort::sort_in_vec_by(&mut list, |a, b| {
		let mut a = a;
		let mut b = b;
		if descending {
			std::mem::swap(&mut a, &mut b);
		}
		let a = a.read_var::<_, f32>(&var).unwrap_or_else(|err| {
			panic!("Failed to read var '{var}' in sort_by_number_var: {err}")
		});
		let b = b.read_var::<_, f32>(&var).unwrap_or_else(|err| {
			panic!("Failed to read var '{var}' in sort_by_number_var: {err}")
		});
		a.total_cmp(&b)
	});
	list.truncate(original_len);
	list
}

#[byond_fn]
pub fn sort_by_string(
	mut list: Vec<String>,
	descending: Option<bool>,
	ignore_case: Option<bool>,
) -> Vec<String> {
	let ignore_case = ignore_case.unwrap_or(false);
	let original_len = list.len();
	let descending = descending.unwrap_or(false);
	glidesort::sort_in_vec_by(&mut list, |a, b| {
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
	list.truncate(original_len);
	list
}

#[byond_fn]
pub fn sort_by_string_var(
	mut list: Vec<ByondValue>,
	var: String,
	descending: Option<bool>,
	ignore_case: Option<bool>,
) -> Vec<ByondValue> {
	let ignore_case = ignore_case.unwrap_or(false);
	let original_len = list.len();
	let descending = descending.unwrap_or(false);
	glidesort::sort_in_vec_by(&mut list, |a, b| {
		let mut a = a;
		let mut b = b;
		if descending {
			std::mem::swap(&mut a, &mut b);
		}
		let mut a = a.read_var::<_, String>(&var).unwrap_or_else(|err| {
			panic!("Failed to read var '{var}' in sort_by_string_var: {err}")
		});
		let mut b = b.read_var::<_, String>(&var).unwrap_or_else(|err| {
			panic!("Failed to read var '{var}' in sort_by_string_var: {err}")
		});
		if ignore_case {
			a = a.to_lowercase();
			b = b.to_lowercase();
		}
		a.cmp(&b)
	});
	list.truncate(original_len);
	list
}
