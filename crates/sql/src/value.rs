// SPDX-License-Identifier: MPL-2.0
use crate::query::{ColumnKind, Failure};
use meowtonin::{ByondResult, ByondValue};
use mysql_async::Value;
use std::fmt::Write;

pub(crate) fn to_byond(value: Value, kind: ColumnKind) -> ByondResult<ByondValue> {
	Ok(match value {
		Value::NULL => ByondValue::NULL,
		Value::Int(num) => ByondValue::new_num(num as f32),
		Value::UInt(num) => ByondValue::new_num(num as f32),
		Value::Float(num) => ByondValue::new_num(num),
		Value::Double(num) => ByondValue::new_num(num as f32),
		Value::Date(year, month, day, hour, minute, second, micros) => {
			let mut text = format!("{year:04}-{month:02}-{day:02}");
			if !matches!(kind, ColumnKind::Date) {
				let _ = write!(text, " {hour:02}:{minute:02}:{second:02}");
				push_fraction(&mut text, micros);
			}
			ByondValue::new_string(text)
		}
		Value::Time(negative, days, hours, minutes, seconds, micros) => {
			let sign = if negative { "-" } else { "" };
			let hours = days * 24 + u32::from(hours);
			let mut text = format!("{sign}{hours:02}:{minutes:02}:{seconds:02}");
			push_fraction(&mut text, micros);
			ByondValue::new_string(text)
		}
		Value::Bytes(bytes) => bytes_to_byond(bytes, kind)?,
	})
}

fn push_fraction(text: &mut String, micros: u32) {
	if micros != 0 {
		let _ = write!(text, ".{micros:06}");
	}
}

fn bytes_to_byond(bytes: Vec<u8>, kind: ColumnKind) -> ByondResult<ByondValue> {
	match kind {
		ColumnKind::Bytes => {
			let mut list = ByondValue::new_list()?;
			list.write_list(bytes.into_iter().map(ByondValue::new_num))?;
			Ok(list)
		}
		ColumnKind::Decimal => match std::str::from_utf8(&bytes)
			.ok()
			.and_then(|text| text.parse::<f64>().ok())
		{
			Some(num) => Ok(ByondValue::new_num(num as f32)),
			None => Ok(text(bytes)),
		},
		ColumnKind::Bit => Ok(ByondValue::new_num(
			bytes
				.iter()
				.fold(0_u64, |acc, byte| (acc << 8) | u64::from(*byte)) as f32,
		)),
		ColumnKind::Text | ColumnKind::Date => Ok(text(bytes)),
	}
}

fn text(bytes: Vec<u8>) -> ByondValue {
	match String::from_utf8(bytes) {
		Ok(text) => ByondValue::new_string(text),
		Err(err) => ByondValue::new_string(String::from_utf8_lossy(err.as_bytes()).into_owned()),
	}
}

/// Turns one DM argument into a value the database can take.
pub(crate) fn from_byond(
	value: &ByondValue,
	what: impl FnOnce() -> String,
) -> Result<Value, Failure> {
	if value.is_null() {
		return Ok(Value::NULL);
	}
	if value.is_number() {
		let num = value
			.get_number()
			.map_err(|err| Failure::new(err.to_string()))?;
		return Ok(number(num));
	}
	if value.is_string() {
		let text = value
			.get_string()
			.map_err(|err| Failure::new(err.to_string()))?;
		return Ok(Value::Bytes(text.into_bytes()));
	}
	Err(Failure::new(format!(
		"{} is a {}, only null, text and numbers can be sent to the database",
		what(),
		value.get_type().name()
	)))
}

fn number(num: f32) -> Value {
	if num.fract() == 0.0 {
		return Value::Int(num as i64);
	}
	Value::Double(num.to_string().parse().unwrap_or(f64::from(num)))
}
