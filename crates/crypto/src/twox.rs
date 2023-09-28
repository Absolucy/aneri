// SPDX-License-Identifier: MPL-2.0
use std::{cell::RefCell, path::PathBuf};
use xxhash_rust::{xxh3::Xxh3, xxh32::Xxh32, xxh64::Xxh64};

thread_local! {
	static XXH32: RefCell<Xxh32> = RefCell::new(Xxh32::new(0));
	static XXH64: RefCell<Xxh64> = RefCell::new(Xxh64::new(0));
	static XXH3: RefCell<Xxh3> = RefCell::new(Xxh3::new());
}

#[byond_fn]
pub fn xxh32(data: String, seed: Option<u32>) -> String {
	XXH32.with_borrow_mut(|xxh| {
		xxh.reset(seed.unwrap_or(0));
		xxh.update(data.as_bytes());
		let hash = xxh.digest();
		faster_hex::hex_string(&hash.to_le_bytes())
	})
}

#[byond_fn]
pub fn xxh32_file(path: PathBuf, seed: Option<u32>) -> Option<String> {
	std::fs::read(path).ok().map(|contents| {
		XXH32.with_borrow_mut(|xxh| {
			xxh.reset(seed.unwrap_or(0));
			xxh.update(&contents);
			let hash = xxh.digest();
			faster_hex::hex_string(&hash.to_le_bytes())
		})
	})
}

#[byond_fn]
pub fn xxh64(data: String, seed: Option<u64>) -> String {
	XXH64.with_borrow_mut(|xxh| {
		xxh.reset(seed.unwrap_or(0));
		xxh.update(data.as_bytes());
		let hash = xxh.digest();
		faster_hex::hex_string(&hash.to_le_bytes())
	})
}

#[byond_fn]
pub fn xxh64_file(path: PathBuf, seed: Option<u64>) -> Option<String> {
	std::fs::read(path).ok().map(|contents| {
		XXH64.with_borrow_mut(|xxh| {
			xxh.reset(seed.unwrap_or(0));
			xxh.update(&contents);
			let hash = xxh.digest();
			faster_hex::hex_string(&hash.to_le_bytes())
		})
	})
}

#[byond_fn]
pub fn xxh3(data: String) -> String {
	XXH3.with_borrow_mut(|xxh| {
		xxh.update(data.as_bytes());
		let hash = xxh.digest();
		xxh.reset();
		faster_hex::hex_string(&hash.to_le_bytes())
	})
}

#[byond_fn]
pub fn xxh3_file(path: PathBuf) -> Option<String> {
	std::fs::read(path).ok().map(|contents| {
		XXH3.with_borrow_mut(|xxh| {
			xxh.update(&contents);
			let hash = xxh.digest();
			xxh.reset();
			faster_hex::hex_string(&hash.to_le_bytes())
		})
	})
}
