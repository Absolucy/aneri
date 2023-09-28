// SPDX-License-Identifier: MPL-2.0
use aneri_core::key;
use meowtonin::{ByondError, ByondResult, ByondValue, FromByond, ToByond};
use mysql_async::{
	consts::{
		ColumnFlags,
		ColumnType::{self, *},
	},
	Column as SqlColumn, Params as SqlParams, Value as SqlValue,
};
use std::collections::HashMap;

pub(crate) fn byond_to_sql(value: &ByondValue) -> ByondResult<SqlValue> {
	if value.is_null() {
		Ok(SqlValue::NULL)
	} else if let Ok(value) = value.get_number() {
		if value.fract() <= f32::EPSILON {
			Ok(SqlValue::Int(value as i64))
		} else {
			Ok(SqlValue::Float(value))
		}
	} else if let Ok(value) = value.read_list() {
		let bytes = value
			.into_iter()
			.map(|value| u8::from_byond(&value))
			.collect::<ByondResult<Vec<_>>>()?;
		Ok(SqlValue::Bytes(bytes))
	} else if let Ok(value) = value.get_string() {
		Ok(SqlValue::Bytes(value.into_bytes()))
	} else {
		Ok(SqlValue::NULL)
	}
}

pub(crate) fn byond_to_params(value: &ByondValue) -> ByondResult<SqlParams> {
	let list = value.read_assoc_list()?;
	let mut params = HashMap::<Vec<u8>, SqlValue>::with_capacity(list.len());
	for [key, value] in list {
		let key = key.get_string()?.into_bytes();
		let value = byond_to_sql(&value)?;
		params.insert(key, value);
	}
	Ok(SqlParams::Named(params))
}
