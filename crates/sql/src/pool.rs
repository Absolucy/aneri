// SPDX-License-Identifier: MPL-2.0
use crate::opts::{ConnectOptions, DEFAULT_PORT};
use aneri_core::{ByondSlotKey, RUNTIME};
use meowtonin::{ByondError, ByondResult, ByondValue, FromByond};
use meowtonin_serde::value::ByondSerde;
use mysql_async::{
	consts::{ColumnFlags, ColumnType::*},
	prelude::Queryable,
	Conn, OptsBuilder, Params, Pool, PoolConstraints, PoolOpts,
};
use slotmap::{hop, SlotMap};
use std::{str::FromStr, time::Duration};

#[dynamic(drop)]
pub static mut POOLS: SlotMap<ByondSlotKey, Pool> = SlotMap::with_capacity_and_key(1);

pub(crate) fn get_conn(pool: ByondSlotKey) -> Result<Option<Conn>, Box<dyn std::error::Error>> {
	let pools = POOLS.read();
	let pool = match pools.get(pool) {
		Some(pool) => pool,
		None => return Ok(None),
	};
	RUNTIME
		.read()
		.block_on(pool.get_conn())
		.map(Some)
		.map_err(Box::from)
}

#[byond_fn]
pub fn sql_connect_pool(
	mut src: ByondValue,
	options: ByondSerde<ConnectOptions>,
) -> ByondResult<bool> {
	let ConnectOptions {
		host,
		port,
		username,
		password,
		database,
		wait_timeout,
		min_threads,
		max_threads,
	} = options.into_inner();
	let pool_constraints = PoolConstraints::new(min_threads, max_threads).expect(
		"invalid pool constraints, max_threads must be equal to or greater than min_threads",
	);
	let pool_opts = PoolOpts::with_constraints(PoolOpts::new(), pool_constraints);
	let builder = OptsBuilder::default()
		.ip_or_hostname(host)
		.tcp_port(port)
		// Work around addresses like `localhost:3307` defaulting to socket as
		// if the port were the default too.
		.prefer_socket(port == DEFAULT_PORT)
		.user(username)
		.pass(password)
		.db_name(Some(database))
		.wait_timeout(wait_timeout)
		.pool_opts(pool_opts);
	let pool = Pool::new(builder);
	POOLS.write().insert(pool).save(&mut src)?;
	Ok(true)
}

#[byond_fn]
pub fn sql_disconnect_pool(pool: ByondSlotKey) -> ByondResult<bool> {
	match POOLS.write().remove(pool) {
		// TODO: clear slotkey to prevent any potential reuse
		Some(pool) => {
			RUNTIME
				.read()
				.block_on(pool.clone().disconnect())
				.map_err(ByondError::boxed)?;
			Ok(true)
		}
		None => Ok(false),
	}
}

#[byond_fn]
pub fn sql_connected(pool: ByondSlotKey) -> bool {
	POOLS.read().contains_key(pool)
}
