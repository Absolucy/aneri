// SPDX-License-Identifier: MPL-2.0

#[byond_fn]
pub fn json_is_valid(json: String) -> bool {
	serde_json::from_str::<serde_json::Value>(&json).is_ok()
}
