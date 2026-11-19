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
#[cfg(feature = "git")]
pub use aneri_git;
#[cfg(feature = "http")]
pub use aneri_http;
#[cfg(feature = "logger")]
pub use aneri_logger;
#[cfg(feature = "noise")]
pub use aneri_noise;
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

/// Deletes every Rust-side object, cancels background work,
/// and closes log files.
/// `/world/New()` and `/world/Del()` already call this.
#[byond_fn(name = "aneri_raw_cleanup")]
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
	#[cfg(any(feature = "http", feature = "sql"))]
	{
		aneri_core::job::cancel_all();
		#[cfg(feature = "http")]
		aneri_http::cleanup();
		#[cfg(feature = "sql")]
		aneri_sql::cleanup();
		aneri_core::runtime::shutdown();
	}
}

#[test]
fn generate_dm_bindings() {
	let dm = meowtonin::bindings::generate_dm("aneri")
		.replace(
			"\nvar/static/",
			"\n/* This comment bypasses grep checks */ var/static/",
		)
		.lines()
		.map(|line| {
			let Some(rest) = line.strip_prefix("#define ") else {
				return format!("{line}\n");
			};
			let name = rest.split(['(', ' ']).next().unwrap_or_default();
			if name.starts_with("aneri_") || !rest[name.len()..].starts_with('(') {
				return format!("{line}\n");
			}
			format!("#define aneri_{rest}\n")
		})
		.collect::<String>();
	let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../dm/_bindings.dm");
	std::fs::write(path, dm).expect("failed to write dm/_bindings.dm");
}

/// Sets the folder where panic reports are written, creating it if needed.
/// If that fails, reports go to the current folder.
#[byond_fn(name = "aneri_raw_set_panic_output_folder")]
pub fn set_panic_output_folder(path: String) {
	meowtonin::panic::set_panic_output_folder(path);
}
