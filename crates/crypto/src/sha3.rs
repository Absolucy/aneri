// SPDX-License-Identifier: MPL-2.0
use sha3::{Digest, Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use std::{cell::RefCell, path::PathBuf};

thread_local! {
	static SHA3_224: RefCell<Sha3_224> = RefCell::new(Sha3_224::new());
	static SHA3_256: RefCell<Sha3_256> = RefCell::new(Sha3_256::new());
	static SHA3_384: RefCell<Sha3_384> = RefCell::new(Sha3_384::new());
	static SHA3_512: RefCell<Sha3_512> = RefCell::new(Sha3_512::new());
}

#[byond_fn]
pub fn sha3_224(data: String) -> String {
	crate::digest_hash(&SHA3_224, data)
}

#[byond_fn]
pub fn sha3_224_file(path: PathBuf) -> Option<String> {
	std::fs::read(path)
		.ok()
		.map(|contents| crate::digest_hash(&SHA3_224, contents))
}

#[byond_fn]
pub fn sha3_256(data: String) -> String {
	crate::digest_hash(&SHA3_256, data)
}

#[byond_fn]
pub fn sha3_256_file(path: PathBuf) -> Option<String> {
	std::fs::read(path)
		.ok()
		.map(|contents| crate::digest_hash(&SHA3_256, contents))
}

#[byond_fn]
pub fn sha3_384(data: String) -> String {
	crate::digest_hash(&SHA3_384, data)
}

#[byond_fn]
pub fn sha3_384_file(path: PathBuf) -> Option<String> {
	std::fs::read(path)
		.ok()
		.map(|contents| crate::digest_hash(&SHA3_384, contents))
}

#[byond_fn]
pub fn sha3_512(data: String) -> String {
	crate::digest_hash(&SHA3_512, data)
}

#[byond_fn]
pub fn sha3_512_file(path: PathBuf) -> Option<String> {
	std::fs::read(path)
		.ok()
		.map(|contents| crate::digest_hash(&SHA3_512, contents))
}
