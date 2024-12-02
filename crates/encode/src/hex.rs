// SPDX-License-Identifier: MPL-2.0
use meowtonin::byond_fn;

#[byond_fn]
pub fn hex_encode(input: String, upper: Option<bool>) -> String {
	let input = input.as_bytes();
	match upper {
		Some(true) => faster_hex::hex_string_upper(input),
		_ => faster_hex::hex_string(input),
	}
}

#[byond_fn]
pub fn hex_decode(input: String) -> Option<String> {
	let mut dst = vec![0_u8; input.len() / 2];
	faster_hex::hex_decode(input.trim().as_bytes(), &mut dst)
		.ok()
		.map(|_| String::from_utf8_lossy(&dst).into_owned())
}
