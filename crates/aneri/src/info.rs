// SPDX-License-Identifier: MPL-2.0
use const_format::formatcp as const_format;
use meowtonin::byond_fn;

#[byond_fn]
pub fn aneri_version() -> &'static str {
	const_format!(
		"{name} v{version} ({git_hash})",
		name = env!("CARGO_PKG_NAME"),
		version = env!("CARGO_PKG_VERSION"),
		git_hash = env!("BOSION_GIT_COMMIT_SHORTHASH")
	)
}

#[byond_fn]
pub fn aneri_features() -> &'static str {
	env!("BOSION_CRATE_FEATURES")
}
