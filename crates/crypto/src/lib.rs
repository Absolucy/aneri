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

pub mod blake3;
//pub mod crc32;
pub mod digest;
pub mod md5;
pub mod sha1;
pub mod sha2;
pub mod sha3;
pub mod totp;
pub mod twox;

pub(crate) use crate::digest::digest_hash;
