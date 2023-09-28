// SPDX-License-Identifier: MPL-2.0
#![cfg_attr(debug_assertions, allow(unused_imports))]
#![warn(
	clippy::correctness,
	clippy::suspicious,
	clippy::complexity,
	clippy::perf,
	clippy::style
)]
#[macro_use]
extern crate meowtonin;
#[macro_use]
extern crate static_init;

pub mod opts;
pub mod pool;
pub mod query;
pub mod util;

use ahash::AHashMap;
use aneri_core::{ByondSlotKey, RUNTIME};
use crossbeam_channel::{Receiver, TryRecvError};
use meowtonin::{ByondError, ByondResult, ByondValue, ToByond};
use mysql_async::Params as SqlParams;
use slotmap::SlotMap;
use std::{
	collections::hash_map::Entry,
	sync::atomic::{AtomicUsize, Ordering},
	thread,
};

static NEXT_JOB_ID: AtomicUsize = AtomicUsize::new(0);

struct Job {
	rx: Receiver<Result<query::QueryResult, String>>,
	handle: thread::JoinHandle<()>,
}

#[byond_fn]
pub fn sql_query_blocking(
	handle: ByondSlotKey,
	query: String,
	params: Option<ByondValue>,
) -> ByondResult<ByondValue> {
	let pool = match pool::get_conn(handle).expect("failed to get connection") {
		Some(pool) => pool,
		None => unimplemented!("pool offline"),
	};
	let params = match params {
		Some(params) => util::byond_to_params(&params)?,
		None => SqlParams::Empty,
	};
	match RUNTIME
		.read()
		.block_on(query::do_query(pool, query, params))
	{
		Ok(result) => result.to_byond(),
		Err(err) => {
			let mut result = ByondValue::new_list()?;
			result.write_list_index("status", "err")?;
			result.write_list_index("data", err.to_string())?;
			Ok(result)
		}
	}
}

#[dynamic(drop)]
static mut JOBS: AHashMap<usize, Job> = AHashMap::with_capacity(32);

#[byond_fn]
pub fn sql_query_async(
	handle: ByondSlotKey,
	query: String,
	params: Option<ByondValue>,
) -> ByondResult<usize> {
	let pool = match pool::get_conn(handle).expect("failed to get connection") {
		Some(pool) => pool,
		None => unimplemented!("pool offline"),
	};
	let params = match params {
		Some(params) => util::byond_to_params(&params)?,
		None => SqlParams::Empty,
	};
	let (tx, rx) = crossbeam_channel::unbounded();
	let handle = thread::spawn(move || {
		let result = RUNTIME
			.read()
			.block_on(query::do_query(pool, query, params))
			.map_err(|err| err.to_string());
		let _ = tx.send(result);
	});
	let id = NEXT_JOB_ID.fetch_add(1, Ordering::Relaxed);
	JOBS.write().insert(id, Job { rx, handle });
	Ok(id)
}

#[byond_fn]
pub fn sql_query_check_job(id: usize) -> ByondResult<ByondValue> {
	let mut jobs = JOBS.write();
	let entry = match jobs.entry(id) {
		Entry::Occupied(occupied) => occupied,
		Entry::Vacant(_) => return "NO SUCH JOB".to_byond(),
	};
	let result = match entry.get().rx.try_recv() {
		Ok(result) => match result {
			Ok(result) => result.to_byond()?,
			Err(err) => {
				let mut result = ByondValue::new_list()?;
				result.write_list_index("status", "err")?;
				result.write_list_index("data", err.to_string())?;
				result
			}
		},
		Err(TryRecvError::Disconnected) => return "JOB_PANICKED".to_byond(),
		Err(TryRecvError::Empty) => return "NO RESULTS YET".to_byond(),
	};
	let _ = entry.remove().handle.join();
	Ok(result)
}
