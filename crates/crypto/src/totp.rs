// SPDX-License-Identifier: MPL-2.0
pub mod check;
pub mod gen;

use totp_rs::{Algorithm, Secret, TOTP};

fn init_totp(
	algorithm: Algorithm,
	secret: String,
	digits: Option<usize>,
	skew: Option<u8>,
	step: Option<u64>,
) -> Option<TOTP> {
	let secret = Secret::Encoded(secret);
	TOTP::new(
		algorithm,
		digits.unwrap_or(6),
		skew.unwrap_or(1),
		step.unwrap_or(30),
		secret.to_bytes().ok()?,
	)
	.ok()
}
