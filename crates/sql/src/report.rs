// SPDX-License-Identifier: MPL-2.0
use crate::{
	query::{Failure, Outcome, Output},
	value,
};
use aneri_core::{main_thread, runtime};
use meowtonin::{ByondResult, ByondValue};
use std::sync::atomic::{AtomicUsize, Ordering};

static FORGOTTEN_IN_FLIGHT: AtomicUsize = AtomicUsize::new(0);

pub(crate) fn forgotten_in_flight() -> usize {
	FORGOTTEN_IN_FLIGHT.load(Ordering::Relaxed)
}

/// Reports a query's result to DM once, or an error if dropped inside the
/// runtime without reporting.
pub(crate) struct Report {
	id: Option<String>,
}

impl Report {
	pub fn new(id: String) -> Self {
		Self { id: Some(id) }
	}

	pub fn send(mut self, outcome: Outcome) {
		if let Some(id) = self.id.take() {
			main_thread::queue(move || finish(id, outcome));
		}
	}

	pub fn send_later(mut self, outcome: Outcome) {
		if let Some(id) = self.id.take() {
			runtime::handle().spawn(async move {
				main_thread::queue(move || finish(id, outcome));
			});
		}
	}
}

impl Drop for Report {
	fn drop(&mut self) {
		let Some(id) = self.id.take() else {
			return;
		};
		if tokio::runtime::Handle::try_current().is_err() {
			return;
		}
		let failure = Failure::new("the query stopped without a result");
		main_thread::queue(move || finish(id, Err(failure)));
	}
}

/// Counts a fire-and-forget query for as long as it exists.
pub(crate) struct Forgotten(());

impl Forgotten {
	pub fn new() -> Self {
		FORGOTTEN_IN_FLIGHT.fetch_add(1, Ordering::Relaxed);
		Self(())
	}
}

impl Drop for Forgotten {
	fn drop(&mut self) {
		FORGOTTEN_IN_FLIGHT.fetch_sub(1, Ordering::Relaxed);
	}
}

fn finish(id: String, outcome: Outcome) {
	let [columns, rows, affected, last_insert_id, error, error_code] = outcome_values(outcome);
	let _ = meowtonin::call_global::<_, _, _, ByondValue>("__aneri_sql_finish", [
		ByondValue::new_string(id),
		columns,
		rows,
		affected,
		last_insert_id,
		error,
		error_code,
	]);
}

/// `[columns, rows, affected, last_insert_id, error, error_code]`
pub(crate) fn outcome_values(outcome: Outcome) -> [ByondValue; 6] {
	match outcome {
		Ok(output) => output_values(output).unwrap_or_else(|err| {
			failure_values(Failure::new(format!(
				"failed to turn the result into DM values: {err}"
			)))
		}),
		Err(failure) => failure_values(failure),
	}
}

fn output_values(output: Output) -> ByondResult<[ByondValue; 6]> {
	let affected = ByondValue::new_num(output.affected as f32);
	let last_insert_id = output
		.last_insert_id
		.map_or(ByondValue::NULL, |id| ByondValue::new_num(id as f32));
	if output.columns.is_empty() {
		return Ok([
			ByondValue::NULL,
			ByondValue::NULL,
			affected,
			last_insert_id,
			ByondValue::NULL,
			ByondValue::NULL,
		]);
	}
	let mut columns = ByondValue::new_list()?;
	columns.write_list(
		output
			.columns
			.iter()
			.map(|column| ByondValue::new_string(column.name.as_str())),
	)?;
	let rows = output
		.rows
		.into_iter()
		.map(|row| {
			let cells = row
				.into_iter()
				.zip(&output.columns)
				.map(|(cell, column)| value::to_byond(cell, column.kind))
				.collect::<ByondResult<Vec<_>>>()?;
			let mut list = ByondValue::new_list()?;
			list.write_list(cells)?;
			Ok(list)
		})
		.collect::<ByondResult<Vec<_>>>()?;
	let mut row_list = ByondValue::new_list()?;
	row_list.write_list(rows)?;
	Ok([
		columns,
		row_list,
		affected,
		last_insert_id,
		ByondValue::NULL,
		ByondValue::NULL,
	])
}

fn failure_values(failure: Failure) -> [ByondValue; 6] {
	[
		ByondValue::NULL,
		ByondValue::NULL,
		ByondValue::NULL,
		ByondValue::NULL,
		ByondValue::new_string(failure.message),
		failure.code.map_or(ByondValue::NULL, ByondValue::new_num),
	]
}
