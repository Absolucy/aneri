// SPDX-License-Identifier: MPL-2.0
#![allow(clippy::too_many_arguments)]
mod insert;
mod params;
mod pool;
mod query;
mod report;
mod transaction;
mod value;

use aneri_core::{ByondSlotKey, job, runtime};
use meowtonin::{ByondError, ByondResult, ByondValue, byond_fn};
use mysql_async::Pool;
use pool::POOLS;
use query::{Failure, Request};
use report::{Forgotten, Report};
use std::{path::PathBuf, time::Duration};
use tokio::sync::mpsc::UnboundedSender;
use transaction::{Command, Reply, TRANSACTIONS};

const DISCONNECT_LIMIT: Duration = Duration::from_secs(2);

/// Drops every transaction and closes every pool.
pub fn cleanup() {
	drop(TRANSACTIONS.take_all());
	let pools = POOLS.take_all();
	if pools.is_empty() {
		return;
	}
	let handle = runtime::handle();
	let tasks = pools
		.into_iter()
		.map(|pool| handle.spawn(pool.disconnect()))
		.collect::<Vec<_>>();
	handle.block_on(async {
		let _ = tokio::time::timeout(DISCONNECT_LIMIT, async {
			for task in tasks {
				let _ = task.await;
			}
		})
		.await;
	});
}

enum Mode {
	Background,
	Blocking,
	Forget,
}

impl Mode {
	fn from_number(mode: u8) -> ByondResult<Self> {
		match mode {
			0 => Ok(Self::Background),
			1 => Ok(Self::Blocking),
			2 => Ok(Self::Forget),
			_ => Err(message_error(format!("unknown sql mode {mode}"))),
		}
	}
}

enum Target {
	Pool(Option<Pool>),
	Transaction(Option<UnboundedSender<Command>>),
}

impl Target {
	fn find(pool: Option<ByondSlotKey>, transaction: Option<ByondSlotKey>) -> Self {
		match transaction {
			Some(key) => Self::Transaction(TRANSACTIONS.with(key, Clone::clone)),
			None => Self::Pool(pool.and_then(|key| POOLS.with(key, Clone::clone))),
		}
	}
}

/// Creates the pool behind a `/datum/sql_pool` and stores its key on `src`.
/// Connections open on first use.
#[byond_fn(name = "aneri_raw_sql_pool_new")]
pub fn sql_pool_new(
	mut src: ByondValue,
	host: String,
	port: u16,
	user: Option<String>,
	password: Option<String>,
	/// The database to `USE`, or null for none.
	database: Option<String>,
	/// Connections kept open even when idle.
	min_connections: usize,
	/// Most connections open at once.
	/// Queries past that wait for a free one.
	max_connections: usize,
	/// Connect with TLS.
	tls: bool,
	/// Extra CA certificate file to trust, for a self-signed server.
	tls_ca_file: Option<PathBuf>,
	/// Accept any server certificate.
	tls_insecure: bool,
) -> ByondResult<()> {
	let pool = pool::build(pool::Settings {
		host,
		port,
		user,
		password,
		database,
		min_connections,
		max_connections,
		tls,
		tls_ca_file,
		tls_insecure,
	})
	.map_err(message_error)?;
	POOLS.insert(pool).save(&mut src)
}

/// Closes a pool. Queries already running on it still finish.
#[byond_fn(name = "aneri_raw_sql_pool_del")]
pub fn sql_pool_del(src: ByondSlotKey) -> bool {
	let Some(pool) = POOLS.remove(src) else {
		return false;
	};
	runtime::handle().spawn(pool.disconnect());
	true
}

/// Starts a transaction on a pool, for a `/datum/sql_transaction`.
#[byond_fn(name = "aneri_raw_sql_transaction_new")]
pub fn sql_transaction_new(
	mut src: ByondValue,
	pool: Option<ByondSlotKey>,
	/// Seconds to get a connection and begin.
	timeout: Option<f32>,
	/// Seconds with no new statement before it rolls back by itself.
	idle_timeout: Option<f32>,
) -> ByondResult<()> {
	let pool = pool.and_then(|key| POOLS.with(key, Clone::clone));
	let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
	TRANSACTIONS.insert(sender).save(&mut src)?;
	let timeout = query::seconds(timeout);
	let idle_timeout = query::seconds(idle_timeout);
	job::spawn(move |_| transaction::run(pool, receiver, timeout, idle_timeout));
	Ok(())
}

/// Lets go of a transaction.
/// Statements already started still run,
/// then it rolls back unless it was committed.
#[byond_fn(name = "aneri_raw_sql_transaction_del")]
pub fn sql_transaction_del(src: ByondSlotKey) -> bool {
	TRANSACTIONS.remove(src).is_some()
}

/// Runs a statement on a pool or in a transaction.
/// Mistakes in the query come back as the result's error, not as runtimes.
#[byond_fn(name = "aneri_raw_sql_execute")]
pub fn sql_execute(
	/// 0 runs in the background and returns an id for `/proc/__aneri_sql_finish`.
	/// 1 blocks and returns `list(columns, rows, affected, last_insert_id, error, error_code)`.
	/// 2 runs in the background and reports nothing.
	mode: u8,
	pool: Option<ByondSlotKey>,
	/// Used instead of `pool` when set.
	transaction: Option<ByondSlotKey>,
	sql: String,
	/// A list of values for `?` placeholders,
	/// or an assoc list for `:name` ones.
	arguments: ByondValue,
	/// Seconds for getting a connection plus running the query.
	/// Null waits forever.
	timeout: Option<f32>,
) -> ByondResult<ByondValue> {
	let mode = Mode::from_number(mode)?;
	let request = params::bind(sql, &arguments).map(|(sql, params)| Request {
		sql,
		params,
		timeout: query::seconds(timeout),
	});
	dispatch(mode, Target::find(pool, transaction), request)
}

/// Inserts many rows in one statement, like tgstation's `SSdbcore.MassInsert`.
/// Takes the same modes as `aneri_raw_sql_execute`.
#[byond_fn(name = "aneri_raw_sql_insert")]
pub fn sql_insert(
	/// Same as `aneri_raw_sql_execute`.
	mode: u8,
	pool: Option<ByondSlotKey>,
	/// Used instead of `pool` when set.
	transaction: Option<ByondSlotKey>,
	table: String,
	/// A list of assoc lists, column name to value.
	/// At most 65535 values in total.
	rows: ByondValue,
	/// TRUE updates every column on a duplicate key.
	/// Text is appended to the statement as-is.
	duplicate_key: ByondValue,
	/// Use `INSERT IGNORE`.
	ignore_errors: bool,
	/// Column name to SQL expression used for every row.
	/// Each `?` in it is that row's value for the column.
	special_columns: Option<Vec<(String, String)>>,
	/// Seconds for getting a connection plus running the query.
	/// Null waits forever.
	timeout: Option<f32>,
) -> ByondResult<ByondValue> {
	let mode = Mode::from_number(mode)?;
	let request = insert::build(
		&table,
		&rows,
		&duplicate_key,
		ignore_errors,
		special_columns.unwrap_or_default(),
	)
	.map(|(sql, params)| Request {
		sql,
		params,
		timeout: query::seconds(timeout),
	});
	dispatch(mode, Target::find(pool, transaction), request)
}

/// Commits or rolls back a transaction in the background.
/// Returns an id for `/proc/__aneri_sql_finish`.
#[byond_fn(name = "aneri_raw_sql_transaction_end")]
pub fn sql_transaction_end(
	transaction: Option<ByondSlotKey>,
	commit: bool,
	timeout: Option<f32>,
) -> String {
	let id = transaction::next_id();
	let command = Command::End {
		commit,
		timeout: query::seconds(timeout),
		reply: Reply::Report(Report::new(id.clone())),
	};
	let sender = transaction.and_then(|key| TRANSACTIONS.with(key, Clone::clone));
	send(sender.as_ref(), command);
	id
}

/// Cancels a background query that is not part of a transaction.
/// Its finish proc will not be called.
#[byond_fn(name = "aneri_raw_sql_cancel")]
pub fn sql_cancel(id: String) -> bool {
	id.parse().is_ok_and(job::cancel)
}

/// How many fire-and-forget queries are still running or waiting.
#[byond_fn(name = "aneri_raw_sql_forgotten_in_flight")]
pub fn sql_forgotten_in_flight() -> f32 {
	report::forgotten_in_flight() as f32
}

fn dispatch(
	mode: Mode,
	target: Target,
	request: Result<Request, Failure>,
) -> ByondResult<ByondValue> {
	match (mode, target) {
		(Mode::Background, Target::Pool(pool)) => {
			let id = job::spawn(|id| async move {
				let report = Report::new(id.to_string());
				let outcome = match (request, pool) {
					(Ok(request), Some(pool)) => query::run_pooled(pool, request).await,
					(Ok(_), None) => Err(closed()),
					(Err(failure), _) => Err(failure),
				};
				report.send(outcome);
			});
			Ok(ByondValue::new_string(id.to_string()))
		}
		(Mode::Blocking, Target::Pool(pool)) => {
			let outcome = match (request, pool) {
				(Ok(request), Some(pool)) => {
					runtime::handle().block_on(query::run_pooled(pool, request))
				}
				(Ok(_), None) => Err(closed()),
				(Err(failure), _) => Err(failure),
			};
			result_list(outcome)
		}
		(Mode::Forget, Target::Pool(pool)) => {
			if let (Ok(request), Some(pool)) = (request, pool) {
				let forgotten = Forgotten::new();
				runtime::handle().spawn(async move {
					let _ = query::run_pooled(pool, request).await;
					drop(forgotten);
				});
			}
			Ok(ByondValue::NULL)
		}
		(Mode::Background, Target::Transaction(sender)) => {
			let id = transaction::next_id();
			let reply = Reply::Report(Report::new(id.clone()));
			send(sender.as_ref(), Command::Statement { request, reply });
			Ok(ByondValue::new_string(id))
		}
		(Mode::Blocking, Target::Transaction(_)) => result_list(Err(Failure::new(
			"statements in a transaction can only run in the background, use execute()",
		))),
		(Mode::Forget, Target::Transaction(sender)) => {
			let reply = Reply::Forget(Forgotten::new());
			send(sender.as_ref(), Command::Statement { request, reply });
			Ok(ByondValue::NULL)
		}
	}
}

fn send(sender: Option<&UnboundedSender<Command>>, command: Command) {
	let command = match sender {
		Some(sender) => match sender.send(command) {
			Ok(()) => return,
			Err(err) => err.0,
		},
		None => command,
	};
	match command.into_reply() {
		Reply::Report(report) => report.send_later(Err(transaction::ended())),
		Reply::Forget(_) => {}
	}
}

fn result_list(outcome: query::Outcome) -> ByondResult<ByondValue> {
	let mut list = ByondValue::new_list()?;
	list.write_list(report::outcome_values(outcome))?;
	Ok(list)
}

fn closed() -> Failure {
	Failure::new("the pool is closed")
}

fn message_error(message: String) -> ByondError {
	ByondError::Boxed(Box::<dyn std::error::Error + Send + Sync>::from(message))
}
