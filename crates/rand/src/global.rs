// SPDX-License-Identifier: MPL-2.0
pub mod dispatcher;
pub mod number;
pub mod pick;
pub mod prob;
pub mod string;

use self::dispatcher::GlobalRngDispatcher;
use biski64::Biski64Rng;
use parking_lot::Mutex;
use rand::{SeedableRng, rngs::StdRng};
use std::sync::LazyLock;

static FAST: LazyLock<Mutex<Biski64Rng>> = LazyLock::new(|| Mutex::new(Biski64Rng::from_os_rng()));
static SECURE: LazyLock<Mutex<StdRng>> = LazyLock::new(|| Mutex::new(StdRng::from_os_rng()));

pub(crate) fn reseed_global_rng() {
	*FAST.lock() = Biski64Rng::from_os_rng();
	*SECURE.lock() = StdRng::from_os_rng();
}

pub(crate) fn global(secure: impl Into<Option<bool>>) -> GlobalRngDispatcher {
	if secure.into().unwrap_or(false) {
		GlobalRngDispatcher::Secure(&SECURE)
	} else {
		GlobalRngDispatcher::Fast(&FAST)
	}
}
