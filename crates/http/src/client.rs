// SPDX-License-Identifier: MPL-2.0
use parking_lot::Mutex;
use reqwest::{Certificate, Client};
use std::time::Duration;

static CLIENT: Mutex<Option<Client>> = Mutex::new(None);

pub(crate) fn get() -> Client {
	CLIENT.lock().get_or_insert_with(build).clone()
}

pub(crate) fn clear() {
	*CLIENT.lock() = None;
}

fn build() -> Client {
	let _ = rustls::crypto::ring::default_provider().install_default();
	let roots = webpki_root_certs::TLS_SERVER_ROOT_CERTS
		.iter()
		.filter_map(|cert| Certificate::from_der(cert).ok());
	Client::builder()
		.user_agent(concat!("aneri/", env!("CARGO_PKG_VERSION")))
		.connect_timeout(Duration::from_secs(10))
		.pool_max_idle_per_host(8)
		.pool_idle_timeout(Duration::from_secs(30))
		.tls_certs_only(roots)
		.build()
		.expect("failed to build HTTP client")
}
