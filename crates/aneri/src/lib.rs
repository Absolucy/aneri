// SPDX-License-Identifier: MPL-2.0
#![warn(
	clippy::correctness,
	clippy::suspicious,
	clippy::complexity,
	clippy::perf,
	clippy::style
)]
#[macro_use]
extern crate meowtonin;

#[cfg(feature = "snmalloc")]
#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

pub mod info;

#[cfg(feature = "crypto")]
pub use aneri_crypto;
#[cfg(feature = "dmi")]
pub use aneri_dmi;
#[cfg(feature = "encode")]
pub use aneri_encode;
#[cfg(feature = "file")]
pub use aneri_file;
#[cfg(feature = "logger")]
pub use aneri_logger;
#[cfg(feature = "rand")]
pub use aneri_rand;
#[cfg(feature = "regex")]
pub use aneri_regex;
#[cfg(feature = "sort")]
pub use aneri_sort;
#[cfg(feature = "sql")]
pub use aneri_sql;
#[cfg(feature = "time")]
pub use aneri_time;
#[cfg(feature = "util")]
pub use aneri_util;
/// Cleans up any resources used by Aneri.
/// This should be run on initialization and shutdown.
#[byond_fn]
pub fn cleanup() {
	#[cfg(feature = "logger")]
	aneri_logger::queue::clear_log_queue();
	#[cfg(feature = "rand")]
	aneri_rand::cleanup();
	#[cfg(feature = "regex")]
	aneri_regex::cleanup();
	#[cfg(feature = "time")]
	aneri_time::cleanup();
	#[cfg(feature = "util")]
	aneri_util::cleanup();
}

#[byond_fn]
pub fn set_panic_output_folder(path: String) {
	meowtonin::panic::set_panic_output_folder(path);
}
