// SPDX-License-Identifier: MPL-2.0
use crate::{
	query::{self, Failure, Outcome, Output, Request},
	report::{Forgotten, Report},
};
use aneri_core::ByondSlotMap;
use mysql_async::{Pool, Transaction, TxOpts};
use std::{
	sync::atomic::{AtomicU64, Ordering},
	time::Duration,
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

const ROLLBACK_LIMIT: Duration = Duration::from_secs(5);

pub(crate) static TRANSACTIONS: ByondSlotMap<UnboundedSender<Command>> = ByondSlotMap::new();
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub(crate) fn next_id() -> String {
	format!("t{}", NEXT_ID.fetch_add(1, Ordering::Relaxed))
}

pub(crate) enum Reply {
	Report(Report),
	Forget(Forgotten),
}

impl Reply {
	fn send(self, outcome: Outcome) {
		match self {
			Self::Report(report) => report.send(outcome),
			Self::Forget(_) => {}
		}
	}
}

pub(crate) enum Command {
	Statement {
		request: Result<Request, Failure>,
		reply: Reply,
	},
	End {
		commit: bool,
		timeout: Option<Duration>,
		reply: Reply,
	},
}

impl Command {
	pub fn into_reply(self) -> Reply {
		match self {
			Self::Statement { reply, .. } | Self::End { reply, .. } => reply,
		}
	}
}

pub(crate) fn ended() -> Failure {
	Failure::new("the transaction has already ended")
}

type State = Result<Transaction<'static>, Failure>;

pub(crate) async fn run(
	pool: Option<Pool>,
	mut commands: UnboundedReceiver<Command>,
	begin_timeout: Option<Duration>,
	idle_timeout: Option<Duration>,
) {
	let mut state: State = match pool {
		Some(pool) => {
			query::limit(begin_timeout, async {
				Ok(pool.start_transaction(TxOpts::default()).await?)
			})
			.await
		}
		None => Err(Failure::new("the pool is closed")),
	};
	while let Some(command) = next(&mut commands, idle_timeout).await {
		match command {
			Command::Statement { request, reply } => {
				let outcome = statement(&mut state, request).await;
				reply.send(outcome);
			}
			Command::End {
				commit,
				timeout,
				reply,
			} => {
				let state = std::mem::replace(&mut state, Err(ended()));
				reply.send(end(state, commit, timeout).await);
				break;
			}
		}
	}
	if let Ok(transaction) = state {
		rollback(transaction).await;
	}
	commands.close();
	while let Ok(command) = commands.try_recv() {
		command.into_reply().send(Err(ended()));
	}
}

async fn next(
	commands: &mut UnboundedReceiver<Command>,
	idle_timeout: Option<Duration>,
) -> Option<Command> {
	match idle_timeout {
		Some(idle_timeout) => tokio::time::timeout(idle_timeout, commands.recv())
			.await
			.ok()
			.flatten(),
		None => commands.recv().await,
	}
}

async fn statement(state: &mut State, request: Result<Request, Failure>) -> Outcome {
	let transaction = match state {
		Ok(transaction) => transaction,
		Err(first) => return Err(first.clone()),
	};
	let outcome = match request {
		Ok(request) => {
			query::limit(
				request.timeout,
				query::run_on(transaction, request.sql, request.params),
			)
			.await
		}
		Err(failure) => Err(failure),
	};
	let Err(failure) = &outcome else {
		return outcome;
	};
	if let Ok(transaction) = std::mem::replace(state, Err(failure.clone())) {
		rollback(transaction).await;
	}
	outcome
}

async fn end(state: State, commit: bool, timeout: Option<Duration>) -> Outcome {
	let transaction = state?;
	query::limit(timeout, async {
		if commit {
			transaction.commit().await?;
		} else {
			transaction.rollback().await?;
		}
		Ok(Output::default())
	})
	.await
}

async fn rollback(transaction: Transaction<'static>) {
	let _ = tokio::time::timeout(ROLLBACK_LIMIT, transaction.rollback()).await;
}
