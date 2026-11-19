// SPDX-License-Identifier: MPL-2.0
use meowtonin::{ByondValue, sync::thread_sync};
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

type Task = Box<dyn FnOnce() + Send>;

static QUEUE: Mutex<Vec<Task>> = Mutex::new(Vec::new());
static WAKE_PENDING: AtomicBool = AtomicBool::new(false);

/// Runs `task` on BYOND's main thread, the next time BYOND gets a chance.
///
/// Tasks queued close together run in one batch, in the order they were
/// queued.
pub fn queue(task: impl FnOnce() + Send + 'static) {
	QUEUE.lock().push(Box::new(task));
	if !WAKE_PENDING.swap(true, Ordering::AcqRel) {
		let _ = thread_sync(
			|| {
				drain();
				ByondValue::NULL
			},
			false,
		);
	}
}

fn drain() {
	WAKE_PENDING.store(false, Ordering::Release);
	let tasks = std::mem::take(&mut *QUEUE.lock());
	for task in tasks {
		task();
	}
}

pub(crate) fn clear() {
	*QUEUE.lock() = Vec::new();
	WAKE_PENDING.store(false, Ordering::Release);
}
