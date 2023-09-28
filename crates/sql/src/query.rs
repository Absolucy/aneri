// SPDX-License-Identifier: MPL-2.0
use aneri_core::{tokio::task::spawn_blocking, ByondSlotKey, RUNTIME};
use meowtonin::{ByondError, ByondResult, ByondValue, ToByond};
use mysql_async::{
	consts::{
		ColumnFlags as SqlColumnFlags,
		ColumnType::{self as SqlColumnType, *},
	},
	prelude::Queryable,
	Column as SqlColumn, Conn as SqlConn, Params as SqlParams, Row as SqlRow, Value as SqlValue,
};

#[derive(Debug)]
pub(crate) struct RowPart {
	pub column: SqlColumn,
	pub value: SqlValue,
	pub ctype: SqlColumnType,
}

impl ToByond for RowPart {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		match &self.value {
			SqlValue::Bytes(b) => match self.ctype {
				MYSQL_TYPE_VARCHAR | MYSQL_TYPE_STRING | MYSQL_TYPE_VAR_STRING => {
					String::from_utf8_lossy(b).to_byond()
				}
				MYSQL_TYPE_BLOB
				| MYSQL_TYPE_LONG_BLOB
				| MYSQL_TYPE_MEDIUM_BLOB
				| MYSQL_TYPE_TINY_BLOB => {
					if self.column.flags().contains(SqlColumnFlags::BINARY_FLAG) {
						b.to_byond()
					} else {
						String::from_utf8_lossy(b).to_byond()
					}
				}
				_ => Ok(ByondValue::null()),
			},
			SqlValue::Float(f) => f.to_byond(),
			SqlValue::Double(f) => (*f as f32).to_byond(),
			SqlValue::Int(i) => (*i as i32).to_byond(),
			SqlValue::UInt(u) => (*u as u32).to_byond(),
			SqlValue::Date(year, month, day, hour, minute, second, _ms) => {
				format!("{year}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02}").to_byond()
			}
			_ => Ok(ByondValue::null()),
		}
	}
}

pub(crate) struct QueryResult {
	pub affected: usize,
	pub last_insert_id: Option<String>,
	pub rows: Vec<Vec<RowPart>>,
}

impl ToByond for QueryResult {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		let mut list = ByondValue::new_list()?;
		list.write_list_index("status", "ok")?;
		list.write_list_index("affected", self.affected)?;
		list.write_list_index("last_insert_id", self.last_insert_id.to_byond()?)?;
		list.write_list_index("rows", self.rows.to_byond()?)?;
		Ok(list)
	}
}

pub(crate) async fn do_query(
	mut conn: SqlConn,
	query: String,
	params: SqlParams,
) -> Result<QueryResult, Box<dyn std::error::Error>> {
	let query_result = conn.exec_iter(query, params).await?;
	let affected = query_result.affected_rows() as usize;
	let last_insert_id = query_result.last_insert_id().map(|id| id.to_string());
	let rows = query_result
		.map_and_drop(|row| {
			row.columns()
				.iter()
				.cloned()
				.enumerate()
				.map(|(idx, column)| {
					let ctype = column.column_type();
					let value = row
						.as_ref(idx)
						.expect("length of row was smaller than column count")
						.clone();
					RowPart {
						column,
						value,
						ctype,
					}
				})
				.collect::<Vec<_>>()
		})
		.await?;
	Ok(QueryResult {
		affected,
		last_insert_id,
		rows,
	})
}
