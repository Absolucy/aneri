// SPDX-License-Identifier: MPL-2.0
use parking_lot::Mutex;
use std::time::Duration;
use tokio::runtime::{Builder, Handle, Runtime};

const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);

static RUNTIME: Mutex<Option<Runtime>> = Mutex::new(None);

/// Returns a handle to the shared runtime, starting it if needed.
pub fn handle() -> Handle {
	RUNTIME.lock().get_or_insert_with(build).handle().clone()
}

/// Cancels every job, stops the runtime, and drops any results that haven't
/// reached the main thread yet. The next [`handle`] call starts a new runtime.
pub fn shutdown() {
	crate::job::cancel_all();
	let runtime = RUNTIME.lock().take();
	if let Some(runtime) = runtime {
		runtime.shutdown_timeout(SHUTDOWN_TIMEOUT);
	}
	crate::main_thread::clear();
}

fn build() -> Runtime {
	Builder::new_multi_thread()
		.worker_threads(2)
		.max_blocking_threads(4)
		.thread_name("aneri-runtime")
		.enable_all()
		.build()
		.expect("failed to build tokio runtime")
}
