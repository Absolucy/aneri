// SPDX-License-Identifier: MPL-2.0
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};
use std::{cell::RefCell, path::PathBuf};

thread_local! {
	static SHA224: RefCell<Sha224> = RefCell::new(Sha224::new());
	static SHA256: RefCell<Sha256> = RefCell::new(Sha256::new());
	static SHA384: RefCell<Sha384> = RefCell::new(Sha384::new());
	static SHA512: RefCell<Sha512> = RefCell::new(Sha512::new());
	static SHA512_224: RefCell<Sha512_224> = RefCell::new(Sha512_224::new());
	static SHA512_256: RefCell<Sha512_256> = RefCell::new(Sha512_256::new());
}

#[byond_fn]
pub fn sha2_224(data: String) -> String {
	crate::digest_hash(&SHA224, data)
}

#[byond_fn]
pub fn sha2_224_file(path: PathBuf) -> Option<String> {
	std::fs::read(path)
		.ok()
		.map(|contents| crate::digest_hash(&SHA224, contents))
}

#[byond_fn]
pub fn sha2_256(data: String) -> String {
	crate::digest_hash(&SHA256, data)
}

#[byond_fn]
pub fn sha2_256_file(path: PathBuf) -> Option<String> {
	std::fs::read(path)
		.ok()
		.map(|contents| crate::digest_hash(&SHA256, contents))
}

#[byond_fn]
pub fn sha2_384(data: String) -> String {
	crate::digest_hash(&SHA384, data)
}

#[byond_fn]
pub fn sha2_384_file(path: PathBuf) -> Option<String> {
	std::fs::read(path)
		.ok()
		.map(|contents| crate::digest_hash(&SHA384, contents))
}

#[byond_fn]
pub fn sha2_512(data: String) -> String {
	crate::digest_hash(&SHA512, data)
}

#[byond_fn]
pub fn sha2_512_file(path: PathBuf) -> Option<String> {
	std::fs::read(path)
		.ok()
		.map(|contents| crate::digest_hash(&SHA512, contents))
}

#[byond_fn]
pub fn sha2_512_224(data: String) -> String {
	crate::digest_hash(&SHA512_224, data)
}

#[byond_fn]
pub fn sha2_512_224_file(path: PathBuf) -> Option<String> {
	std::fs::read(path)
		.ok()
		.map(|contents| crate::digest_hash(&SHA512_224, contents))
}

#[byond_fn]
pub fn sha2_512_256(data: String) -> String {
	crate::digest_hash(&SHA512_256, data)
}

#[byond_fn]
pub fn sha2_512_256_file(path: PathBuf) -> Option<String> {
	std::fs::read(path)
		.ok()
		.map(|contents| crate::digest_hash(&SHA512_256, contents))
}
