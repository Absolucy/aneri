// SPDX-License-Identifier: MPL-2.0
use parking_lot::MutexGuard;
use rand::{
	Rng, TryRng,
	rngs::{SmallRng, StdRng},
};
use std::convert::Infallible;

pub(crate) enum GlobalRngDispatcher {
	Secure(MutexGuard<'static, StdRng>),
	Fast(MutexGuard<'static, SmallRng>),
}

impl TryRng for GlobalRngDispatcher {
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
