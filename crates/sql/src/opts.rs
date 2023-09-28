// SPDX-License-Identifier: MPL-2.0
use mysql_async::PoolConstraints;
use serde::Deserialize;

pub const DEFAULT_PORT: u16 = 3306;

#[derive(Deserialize)]
pub struct ConnectOptions {
	pub host: String,
	#[serde(default = "default_port")]
	pub port: u16,
	#[serde(alias = "user")]
	pub username: Option<String>,
	#[serde(alias = "pass")]
	pub password: Option<String>,
	#[serde(alias = "db_name")]
	pub database: String,
	pub wait_timeout: Option<usize>,
	#[serde(default = "default_min_threads")]
	pub min_threads: usize,
	#[serde(default = "default_max_threads")]
	pub max_threads: usize,
}

const fn default_port() -> u16 {
	DEFAULT_PORT
}

const fn default_min_threads() -> usize {
	1
}

const fn default_max_threads() -> usize {
	10
}
