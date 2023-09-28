// SPDX-License-Identifier: MPL-2.0
use static_init::dynamic;
use tokio::runtime::{Builder, Runtime};

#[dynamic(drop)]
pub static mut RUNTIME: Runtime = {
	Builder::new_multi_thread()
		.enable_all()
		.thread_name("meowtonin-tokio-runtime-worker")
		.build()
		.expect("Failed to build Tokio runtime")
};
