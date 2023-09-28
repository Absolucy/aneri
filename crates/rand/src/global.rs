// SPDX-License-Identifier: MPL-2.0
pub mod dispatcher;
pub mod number;
pub mod pick;
pub mod prob;
pub mod string;

use self::dispatcher::GlobalRngDispatcher;
use parking_lot::Mutex;
use rand::SeedableRng;
use rand_blake3::Rng as Blake3Rng;
use rand_wyrand::WyRand;
use std::sync::OnceLock;

static WYRAND: OnceLock<Mutex<WyRand>> = OnceLock::new();
static BLAKE3: OnceLock<Mutex<Blake3Rng>> = OnceLock::new();

pub(crate) fn reseed_global_rng() {
	if let Some(wyrand) = WYRAND.get() {
		*wyrand.lock() = WyRand::from_entropy();
	}
	if let Some(blake3) = BLAKE3.get() {
		*blake3.lock() = Blake3Rng::from_entropy();
	}
}

#[inline]
pub(crate) fn global(secure: impl Into<Option<bool>>) -> GlobalRngDispatcher {
	if secure.into().unwrap_or(false) {
		GlobalRngDispatcher::Blake3(BLAKE3.get_or_init(|| Mutex::new(Blake3Rng::from_entropy())))
	} else {
		GlobalRngDispatcher::WyRand(WYRAND.get_or_init(|| Mutex::new(WyRand::from_entropy())))
	}
}
