// SPDX-License-Identifier: MPL-2.0

use crate::message::LogMessage;
use ahash::AHashMap;
use crossbeam_channel::Sender;
use parking_lot::Mutex;
use std::{
	path::{Path, PathBuf},
	sync::{
		atomic::{AtomicUsize, Ordering},
		OnceLock,
	},
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

static MESSAGE_QUEUE: OnceLock<Mutex<LogQueues>> = OnceLock::new();

/// Clear the log message queue for a given path.
/// Any existing log messages will still be written to the log file.
pub fn clear_log_queue() {
	if let Some(queue) = MESSAGE_QUEUE.get() {
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
		.get_or_init(|| Mutex::new(AHashMap::with_capacity(DEFAULT_CAPACITY)))
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
