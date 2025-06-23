// SPDX-License-Identifier: MPL-2.0
use biski64::Biski64Rng;
use rand::{RngCore, SeedableRng, rngs::StdRng};

pub(crate) enum RngDispatcher {
	Secure(Box<StdRng>),
	Fast(Biski64Rng),
}

impl RngDispatcher {
	pub fn secure(seed: Option<u32>) -> Self {
		match seed {
			Some(seed) => Self::Secure(Box::new(StdRng::seed_from_u64(seed as u64))),
			None => Self::Secure(Box::new(StdRng::from_os_rng())),
		}
	}

	pub fn fast(seed: Option<u32>) -> Self {
		match seed {
			Some(seed) => Self::Fast(Biski64Rng::seed_from_u64(seed as u64)),
			None => Self::Fast(Biski64Rng::from_os_rng()),
		}
	}
}

impl RngCore for RngDispatcher {
	fn next_u32(&mut self) -> u32 {
		match self {
			Self::Secure(rng) => rng.next_u32(),
			Self::Fast(rng) => rng.next_u32(),
		}
	}

	fn next_u64(&mut self) -> u64 {
		match self {
			Self::Secure(rng) => rng.next_u64(),
			Self::Fast(rng) => rng.next_u64(),
		}
	}

	fn fill_bytes(&mut self, dest: &mut [u8]) {
		match self {
			Self::Secure(rng) => rng.fill_bytes(dest),
			Self::Fast(rng) => rng.fill_bytes(dest),
		}
	}
}
