// SPDX-License-Identifier: MPL-2.0
use super::global;
use rand::{
	distributions::{Alphanumeric, Bernoulli, Distribution},
	Rng,
};

#[byond_fn]
pub fn random_string_alphanumeric(length: usize, secure: Option<bool>) -> String {
	let mut rng = global(secure);
	(0..=length)
		.map(|_| rng.sample(Alphanumeric) as char)
		.collect()
}

#[byond_fn]
pub fn replace_chars_prob(
	input: String,
	replacement: String,
	prob: f32,
	secure: Option<bool>,
) -> String {
	if !prob.is_normal() || !prob.is_sign_positive() {
		return input;
	}
	let mut rng = global(secure);
	let distro =
		Bernoulli::new((prob as f64 / 100.0).clamp(0.0, 1.0)).expect("invalid probability, wtf???");
	let mut output = String::with_capacity(input.len() * replacement.len()); // Allocate for worst case scenario.
	input.chars().for_each(|c| {
		if distro.sample(&mut rng) {
			output.push_str(&replacement);
		} else {
			output.push(c);
		}
	});
	output
}
