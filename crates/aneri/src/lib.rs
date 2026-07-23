// SPDX-License-Identifier: MPL-2.0

pub mod info;

#[cfg(feature = "audio")]
pub use aneri_audio;
#[cfg(feature = "crypto")]
pub use aneri_crypto;
#[cfg(feature = "dmi")]
pub use aneri_dmi;
#[cfg(feature = "encode")]
pub use aneri_encode;
#[cfg(feature = "file")]
pub use aneri_file;
#[cfg(feature = "http")]
pub use aneri_http;
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

use meowtonin::byond_fn;

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
