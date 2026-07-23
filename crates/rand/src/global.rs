// SPDX-License-Identifier: MPL-2.0
pub mod dispatcher;
pub mod number;
pub mod pick;
pub mod prob;
pub mod string;

use self::dispatcher::GlobalRngDispatcher;
use parking_lot::Mutex;
use rand::{
	SeedableRng,
	rngs::{SmallRng, StdRng},
};
use std::sync::LazyLock;

static FAST: LazyLock<Mutex<SmallRng>> =
	LazyLock::new(|| Mutex::new(SmallRng::from_rng(&mut rand::rng())));
static SECURE: LazyLock<Mutex<StdRng>> =
	LazyLock::new(|| Mutex::new(StdRng::from_rng(&mut rand::rng())));

pub(crate) fn reseed_global_rng() {
	*FAST.lock() = SmallRng::from_rng(&mut rand::rng());
	*SECURE.lock() = StdRng::from_rng(&mut rand::rng());
}

pub(crate) fn global(secure: impl Into<Option<bool>>) -> GlobalRngDispatcher {
	if secure.into().unwrap_or(false) {
		GlobalRngDispatcher::Secure(&SECURE)
	} else {
		GlobalRngDispatcher::Fast(&FAST)
	}
}
