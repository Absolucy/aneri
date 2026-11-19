// SPDX-License-Identifier: MPL-2.0
use meowtonin::byond_fn;
use uuid::Uuid;

/// A random UUID, like `67e55044-10b1-426f-9247-bb680e5fe0c8`.
#[byond_fn]
pub fn uuid_v4() -> String {
	Uuid::new_v4().to_string()
}

/// A UUID that starts with the current time,
/// so newer ones sort after older ones.
#[byond_fn]
pub fn uuid_v7() -> String {
	Uuid::now_v7().to_string()
}

/// A random, URL-safe id made of lowercase letters and digits,
/// always starting with a letter.
#[byond_fn]
pub fn cuid2(
	/// How many characters.
	/// Null or 0 uses the default of 24.
	length: Option<u16>,
) -> String {
	match length {
		Some(length) if length > 0 => cuid2::CuidConstructor::new()
			.with_length(length)
			.create_id(),
		_ => cuid2::create_id(),
	}
}
