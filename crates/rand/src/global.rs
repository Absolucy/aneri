// SPDX-License-Identifier: MPL-2.0
pub mod dispatcher;
pub mod number;
pub mod pick;
pub mod prob;
pub mod string;

use self::dispatcher::GlobalRngDispatcher;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use rand::SeedableRng;
use rand_blake3::Rng as Blake3Rng;
use rand_wyrand::WyRand;

static WYRAND: Lazy<Mutex<WyRand>> = Lazy::new(|| Mutex::new(WyRand::from_entropy()));
static BLAKE3: Lazy<Mutex<Blake3Rng>> = Lazy::new(|| Mutex::new(Blake3Rng::from_entropy()));

pub(crate) fn reseed_global_rng() {
	if let Some(wyrand) = Lazy::get(&WYRAND) {
		*wyrand.lock() = WyRand::from_entropy();
	}
	if let Some(blake3) = Lazy::get(&BLAKE3) {
		*blake3.lock() = Blake3Rng::from_entropy();
	}
}

#[inline]
pub(crate) fn global(secure: impl Into<Option<bool>>) -> GlobalRngDispatcher {
	if secure.into().unwrap_or(false) {
		GlobalRngDispatcher::Blake3(&BLAKE3)
	} else {
		GlobalRngDispatcher::WyRand(&WYRAND)
	}
}
