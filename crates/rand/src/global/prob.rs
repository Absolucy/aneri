// SPDX-License-Identifier: MPL-2.0
use super::global;
use crate::shared;
use meowtonin::byond_fn;

/// Returns true `probability` percent of the time, like DM's `prob()`.
#[byond_fn(name = "aneri_rand_prob")]
pub fn prob(
	/// Chance from 0 to 100, with values outside that clamped.
	/// NaN and infinity always return true.
	probability: f64,
	/// Use the slower, cryptographically secure generator, default false.
	secure: Option<bool>,
) -> bool {
	shared::prob(&mut global(secure), probability)
}

/// Returns true `numerator` in `denominator` times,
/// so `(1, 3)` is a one in three chance.
/// Runtimes if `denominator` is 0 or smaller than `numerator`.
#[byond_fn(name = "aneri_rand_prob_ratio")]
pub fn prob_ratio(
	numerator: u32,
	denominator: u32,
	/// Use the slower, cryptographically secure generator, default false.
	secure: Option<bool>,
) -> bool {
	shared::prob_ratio(&mut global(secure), numerator, denominator)
}
