// SPDX-License-Identifier: MPL-2.0
#[macro_use]
extern crate meowtonin;

use meowtonin::ByondValue;
use std::cmp::Ordering;

#[byond_fn]
pub fn sort(mut list: Vec<ByondValue>, proc_name: String) -> Vec<ByondValue> {
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
