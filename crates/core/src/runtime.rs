// SPDX-License-Identifier: MPL-2.0
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use tokio::runtime::{Builder, Runtime};

pub static RUNTIME: Lazy<RwLock<Runtime>> = Lazy::new(|| {
	Builder::new_multi_thread()
		.enable_all()
		.thread_name("meowtonin-tokio-runtime-worker")
		.build()
		.map(RwLock::new)
		.expect("Failed to build Tokio runtime")
});
