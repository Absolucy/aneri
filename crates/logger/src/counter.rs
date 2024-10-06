// SPDX-License-Identifier: MPL-2.0
use parking_lot::{Condvar, Mutex, RwLock};
use std::{
	sync::{Arc, LazyLock},
	time::{Duration, Instant},
};

pub(crate) static THREAD_COUNTER: LazyLock<RwLock<Arc<ThreadCounter>>> =
	LazyLock::new(RwLock::default);

#[cfg_attr(target_pointer_width = "32", repr(align(64)))]
#[cfg_attr(target_pointer_width = "64", repr(align(128)))]
pub(crate) struct ThreadCounter {
	count: Mutex<usize>,
	condvar: Condvar,
}

impl ThreadCounter {
	pub fn increment(&self) {
		let mut count = self.count.lock();
		*count += 1;
	}

	pub fn decrement(&self) {
		let mut count = self.count.lock();
		*count -= 1;
		if *count == 0 {
			self.condvar.notify_all();
		}
	}

	pub fn wait_for_zero(&self, timeout: Duration) -> bool {
		let start = Instant::now();
		let mut count = self.count.lock();

		while *count > 0 {
			let remaining = match timeout.checked_sub(start.elapsed()) {
				Some(remaining) => remaining,
				None => break,
			};
			if self.condvar.wait_for(&mut count, remaining).timed_out() {
				return false;
			}
		}
		true
	}
}

impl Default for ThreadCounter {
	fn default() -> Self {
		ThreadCounter {
			count: Mutex::new(0),
			condvar: Condvar::new(),
		}
	}
}

pub(crate) fn reset_thread_counter() {
	*THREAD_COUNTER.write() = Arc::default();
}
