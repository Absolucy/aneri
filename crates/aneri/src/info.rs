// SPDX-License-Identifier: MPL-2.0
use const_format::formatcp as const_format;
use meowtonin::byond_fn;

/// The library's name, version and git commit, like `aneri v1.2.3 (abc1234)`.
#[byond_fn(name = "aneri_raw_aneri_version")]
pub fn aneri_version() -> &'static str {
	const_format!(
		"{name} v{version} ({git_hash})",
		name = env!("CARGO_PKG_NAME"),
		version = env!("CARGO_PKG_VERSION"),
		git_hash = env!("BOSION_GIT_COMMIT_SHORTHASH")
	)
}

/// The Cargo features this build was compiled with, comma-separated.
#[byond_fn(name = "aneri_raw_aneri_features")]
pub fn aneri_features() -> &'static str {
	env!("BOSION_CRATE_FEATURES")
}
