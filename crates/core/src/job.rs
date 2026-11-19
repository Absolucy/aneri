// SPDX-License-Identifier: MPL-2.0
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use std::sync::{
	LazyLock,
	atomic::{AtomicU64, Ordering},
};
use tokio::task::AbortHandle;

static NEXT_ID: AtomicU64 = AtomicU64::new(1);
static JOBS: LazyLock<Mutex<FxHashMap<u64, AbortHandle>>> = LazyLock::new(Mutex::default);

/// Runs a future on the shared runtime and returns its job id.
/// `job` gets the id too, so it can say which job a result belongs to.
pub fn spawn<F, Fut>(job: F) -> u64
where
	F: FnOnce(u64) -> Fut,
	Fut: Future<Output = ()> + Send + 'static,
{
	let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
	let future = job(id);
	let handle = crate::runtime::handle();
	let mut jobs = JOBS.lock();
	let task = handle.spawn(async move {
		future.await;
		JOBS.lock().remove(&id);
	});
	jobs.insert(id, task.abort_handle());
	id
}

/// Cancels a job. Returns false if it already finished or never existed.
pub fn cancel(id: u64) -> bool {
	let Some(task) = JOBS.lock().remove(&id) else {
		return false;
	};
	task.abort();
	true
}

/// Cancels every job.
pub fn cancel_all() {
	let jobs = std::mem::take(&mut *JOBS.lock());
	for task in jobs.into_values() {
		task.abort();
	}
}
