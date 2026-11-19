// SPDX-License-Identifier: MPL-2.0
use meowtonin::byond_fn;
use totp_rs::Algorithm;

fn totp_check_impl(
	token: String,
	algorithm: Algorithm,
	secret: String,
	digits: Option<usize>,
	skew: Option<u8>,
	step: Option<u64>,
) -> Option<bool> {
	super::init_totp(algorithm, secret, digits, skew, step)
		.and_then(|totp| totp.check_current(&token).ok())
}

/// Checks a TOTP code against a secret, using HMAC-SHA1.
/// This is the one most authenticator apps use.
/// Returns null if the secret or digit count is invalid.
#[byond_fn]
pub fn totp_check_sha1(
	/// The code the user typed.
	token: String,
	/// Base32 secret, at least 128 bits (26 characters).
	secret: String,
	/// Code length from 6 to 8, default 6.
	digits: Option<usize>,
	/// How many steps before or after now still count, default 1.
	/// Covers clock drift.
	skew: Option<u8>,
	/// Seconds each code lasts, default 30.
	step: Option<u64>,
) -> Option<bool> {
	totp_check_impl(token, Algorithm::SHA1, secret, digits, skew, step)
}

/// Checks a TOTP code against a secret, using HMAC-SHA256.
/// Returns null if the secret or digit count is invalid.
#[byond_fn]
pub fn totp_check_sha256(
	/// The code the user typed.
	token: String,
	/// Base32 secret, at least 128 bits (26 characters).
	secret: String,
	/// Code length from 6 to 8, default 6.
	digits: Option<usize>,
	/// How many steps before or after now still count, default 1.
	/// Covers clock drift.
	skew: Option<u8>,
	/// Seconds each code lasts, default 30.
	step: Option<u64>,
) -> Option<bool> {
	totp_check_impl(token, Algorithm::SHA256, secret, digits, skew, step)
}

/// Checks a TOTP code against a secret, using HMAC-SHA512.
/// Returns null if the secret or digit count is invalid.
#[byond_fn]
pub fn totp_check_sha512(
	/// The code the user typed.
	token: String,
	/// Base32 secret, at least 128 bits (26 characters).
	secret: String,
	/// Code length from 6 to 8, default 6.
	digits: Option<usize>,
	/// How many steps before or after now still count, default 1.
	/// Covers clock drift.
	skew: Option<u8>,
	/// Seconds each code lasts, default 30.
	step: Option<u64>,
) -> Option<bool> {
	totp_check_impl(token, Algorithm::SHA512, secret, digits, skew, step)
}
