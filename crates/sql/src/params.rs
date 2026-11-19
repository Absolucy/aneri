// SPDX-License-Identifier: MPL-2.0
use crate::{query::Failure, value};
use meowtonin::ByondValue;
use mysql_async::Value;
use mysql_common::named_params::ParsedNamedParams;
use std::borrow::Cow;

/// Turns `:name` placeholders into `?` and puts the arguments in the order
/// the statement needs them.
pub(crate) fn bind(sql: String, arguments: &ByondValue) -> Result<(String, Vec<Value>), Failure> {
	let parsed = ParsedNamedParams::parse(sql.as_bytes())
		.map_err(|_| Failure::new("the query mixes ? and :name placeholders, use only one kind"))?;
	if parsed.params().is_empty() {
		let values = positional(arguments)?;
		return Ok((sql, values));
	}
	let values = named(parsed.params(), arguments)?;
	let query = String::from_utf8_lossy(parsed.query()).into_owned();
	Ok((query, values))
}

fn positional(arguments: &ByondValue) -> Result<Vec<Value>, Failure> {
	if arguments.is_null() {
		return Ok(Vec::new());
	}
	if !arguments.is_list() {
		return Err(Failure::new("arguments must be a list"));
	}
	if arguments.is_likely_assoc().unwrap_or(false) {
		return Err(Failure::new(
			"the query uses ? placeholders, so arguments must be a plain list, not an assoc list",
		));
	}
	arguments
		.read_list()
		.map_err(|err| Failure::new(err.to_string()))?
		.iter()
		.enumerate()
		.map(|(index, argument)| value::from_byond(argument, || format!("argument {}", index + 1)))
		.collect()
}

fn named(names: &[Cow<'_, [u8]>], arguments: &ByondValue) -> Result<Vec<Value>, Failure> {
	let pairs = if arguments.is_list() {
		arguments
			.read_assoc_list()
			.map_err(|err| Failure::new(err.to_string()))?
	} else {
		Vec::new()
	};
	let pairs = pairs
		.into_iter()
		.filter_map(|[key, value]| Some((key.get_string().ok()?, value)))
		.collect::<Vec<_>>();
	names
		.iter()
		.map(|name| {
			let name = String::from_utf8_lossy(name);
			let Some((_, argument)) = pairs.iter().find(|(key, _)| *key == name) else {
				return Err(Failure::new(format!("no value for :{name}")));
			};
			value::from_byond(argument, || format!("argument :{name}"))
		})
		.collect()
}
