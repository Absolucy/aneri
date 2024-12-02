// SPDX-License-Identifier: MPL-2.0
use meowtonin::byond_fn;
use uuid::Uuid;

#[byond_fn]
pub fn uuid() -> String {
	Uuid::new_v4().to_string()
}

#[byond_fn]
pub fn cuid2(length: Option<u16>) -> String {
	match length {
		Some(length) if length > 0 => cuid2::CuidConstructor::new()
			.with_length(length)
			.create_id(),
		_ => cuid2::create_id(),
	}
}
