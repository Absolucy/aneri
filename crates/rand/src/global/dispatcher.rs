// SPDX-License-Identifier: MPL-2.0
use parking_lot::Mutex;
use rand::RngCore;
use rand_blake3::Rng as Blake3Rng;
use rand_wyrand::WyRand;

pub(crate) enum GlobalRngDispatcher {
	Blake3(&'static Mutex<Blake3Rng>),
	WyRand(&'static Mutex<WyRand>),
}

impl RngCore for GlobalRngDispatcher {
	fn next_u32(&mut self) -> u32 {
		match *self {
			Self::Blake3(rng) => rng.lock().next_u32(),
			Self::WyRand(rng) => rng.lock().next_u32(),
		}
	}

	fn next_u64(&mut self) -> u64 {
		match *self {
			Self::Blake3(rng) => rng.lock().next_u64(),
			Self::WyRand(rng) => rng.lock().next_u64(),
		}
	}

	fn fill_bytes(&mut self, dest: &mut [u8]) {
		match *self {
			Self::Blake3(rng) => rng.lock().fill_bytes(dest),
			Self::WyRand(rng) => rng.lock().fill_bytes(dest),
		}
	}

	fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
		match *self {
			Self::Blake3(rng) => rng.lock().try_fill_bytes(dest),
			Self::WyRand(rng) => rng.lock().try_fill_bytes(dest),
		}
	}
}
