// SPDX-License-Identifier: MPL-2.0
use biski64::Biski64Rng;
use parking_lot::Mutex;
use rand::{RngCore, rngs::StdRng};

pub(crate) enum GlobalRngDispatcher {
	Secure(&'static Mutex<StdRng>),
	Fast(&'static Mutex<Biski64Rng>),
}

impl RngCore for GlobalRngDispatcher {
	fn next_u32(&mut self) -> u32 {
		match *self {
			Self::Secure(rng) => rng.lock().next_u32(),
			Self::Fast(rng) => rng.lock().next_u32(),
		}
	}

	fn next_u64(&mut self) -> u64 {
		match *self {
			Self::Secure(rng) => rng.lock().next_u64(),
			Self::Fast(rng) => rng.lock().next_u64(),
		}
	}

	fn fill_bytes(&mut self, dest: &mut [u8]) {
		match *self {
			Self::Secure(rng) => rng.lock().fill_bytes(dest),
			Self::Fast(rng) => rng.lock().fill_bytes(dest),
		}
	}
}
