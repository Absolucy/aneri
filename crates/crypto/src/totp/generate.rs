// SPDX-License-Identifier: MPL-2.0
use meowtonin::byond_fn;
use totp_rs::Algorithm;

fn totp_gen_impl(
	algorithm: Algorithm,
	secret: String,
	digits: Option<usize>,
	skew: Option<u8>,
	step: Option<u64>,
) -> Option<String> {
	super::init_totp(algorithm, secret, digits, skew, step)
		.and_then(|totp| totp.generate_current().ok())
}

/// The current TOTP code for a secret, using HMAC-SHA1.
/// This is the one most authenticator apps expect.
/// Returns an empty string if the secret or digit count is invalid.
#[byond_fn]
pub fn totp_gen_sha1(
	/// Base32 secret, at least 128 bits (26 characters).
	secret: String,
	/// Code length from 6 to 8, default 6.
	digits: Option<usize>,
	/// Unused when generating.
	skew: Option<u8>,
	/// Seconds each code lasts, default 30.
	step: Option<u64>,
) -> String {
	totp_gen_impl(Algorithm::SHA1, secret, digits, skew, step).unwrap_or_default()
}

/// The current TOTP code for a secret, using HMAC-SHA256.
/// Returns an empty string if the secret or digit count is invalid.
#[byond_fn]
pub fn totp_gen_sha256(
	/// Base32 secret, at least 128 bits (26 characters).
	secret: String,
	/// Code length from 6 to 8, default 6.
	digits: Option<usize>,
	/// Unused when generating.
	skew: Option<u8>,
	/// Seconds each code lasts, default 30.
	step: Option<u64>,
) -> String {
	totp_gen_impl(Algorithm::SHA256, secret, digits, skew, step).unwrap_or_default()
}

/// The current TOTP code for a secret, using HMAC-SHA512.
/// Returns an empty string if the secret or digit count is invalid.
#[byond_fn]
pub fn totp_gen_sha512(
	/// Base32 secret, at least 128 bits (26 characters).
	secret: String,
	/// Code length from 6 to 8, default 6.
	digits: Option<usize>,
	/// Unused when generating.
	skew: Option<u8>,
	/// Seconds each code lasts, default 30.
	step: Option<u64>,
) -> String {
	totp_gen_impl(Algorithm::SHA512, secret, digits, skew, step).unwrap_or_default()
}
