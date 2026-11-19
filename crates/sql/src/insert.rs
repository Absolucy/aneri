// SPDX-License-Identifier: MPL-2.0
use crate::{query::Failure, value};
use meowtonin::ByondValue;
use mysql_async::Value;

const MAX_PLACEHOLDERS: usize = u16::MAX as usize;

struct Column {
	name: String,
	expression: Option<String>,
}

impl Column {
	fn placeholders(&self) -> usize {
		match &self.expression {
			Some(expression) => expression.matches('?').count(),
			None => 1,
		}
	}
}

/// Builds one `INSERT` statement for every row, the same way tgstation's
/// `SSdbcore.MassInsert` does. Returns empty SQL when there are no rows.
pub(crate) fn build(
	table: &str,
	rows: &ByondValue,
	duplicate_key: &ByondValue,
	ignore_errors: bool,
	special_columns: Vec<(String, String)>,
) -> Result<(String, Vec<Value>), Failure> {
	let fail = |err: meowtonin::ByondError| Failure::new(err.to_string());
	if !rows.is_list() {
		return Err(Failure::new("rows must be a list of assoc lists"));
	}
	let mut columns = Vec::<Column>::new();
	let rows = rows
		.read_list()
		.map_err(fail)?
		.iter()
		.map(|row| {
			if !row.is_list() {
				return Err(Failure::new("rows must be a list of assoc lists"));
			}
			let cells = row
				.read_assoc_list()
				.map_err(fail)?
				.into_iter()
				.filter_map(|[key, value]| Some((key.get_string().ok()?, value)))
				.map(|(name, value)| (column_index(&mut columns, name), value))
				.collect::<Vec<_>>();
			Ok(cells)
		})
		.collect::<Result<Vec<_>, Failure>>()?;
	if rows.is_empty() {
		return Ok((String::new(), Vec::new()));
	}
	for (name, expression) in special_columns {
		let index = column_index(&mut columns, name);
		columns[index].expression = Some(expression);
	}

	let per_row = columns.iter().map(Column::placeholders).sum::<usize>();
	let total = per_row * rows.len();
	if total > MAX_PLACEHOLDERS {
		return Err(Failure::new(format!(
			"this insert needs {total} values ({} rows of {per_row}), but one statement can take \
			 at most {MAX_PLACEHOLDERS}; split the rows",
			rows.len()
		)));
	}

	let row_sql = row_template(&columns);
	let mut sql = String::with_capacity(64 + table.len() + (row_sql.len() + 2) * rows.len());
	sql.push_str(if ignore_errors {
		"INSERT IGNORE INTO "
	} else {
		"INSERT INTO "
	});
	sql.push_str(table);
	sql.push_str(" (");
	for (index, column) in columns.iter().enumerate() {
		if index > 0 {
			sql.push_str(", ");
		}
		push_quoted(&mut sql, &column.name);
	}
	sql.push_str(") VALUES ");
	for index in 0..rows.len() {
		if index > 0 {
			sql.push_str(", ");
		}
		sql.push_str(&row_sql);
	}
	push_duplicate_key(&mut sql, &columns, duplicate_key)?;

	let mut values = Vec::with_capacity(total);
	for (row_index, cells) in rows.iter().enumerate() {
		for (column_index, column) in columns.iter().enumerate() {
			let count = column.placeholders();
			if count == 0 {
				continue;
			}
			let value = match cells.iter().find(|(index, _)| *index == column_index) {
				Some((_, cell)) => value::from_byond(cell, || {
					format!("column {} in row {}", column.name, row_index + 1)
				})?,
				None => Value::NULL,
			};
			values.extend(std::iter::repeat_n(value, count));
		}
	}
	Ok((sql, values))
}

fn column_index(columns: &mut Vec<Column>, name: String) -> usize {
	if let Some(index) = columns.iter().position(|column| column.name == name) {
		return index;
	}
	columns.push(Column {
		name,
		expression: None,
	});
	columns.len() - 1
}

fn row_template(columns: &[Column]) -> String {
	let mut row = String::from("(");
	for (index, column) in columns.iter().enumerate() {
		if index > 0 {
			row.push_str(", ");
		}
		row.push_str(column.expression.as_deref().unwrap_or("?"));
	}
	row.push(')');
	row
}

fn push_quoted(sql: &mut String, name: &str) {
	sql.push('`');
	sql.push_str(&name.replace('`', "``"));
	sql.push('`');
}

fn push_duplicate_key(
	sql: &mut String,
	columns: &[Column],
	duplicate_key: &ByondValue,
) -> Result<(), Failure> {
	if duplicate_key.is_string() {
		let text = duplicate_key
			.get_string()
			.map_err(|err| Failure::new(err.to_string()))?;
		sql.push_str(&text);
		return Ok(());
	}
	if !duplicate_key.is_true() {
		return Ok(());
	}
	sql.push_str(" ON DUPLICATE KEY UPDATE ");
	for (index, column) in columns.iter().enumerate() {
		if index > 0 {
			sql.push_str(", ");
		}
		push_quoted(sql, &column.name);
		sql.push_str(" = VALUES(");
		push_quoted(sql, &column.name);
		sql.push(')');
	}
	Ok(())
}
