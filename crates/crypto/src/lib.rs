// SPDX-License-Identifier: MPL-2.0

pub mod totp;

use ::digest::{FixedOutputReset, Update};
use blake3::Hasher as Blake3;
use md5::{Digest, Md5};
use meowtonin::byond_fn;
use parking_lot::Mutex;
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use std::{fs::File, io::Read, path::PathBuf, sync::LazyLock};
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

const FILE_CHUNK_SIZE: usize = 64 * 1024;

/// Hashes text and returns the hash as lowercase hex.
/// For `xxh*` and `crc32*` the bytes are little-endian,
/// so the hex is reversed compared to most other tools.
/// Returns null for an unknown algorithm.
#[byond_fn]
pub fn hash(
	/// `md5`, `sha1`, `sha224`, `sha256` (or `sha2`), `sha384`, `sha512`,
	/// `sha512_224`, `sha512_256`,
	/// `sha3_224`, `sha3_256` (or `sha3`), `sha3_384`, `sha3_512`,
	/// `blake3`, `xxh32`, `xxh64`, `xxh3`, `crc32` or `crc32c`.
	algorithm: String,
	data: String,
) -> Option<String> {
	hash_data(&algorithm, Input::Bytes(data.as_bytes()))
}

/// Hashes a file and returns the hash as lowercase hex, like `aneri_hash`.
/// Reads the file in chunks, so big files are fine.
/// Returns null for an unknown algorithm or a file that can't be read.
#[byond_fn]
pub fn hash_file(
	/// Same names as `aneri_hash`.
	algorithm: String,
	file: PathBuf,
) -> Option<String> {
	let file = File::open(file).ok()?;
	hash_data(&algorithm, Input::File(file))
}

enum Input<'a> {
	Bytes(&'a [u8]),
	File(File),
}

impl Input<'_> {
	fn feed(self, mut update: impl FnMut(&[u8])) -> Option<()> {
		match self {
			Self::Bytes(bytes) => update(bytes),
			Self::File(mut file) => {
				let mut chunk = vec![0_u8; FILE_CHUNK_SIZE];
				loop {
					let bytes_read = file.read(&mut chunk).ok()?;
					if bytes_read == 0 {
						break;
					}
					update(&chunk[..bytes_read]);
				}
			}
		}
		Some(())
	}
}

fn hash_data(algorithm: &str, input: Input) -> Option<String> {
	match algorithm {
		"blake3" => digest_hash(&BLAKE3, input),
		"md5" => digest_hash(&MD5, input),
		"sha1" => digest_hash(&SHA1, input),
		"sha224" => digest_hash(&SHA224, input),
		"sha2" | "sha256" => digest_hash(&SHA256, input),
		"sha384" => digest_hash(&SHA384, input),
		"sha512" => digest_hash(&SHA512, input),
		"sha512_224" => digest_hash(&SHA512_224, input),
		"sha512_256" => digest_hash(&SHA512_256, input),
		"sha3_224" => digest_hash(&SHA3_224, input),
		"sha3" | "sha3_256" => digest_hash(&SHA3_256, input),
		"sha3_384" => digest_hash(&SHA3_384, input),
		"sha3_512" => digest_hash(&SHA3_512, input),
		"xxh32" => {
			let mut xxh = XXH32.lock();
			xxh.reset(0);
			input.feed(|chunk| xxh.update(chunk))?;
			let hash = xxh.digest();
			Some(faster_hex::hex_string(&hash.to_le_bytes()))
		}
		"xxh64" => {
			let mut xxh = XXH64.lock();
			xxh.reset(0);
			input.feed(|chunk| xxh.update(chunk))?;
			let hash = xxh.digest();
			Some(faster_hex::hex_string(&hash.to_le_bytes()))
		}
		"xxh3" => {
			let mut xxh = XXH3.lock();
			xxh.reset();
			input.feed(|chunk| xxh.update(chunk))?;
			let hash = xxh.digest();
			Some(faster_hex::hex_string(&hash.to_le_bytes()))
		}
		"crc32" => {
			let mut crc = crc32fast::Hasher::new();
			input.feed(|chunk| crc.update(chunk))?;
			Some(faster_hex::hex_string(&crc.finalize().to_le_bytes()))
		}
		"crc32c" => {
			let mut crc = 0_u32;
			input.feed(|chunk| crc = crc32c::crc32c_append(crc, chunk))?;
			Some(faster_hex::hex_string(&crc.to_le_bytes()))
		}
		_ => None,
	}
}

pub(crate) fn digest_hash<Hasher>(
	lazy_hasher: &'static LazyLock<Mutex<Hasher>>,
	input: Input,
) -> Option<String>
where
	Hasher: Update + FixedOutputReset,
{
	let mut hasher = lazy_hasher.lock();
	let fed = input.feed(|chunk| hasher.update(chunk));
	let hash = hasher.finalize_fixed_reset();
	fed.map(|()| faster_hex::hex_string(&hash))
}
