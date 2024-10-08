// SPDX-License-Identifier: MPL-2.0
use parking_lot::RwLock;
use std::sync::LazyLock;
use thread_counter::{ThreadCounter, Ticket};

pub static THREAD_COUNTER: LazyLock<RwLock<ThreadCounter>> = LazyLock::new(RwLock::default);

pub fn take_thread_ticket() -> Ticket {
	THREAD_COUNTER.read().ticket()
}

pub fn reset_thread_counter() {
	*THREAD_COUNTER.write() = ThreadCounter::default();
}
