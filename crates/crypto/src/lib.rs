// SPDX-License-Identifier: MPL-2.0
#![warn(
	clippy::correctness,
	clippy::suspicious,
	clippy::complexity,
	clippy::perf,
	clippy::style
)]
#[macro_use]
extern crate meowtonin;

use ::digest::{FixedOutputReset, Update};
use blake3::Hasher as Blake3;
use md5::{Digest, Md5};
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use std::{cell::RefCell, path::PathBuf, thread::LocalKey};
use xxhash_rust::{xxh3::Xxh3, xxh32::Xxh32, xxh64::Xxh64};

#[byond_fn]
pub fn hash(algorithm: String, data: String) -> Option<String> {
	hash_data(algorithm, data)
}

#[byond_fn]
pub fn hash_file(algorithm: String, file: PathBuf) -> Option<String> {
	let data = std::fs::read(file).ok()?;
	hash_data(algorithm, data)
}

thread_local! {
	static BLAKE3: RefCell<Blake3> = RefCell::new(Blake3::new());
	static MD5: RefCell<Md5> = RefCell::new(Md5::new());
	static SHA1: RefCell<Sha1> = RefCell::new(Sha1::new());
	static SHA224: RefCell<Sha224> = RefCell::new(Sha224::new());
	static SHA256: RefCell<Sha256> = RefCell::new(Sha256::new());
	static SHA384: RefCell<Sha384> = RefCell::new(Sha384::new());
	static SHA512: RefCell<Sha512> = RefCell::new(Sha512::new());
	static SHA512_224: RefCell<Sha512_224> = RefCell::new(Sha512_224::new());
	static SHA512_256: RefCell<Sha512_256> = RefCell::new(Sha512_256::new());
	static SHA3_224: RefCell<Sha3_224> = RefCell::new(Sha3_224::new());
	static SHA3_256: RefCell<Sha3_256> = RefCell::new(Sha3_256::new());
	static SHA3_384: RefCell<Sha3_384> = RefCell::new(Sha3_384::new());
	static SHA3_512: RefCell<Sha3_512> = RefCell::new(Sha3_512::new());
	static XXH32: RefCell<Xxh32> = RefCell::new(Xxh32::new(0));
	static XXH64: RefCell<Xxh64> = RefCell::new(Xxh64::new(0));
	static XXH3: RefCell<Xxh3> = RefCell::new(Xxh3::new());
}

fn hash_data(algorithm: impl AsRef<str>, data: impl AsRef<[u8]>) -> Option<String> {
	let algorithm = algorithm.as_ref();
	let data = data.as_ref();
	match algorithm {
		"blake3" => Some(digest_hash(&BLAKE3, data)),
		"md5" => Some(digest_hash(&MD5, data)),
		"sha1" => Some(digest_hash(&SHA1, data)),
		"sha224" => Some(digest_hash(&SHA224, data)),
		"sha2" | "sha256" => Some(digest_hash(&SHA256, data)),
		"sha384" => Some(digest_hash(&SHA384, data)),
		"sha512" => Some(digest_hash(&SHA512, data)),
		"sha512_224" => Some(digest_hash(&SHA512_224, data)),
		"sha512_256" => Some(digest_hash(&SHA512_256, data)),
		"sha3_224" => Some(digest_hash(&SHA3_224, data)),
		"sha3" | "sha3_256" => Some(digest_hash(&SHA3_256, data)),
		"sha3_384" => Some(digest_hash(&SHA3_384, data)),
		"sha3_512" => Some(digest_hash(&SHA3_512, data)),
		"xxh32" => XXH32.with_borrow_mut(|xxh| {
			xxh.reset(0);
			xxh.update(data);
			let hash = xxh.digest();
			Some(faster_hex::hex_string(&hash.to_le_bytes()))
		}),
		"xxh64" => XXH64.with_borrow_mut(|xxh| {
			xxh.reset(0);
			xxh.update(data);
			let hash = xxh.digest();
			Some(faster_hex::hex_string(&hash.to_le_bytes()))
		}),
		"xxh3" => XXH3.with_borrow_mut(|xxh| {
			xxh.reset();
			xxh.update(data);
			let hash = xxh.digest();
			Some(faster_hex::hex_string(&hash.to_le_bytes()))
		}),
		"crc32" => Some(faster_hex::hex_string(&crc32fast::hash(data).to_le_bytes())),
		"crc32c" => Some(faster_hex::hex_string(&crc32c::crc32c(data).to_le_bytes())),
		_ => None,
	}
}

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
