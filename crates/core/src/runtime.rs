// SPDX-License-Identifier: MPL-2.0
use parking_lot::RwLock;
use std::sync::LazyLock;
use tokio::runtime::{Builder, Runtime};

pub static RUNTIME: LazyLock<RwLock<Runtime>> = LazyLock::new(|| {
	Builder::new_multi_thread()
		.enable_all()
		.thread_name("meowtonin-tokio-runtime-worker")
		.build()
		.map(RwLock::new)
		.expect("Failed to build Tokio runtime")
});
