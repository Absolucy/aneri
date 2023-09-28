// SPDX-License-Identifier: MPL-2.0
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

#[byond_fn]
pub fn totp_gen_sha1(
	secret: String,
	digits: Option<usize>,
	skew: Option<u8>,
	step: Option<u64>,
) -> String {
	totp_gen_impl(Algorithm::SHA1, secret, digits, skew, step).unwrap_or_default()
}

#[byond_fn]
pub fn totp_gen_sha256(
	secret: String,
	digits: Option<usize>,
	skew: Option<u8>,
	step: Option<u64>,
) -> String {
	totp_gen_impl(Algorithm::SHA256, secret, digits, skew, step).unwrap_or_default()
}

#[byond_fn]
pub fn totp_gen_sha512(
	secret: String,
	digits: Option<usize>,
	skew: Option<u8>,
	step: Option<u64>,
) -> String {
	totp_gen_impl(Algorithm::SHA512, secret, digits, skew, step).unwrap_or_default()
}
