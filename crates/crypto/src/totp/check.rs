// SPDX-License-Identifier: MPL-2.0
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

#[byond_fn]
pub fn totp_check_sha1(
	token: String,
	secret: String,
	digits: Option<usize>,
	skew: Option<u8>,
	step: Option<u64>,
) -> Option<bool> {
	totp_check_impl(token, Algorithm::SHA1, secret, digits, skew, step)
}

#[byond_fn]
pub fn totp_check_sha256(
	token: String,
	secret: String,
	digits: Option<usize>,
	skew: Option<u8>,
	step: Option<u64>,
) -> Option<bool> {
	totp_check_impl(token, Algorithm::SHA256, secret, digits, skew, step)
}

#[byond_fn]
pub fn totp_check_sha512(
	token: String,
	secret: String,
	digits: Option<usize>,
	skew: Option<u8>,
	step: Option<u64>,
) -> Option<bool> {
	totp_check_impl(token, Algorithm::SHA512, secret, digits, skew, step)
}
