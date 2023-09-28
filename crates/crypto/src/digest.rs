// SPDX-License-Identifier: MPL-2.0
use ::digest::{FixedOutputReset, Update};
use std::{cell::RefCell, thread::LocalKey};

pub(crate) fn digest_hash<Hasher, Bytes>(
	local_hasher: &'static LocalKey<RefCell<Hasher>>,
	input: Bytes,
) -> String
where
	Hasher: Update + FixedOutputReset,
	Bytes: AsRef<[u8]>,
{
	local_hasher.with_borrow_mut(|hasher| {
		hasher.update(input.as_ref());
		let hash = hasher.finalize_fixed_reset();
		faster_hex::hex_string(&hash)
	})
}
