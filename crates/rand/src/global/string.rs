// SPDX-License-Identifier: MPL-2.0
use super::global;
use rand::{distributions::Alphanumeric, Rng};

#[byond_fn]
pub fn random_string_alphanumeric(length: usize, secure: Option<bool>) -> String {
	let mut rng = global(secure);
	(0..=length)
		.map(|_| rng.sample(Alphanumeric) as char)
		.collect()
}
