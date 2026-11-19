// SPDX-License-Identifier: MPL-2.0
use mysql_async::{
	BinaryProtocol, Column as RawColumn, Params, Pool, QueryResult, Value, consts::ColumnType,
	prelude::Queryable,
};
use std::time::Duration;

const MAX_RESULT_SIZE: usize = 64 * 1024 * 1024;
const UNCACHED_SQL_LEN: usize = 2048;
const BINARY_CHARSET: u16 = 63;

pub(crate) struct Request {
	pub sql: String,
	pub params: Vec<Value>,
	pub timeout: Option<Duration>,
}

#[derive(Clone, Copy)]
pub(crate) enum ColumnKind {
	Text,
	Bytes,
	Decimal,
	Bit,
	Date,
}

pub(crate) struct Column {
	pub name: String,
	pub kind: ColumnKind,
}

#[derive(Default)]
pub(crate) struct Output {
	pub columns: Vec<Column>,
	pub rows: Vec<Vec<Value>>,
	pub affected: u64,
	pub last_insert_id: Option<u64>,
}

#[derive(Clone)]
pub(crate) struct Failure {
	pub message: String,
	pub code: Option<u16>,
}

impl Failure {
	pub fn new(message: impl Into<String>) -> Self {
		Self {
			message: message.into(),
			code: None,
		}
	}
}

impl From<mysql_async::Error> for Failure {
	fn from(err: mysql_async::Error) -> Self {
		match err {
			mysql_async::Error::Server(err) => Self {
				message: err.message,
				code: Some(err.code),
			},
			err => Self::new(err.to_string()),
		}
	}
}

pub(crate) type Outcome = Result<Output, Failure>;

pub(crate) fn seconds(value: Option<f32>) -> Option<Duration> {
	value
		.filter(|secs| *secs > 0.0)
		.and_then(|secs| Duration::try_from_secs_f32(secs).ok())
}

pub(crate) async fn limit<T>(
	timeout: Option<Duration>,
	work: impl Future<Output = Result<T, Failure>>,
) -> Result<T, Failure> {
	let Some(timeout) = timeout else {
		return work.await;
	};
	tokio::time::timeout(timeout, work)
		.await
		.unwrap_or_else(|_| {
			Err(Failure::new(format!(
				"timed out after {} seconds",
				timeout.as_secs_f32()
			)))
		})
}

pub(crate) async fn run_pooled(pool: Pool, request: Request) -> Outcome {
	limit(request.timeout, async {
		if request.sql.is_empty() {
			return Ok(Output::default());
		}
		let mut conn = pool.get_conn().await?;
		run_on(&mut conn, request.sql, request.params).await
	})
	.await
}

pub(crate) async fn run_on(conn: &mut impl Queryable, sql: String, params: Vec<Value>) -> Outcome {
	if sql.is_empty() {
		return Ok(Output::default());
	}
	let params = if params.is_empty() {
		Params::Empty
	} else {
		Params::Positional(params)
	};
	if sql.len() <= UNCACHED_SQL_LEN {
		return read(conn.exec_iter(sql, params).await?).await;
	}
	let stmt = conn.prep(sql).await?;
	let outcome = match conn.exec_iter(&stmt, params).await {
		Ok(result) => read(result).await,
		Err(err) => Err(err.into()),
	};
	conn.close(stmt).await?;
	outcome
}

async fn read(mut result: QueryResult<'_, 'static, BinaryProtocol>) -> Outcome {
	let columns = result
		.columns_ref()
		.iter()
		.map(|column| Column {
			name: column.name_str().into_owned(),
			kind: kind(column),
		})
		.collect::<Vec<_>>();
	let mut rows = Vec::new();
	let mut size = 0;
	while let Some(row) = result.next().await? {
		let values = row.unwrap();
		size += values.iter().map(value_size).sum::<usize>();
		if size > MAX_RESULT_SIZE {
			result.drop_result().await?;
			return Err(Failure::new(
				"the result is over 64 MiB, select fewer rows or columns",
			));
		}
		rows.push(values);
	}
	let affected = result.affected_rows();
	let last_insert_id = result.last_insert_id().filter(|id| *id != 0);
	result.drop_result().await?;
	Ok(Output {
		columns,
		rows,
		affected,
		last_insert_id,
	})
}

fn kind(column: &RawColumn) -> ColumnKind {
	match column.column_type() {
		ColumnType::MYSQL_TYPE_DECIMAL | ColumnType::MYSQL_TYPE_NEWDECIMAL => ColumnKind::Decimal,
		ColumnType::MYSQL_TYPE_BIT => ColumnKind::Bit,
		ColumnType::MYSQL_TYPE_DATE | ColumnType::MYSQL_TYPE_NEWDATE => ColumnKind::Date,
		ColumnType::MYSQL_TYPE_VARCHAR
		| ColumnType::MYSQL_TYPE_VAR_STRING
		| ColumnType::MYSQL_TYPE_STRING
		| ColumnType::MYSQL_TYPE_TINY_BLOB
		| ColumnType::MYSQL_TYPE_MEDIUM_BLOB
		| ColumnType::MYSQL_TYPE_LONG_BLOB
		| ColumnType::MYSQL_TYPE_BLOB
		| ColumnType::MYSQL_TYPE_GEOMETRY
			if column.character_set() == BINARY_CHARSET =>
		{
			ColumnKind::Bytes
		}
		_ => ColumnKind::Text,
	}
}

fn value_size(value: &Value) -> usize {
	match value {
		Value::Bytes(bytes) => bytes.len() + size_of::<Value>(),
		_ => size_of::<Value>(),
	}
}
