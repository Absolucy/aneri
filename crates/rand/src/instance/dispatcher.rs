// SPDX-License-Identifier: MPL-2.0
use rand::{
	Rng, SeedableRng, TryRng,
	rngs::{SmallRng, StdRng},
};
use std::convert::Infallible;

pub(crate) enum RngDispatcher {
	Secure(Box<StdRng>),
	Fast(SmallRng),
}

impl RngDispatcher {
	pub fn secure(seed: Option<u32>) -> Self {
		match seed {
			Some(seed) => Self::Secure(Box::new(StdRng::seed_from_u64(seed as u64))),
			None => Self::Secure(Box::new(rand::make_rng())),
		}
	}

	pub fn fast(seed: Option<u32>) -> Self {
		match seed {
			Some(seed) => Self::Fast(SmallRng::seed_from_u64(seed as u64)),
			None => Self::Fast(rand::make_rng()),
		}
	}
}

impl TryRng for RngDispatcher {
	type Error = Infallible;

	fn try_next_u32(&mut self) -> Result<u32, Infallible> {
		match self {
			Self::Secure(rng) => Ok(rng.next_u32()),
			Self::Fast(rng) => Ok(rng.next_u32()),
		}
	}

	fn try_next_u64(&mut self) -> Result<u64, Infallible> {
		match self {
			Self::Secure(rng) => Ok(rng.next_u64()),
			Self::Fast(rng) => Ok(rng.next_u64()),
		}
	}

	fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Infallible> {
		match self {
			Self::Secure(rng) => rng.fill_bytes(dest),
			Self::Fast(rng) => rng.fill_bytes(dest),
		}
		Ok(())
	}
}
