// SPDX-License-Identifier: MPL-2.0
use rand::{RngCore, SeedableRng};
use rand_blake3::Rng as Blake3Rand;
use rand_wyrand::WyRand;

#[cfg_attr(target_pointer_width = "32", repr(align(64)))]
#[cfg_attr(target_pointer_width = "64", repr(align(128)))]
pub(crate) enum RngDispatcher {
	Blake3(Blake3Rand),
	WyRand(WyRand),
}

impl RngDispatcher {
	#[inline]
	pub fn blake3(seed: Option<u32>) -> Self {
		match seed {
			Some(seed) => Self::Blake3(Blake3Rand::seed_from_u64(seed as u64)),
			None => Self::Blake3(Blake3Rand::from_entropy()),
		}
	}

	#[inline]
	pub fn wyrand(seed: Option<u32>) -> Self {
		match seed {
			Some(seed) => Self::WyRand(WyRand::seed_from_u64(seed as u64)),
			None => Self::WyRand(WyRand::from_entropy()),
		}
	}
}

impl RngCore for RngDispatcher {
	#[inline]
	fn next_u32(&mut self) -> u32 {
		match self {
			Self::Blake3(rng) => rng.next_u32(),
			Self::WyRand(rng) => rng.next_u32(),
		}
	}

	#[inline]
	fn next_u64(&mut self) -> u64 {
		match self {
			Self::Blake3(rng) => rng.next_u64(),
			Self::WyRand(rng) => rng.next_u64(),
		}
	}

	#[inline]
	fn fill_bytes(&mut self, dest: &mut [u8]) {
		match self {
			Self::Blake3(rng) => rng.fill_bytes(dest),
			Self::WyRand(rng) => rng.fill_bytes(dest),
		}
	}

	#[inline]
	fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
		match self {
			Self::Blake3(rng) => rng.try_fill_bytes(dest),
			Self::WyRand(rng) => rng.try_fill_bytes(dest),
		}
	}
}
