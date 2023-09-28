// SPDX-License-Identifier: MPL-2.0
use uuid::Uuid;

#[byond_fn]
pub fn uuid() -> String {
	Uuid::new_v4().to_string()
}
