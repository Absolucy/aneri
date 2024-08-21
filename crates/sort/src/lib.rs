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
pub fn sort_by_number(list: Vec<ByondValue>, _ascending: Option<bool>) -> Vec<f32> {
	let mut list = list
		.into_iter()
		.flat_map(|value| value.get_number().ok())
		.collect::<Vec<f32>>();
	let original_len = list.len();
	glidesort::sort_in_vec_by(&mut list, |a, b| a.total_cmp(b));
	list.truncate(original_len);
	list
}

#[byond_fn]
pub fn sort_by_number_var(
	mut list: Vec<ByondValue>,
	var: String,
	_ascending: Option<bool>,
) -> Vec<ByondValue> {
	let original_len = list.len();
	glidesort::sort_in_vec_by(&mut list, |a, b| {
		let a = a.read_var::<_, f32>(&var);
		let b = b.read_var::<_, f32>(&var);
		match (a, b) {
			(Ok(a), Ok(b)) => a.total_cmp(&b),
			(Ok(_), Err(_)) => Ordering::Greater,
			(Err(_), Ok(_)) => Ordering::Less,
			_ => Ordering::Equal,
		}
	});
	list.truncate(original_len);
	list
}

#[byond_fn]
pub fn sort_by_string(
	mut list: Vec<String>,
	_ascending: Option<bool>,
	ignore_case: Option<bool>,
) -> Vec<String> {
	let ignore_case = ignore_case.unwrap_or(false);
	let original_len = list.len();
	glidesort::sort_in_vec_by(&mut list, |a, b| {
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
