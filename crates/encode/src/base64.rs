// SPDX-License-Identifier: MPL-2.0
use data_encoding::{BASE64, BASE64URL};

#[byond_fn]
pub fn base64_encode(data: String) -> String {
	BASE64.encode(data.as_bytes())
}

#[byond_fn]
pub fn base64_decode(data: String) -> Option<String> {
	BASE64
		.decode(data.trim().as_bytes())
		.ok()
		.map(|decoded| String::from_utf8_lossy(&decoded).into_owned())
}

#[byond_fn]
pub fn base64url_encode(data: String) -> String {
	BASE64URL.encode(data.as_bytes())
}

#[byond_fn]
pub fn base64url_decode(data: String) -> Option<String> {
	BASE64URL
		.decode(data.trim().as_bytes())
		.ok()
		.map(|decoded| String::from_utf8_lossy(&decoded).into_owned())
}
