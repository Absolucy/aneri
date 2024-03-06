// SPDX-License-Identifier: MPL-2.0

use crate::message::LogMessage;
use ahash::AHashMap;
use crossbeam_channel::Sender;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::{
	path::{Path, PathBuf},
	sync::atomic::{AtomicUsize, Ordering},
	time::{Duration, Instant},
};

pub(crate) static THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

/// A log message sender
/// A None value indicates the end of the queue.
pub(crate) type MessageQueue = Sender<LogMessage>;
/// A map of log file paths to their respective queues.
pub(crate) type LogQueues = AHashMap<PathBuf, MessageQueue>;

const DEFAULT_CAPACITY: usize = 16;
const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(10);

static MESSAGE_QUEUE: Lazy<Mutex<LogQueues>> =
	Lazy::new(|| Mutex::new(AHashMap::with_capacity(DEFAULT_CAPACITY)));

/// Clear the log message queue for a given path.
/// Any existing log messages will still be written to the log file.
pub fn clear_log_queue() {
	if let Some(queue) = Lazy::get(&MESSAGE_QUEUE) {
		queue.lock().clear();
	}
	let start = Instant::now();
	while THREAD_COUNT.load(Ordering::SeqCst) > 0 {
		if start.elapsed() > SHUTDOWN_TIMEOUT {
			panic!("failed to shut down logger threads in time");
		}
		std::thread::yield_now();
	}
}

/// Returns the log queue for a given path, creating it if it doesn't exist.
pub(crate) fn get_queue(path: &Path) -> MessageQueue {
	MESSAGE_QUEUE
		.lock()
		.entry(path.to_path_buf())
		.or_insert_with(|| initialize_queue(path))
		.clone()
}

fn initialize_queue(path: &Path) -> MessageQueue {
	let (tx, rx) = crossbeam_channel::unbounded::<LogMessage>();
	crate::writer::spawn_log_thread(path.to_path_buf(), rx);
	tx
}
