// SPDX-License-Identifier: MPL-2.0
#[byond_fn]
pub fn url_encode(data: String) -> String {
	form_urlencoded::byte_serialize(data.as_bytes()).collect()
}

#[byond_fn]
pub fn url_decode(encoded: String) -> String {
	let mut encoded = encoded.into_bytes();
	// Replace any '+' characters with spaces.
	encoded.iter_mut().for_each(|byte| {
		if *byte == b'+' {
			*byte = b' ';
		}
	});
	percent_encoding::percent_decode(&encoded)
		.decode_utf8_lossy()
		.into_owned()
}
