// SPDX-License-Identifier: MPL-2.0
use aneri_core::ByondSlotMap;
use mysql_async::{OptsBuilder, Pool, PoolConstraints, PoolOpts};
use std::{path::PathBuf, time::Duration};

const IDLE_CONNECTION_TTL: Duration = Duration::from_secs(60);

pub(crate) static POOLS: ByondSlotMap<Pool> = ByondSlotMap::new();

pub(crate) struct Settings {
	pub host: String,
	pub port: u16,
	pub user: Option<String>,
	pub password: Option<String>,
	pub database: Option<String>,
	pub min_connections: usize,
	pub max_connections: usize,
	pub tls: bool,
	pub tls_ca_file: Option<PathBuf>,
	pub tls_insecure: bool,
}

pub(crate) fn build(settings: Settings) -> Result<Pool, String> {
	let constraints = PoolConstraints::new(settings.min_connections, settings.max_connections)
		.ok_or_else(|| {
			format!(
				"min_connections ({}) must not be above max_connections ({}), and max_connections \
				 must be at least 1",
				settings.min_connections, settings.max_connections
			)
		})?;
	let pool_opts = PoolOpts::default()
		.with_constraints(constraints)
		.with_reset_connection(false)
		.with_inactive_connection_ttl(IDLE_CONNECTION_TTL);
	let mut opts = OptsBuilder::default()
		.ip_or_hostname(settings.host)
		.tcp_port(settings.port)
		.user(settings.user)
		.pass(settings.password)
		.db_name(settings.database)
		.prefer_socket(false)
		.pool_opts(pool_opts);
	if settings.tls {
		opts = opts.ssl_opts(tls_opts(settings.tls_ca_file, settings.tls_insecure)?);
	}
	Ok(Pool::new(opts))
}

#[cfg(feature = "tls")]
fn tls_opts(ca_file: Option<PathBuf>, insecure: bool) -> Result<mysql_async::SslOpts, String> {
	let _ = rustls::crypto::ring::default_provider().install_default();
	Ok(mysql_async::SslOpts::default()
		.with_root_certs(ca_file.into_iter().map(Into::into).collect())
		.with_danger_accept_invalid_certs(insecure)
		.with_danger_skip_domain_validation(insecure))
}

#[cfg(not(feature = "tls"))]
fn tls_opts(_: Option<PathBuf>, _: bool) -> Result<mysql_async::SslOpts, String> {
	Err("aneri was built without TLS support".to_owned())
}
