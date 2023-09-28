// SPDX-License-Identifier: MPL-2.0
use blake3::Hasher as Blake3;
use std::{cell::RefCell, path::PathBuf};

thread_local! {
	static BLAKE3: RefCell<Blake3> = RefCell::new(Blake3::new());
}

#[byond_fn]
pub fn blake3(data: String) -> String {
	BLAKE3.with_borrow_mut(|blake3| {
		blake3.update(data.as_bytes());
		let hash = blake3.finalize();
		blake3.reset();
		faster_hex::hex_string(hash.as_bytes())
	})
}

#[byond_fn]
pub fn blake3_file(path: PathBuf) -> Option<String> {
	std::fs::read(path).ok().map(|contents| {
		BLAKE3.with_borrow_mut(|blake3| {
			blake3.update(&contents);
			let hash = blake3.finalize();
			blake3.reset();
			faster_hex::hex_string(hash.as_bytes())
		})
	})
}
