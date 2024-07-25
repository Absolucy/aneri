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
use std::sync::LazyLock;

static WYRAND: LazyLock<Mutex<WyRand>> = LazyLock::new(|| Mutex::new(WyRand::from_entropy()));
static BLAKE3: LazyLock<Mutex<Blake3Rng>> = LazyLock::new(|| Mutex::new(Blake3Rng::from_entropy()));

pub(crate) fn reseed_global_rng() {
	*WYRAND.lock() = WyRand::from_entropy();
	*BLAKE3.lock() = Blake3Rng::from_entropy();
}

pub(crate) fn global(secure: impl Into<Option<bool>>) -> GlobalRngDispatcher {
	if secure.into().unwrap_or(false) {
		GlobalRngDispatcher::Blake3(&BLAKE3)
	} else {
		GlobalRngDispatcher::WyRand(&WYRAND)
	}
}
