// SPDX-License-Identifier: MPL-2.0
use data_encoding::{BASE64, BASE64URL};
use meowtonin::byond_fn;

/// Encodes text as standard base64, with `=` padding.
#[byond_fn]
pub fn base64_encode(data: String) -> String {
	BASE64.encode(data.as_bytes())
}

/// Decodes standard base64 (padding required) into text.
/// Bytes that aren't valid UTF-8 are replaced, so binary data won't survive.
/// Returns null if the input isn't valid base64.
#[byond_fn]
pub fn base64_decode(data: String) -> Option<String> {
	BASE64
		.decode(data.trim().as_bytes())
		.ok()
		.map(|decoded| String::from_utf8_lossy(&decoded).into_owned())
}

/// Encodes text as URL-safe base64, with `=` padding.
/// It uses `-` and `_` instead of `+` and `/`.
#[byond_fn]
pub fn base64url_encode(data: String) -> String {
	BASE64URL.encode(data.as_bytes())
}

/// Decodes URL-safe base64 (padding required) into text.
/// Bytes that aren't valid UTF-8 are replaced, so binary data won't survive.
/// Returns null if the input isn't valid base64.
#[byond_fn]
pub fn base64url_decode(data: String) -> Option<String> {
	BASE64URL
		.decode(data.trim().as_bytes())
		.ok()
		.map(|decoded| String::from_utf8_lossy(&decoded).into_owned())
}
