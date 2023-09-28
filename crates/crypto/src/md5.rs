// SPDX-License-Identifier: MPL-2.0
use md5::{Digest, Md5};
use std::{cell::RefCell, path::PathBuf};

thread_local! {
	static MD5: RefCell<Md5> = RefCell::new(Md5::new());
}

#[byond_fn]
pub fn md5(data: String) -> String {
	crate::digest_hash(&MD5, data)
}

#[byond_fn]
pub fn md5_file(path: PathBuf) -> Option<String> {
	std::fs::read(path)
		.ok()
		.map(|contents| crate::digest_hash(&MD5, contents))
}
