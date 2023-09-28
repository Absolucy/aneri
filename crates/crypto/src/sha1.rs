// SPDX-License-Identifier: MPL-2.0
use sha1::{Digest, Sha1};
use std::{cell::RefCell, path::PathBuf};

thread_local! {
	static SHA1: RefCell<Sha1> = RefCell::new(Sha1::new());
}

#[byond_fn]
pub fn sha1(data: String) -> String {
	crate::digest_hash(&SHA1, data)
}

#[byond_fn]
pub fn sha1_file(path: PathBuf) -> Option<String> {
	std::fs::read(path)
		.ok()
		.map(|contents| crate::digest_hash(&SHA1, contents))
}
