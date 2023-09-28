// SPDX-License-Identifier: MPL-2.0
use meowtonin::{ByondError, ByondResult, ByondValue, FromByond};
use mysql_async::{Params as SqlParams, Value as SqlValue};
use std::collections::HashMap;

pub(crate) fn value_to_sql(value: &ByondValue) -> ByondResult<SqlValue> {
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
