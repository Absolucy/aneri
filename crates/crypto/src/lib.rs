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
use parking_lot::Mutex;
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use std::{path::PathBuf, sync::LazyLock};
use xxhash_rust::{xxh3::Xxh3, xxh32::Xxh32, xxh64::Xxh64};

macro_rules! impl_hasher {
	($(($name:ident, $hasher:ty, $initializer:expr)),+) => {
		$(
			impl_hasher!($name, $hasher, $initializer);
		)+
	};
	($name:ident, $hasher:ty, $initializer:expr) => {
		static $name: LazyLock<Mutex<$hasher>> = LazyLock::new(|| Mutex::new($initializer));
	};
}

impl_hasher!(
	(BLAKE3, Blake3, Blake3::new()),
	(MD5, Md5, Md5::new()),
	(SHA1, Sha1, Sha1::new()),
	(SHA224, Sha224, Sha224::new()),
	(SHA256, Sha256, Sha256::new()),
	(SHA384, Sha384, Sha384::new()),
	(SHA512, Sha512, Sha512::new()),
	(SHA512_224, Sha512_224, Sha512_224::new()),
	(SHA512_256, Sha512_256, Sha512_256::new()),
	(SHA3_224, Sha3_224, Sha3_224::new()),
	(SHA3_256, Sha3_256, Sha3_256::new()),
	(SHA3_384, Sha3_384, Sha3_384::new()),
	(SHA3_512, Sha3_512, Sha3_512::new()),
	(XXH32, Xxh32, Xxh32::new(0)),
	(XXH64, Xxh64, Xxh64::new(0)),
	(XXH3, Xxh3, Xxh3::new())
);

#[byond_fn]
pub fn hash(algorithm: String, data: String) -> Option<String> {
	hash_data(algorithm, data)
}

#[byond_fn]
pub fn hash_file(algorithm: String, file: PathBuf) -> Option<String> {
	let data = std::fs::read(file).ok()?;
	hash_data(algorithm, data)
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
		"xxh32" => {
			let mut xxh = XXH32.lock();
			xxh.reset(0);
			xxh.update(data);
			let hash = xxh.digest();
			Some(faster_hex::hex_string(&hash.to_le_bytes()))
		}
		"xxh64" => {
			let mut xxh = XXH64.lock();
			xxh.reset(0);
			xxh.update(data);
			let hash = xxh.digest();
			Some(faster_hex::hex_string(&hash.to_le_bytes()))
		}
		"xxh3" => {
			let mut xxh = XXH3.lock();
			xxh.reset();
			xxh.update(data);
			let hash = xxh.digest();
			Some(faster_hex::hex_string(&hash.to_le_bytes()))
		}
		"crc32" => Some(faster_hex::hex_string(&crc32fast::hash(data).to_le_bytes())),
		"crc32c" => Some(faster_hex::hex_string(&crc32c::crc32c(data).to_le_bytes())),
		_ => None,
	}
}

pub(crate) fn digest_hash<Hasher, Bytes>(
	lazy_hasher: &'static LazyLock<Mutex<Hasher>>,
	input: Bytes,
) -> String
where
	Hasher: Update + FixedOutputReset,
	Bytes: AsRef<[u8]>,
{
	let mut hasher = lazy_hasher.lock();
	hasher.update(input.as_ref());
	let hash = hasher.finalize_fixed_reset();
	faster_hex::hex_string(&hash)
}
