// SPDX-License-Identifier: MPL-2.0
#![warn(
	clippy::correctness,
	clippy::suspicious,
	clippy::complexity,
	clippy::perf,
	clippy::style
)]

pub mod key;
#[cfg(feature = "runtime")]
pub mod runtime;

pub use key::ByondSlotKey;
#[cfg(feature = "runtime")]
pub use runtime::RUNTIME;
#[cfg(feature = "runtime")]
pub use tokio;
