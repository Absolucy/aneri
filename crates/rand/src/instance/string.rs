// SPDX-License-Identifier: MPL-2.0
use super::INSTANCES;
use aneri_core::ByondSlotKey;
use rand::{
	distributions::{Alphanumeric, Bernoulli, Distribution},
	Rng,
};

#[byond_fn]
pub fn instanced_random_string_alphanumeric(src: ByondSlotKey, length: usize) -> Option<String> {
	INSTANCES.lock().get_mut(src).map(|rng| {
		(0..=length)
			.map(|_| rng.sample(Alphanumeric) as char)
			.collect()
	})
}

#[byond_fn]
pub fn instnaced_replace_chars_prob(
	src: ByondSlotKey,
	input: String,
	replacement: String,
	prob: f32,
) -> Option<String> {
	if !prob.is_normal() || !prob.is_sign_positive() {
		return Some(input);
	}
	INSTANCES.lock().get_mut(src).map(|rng| {
		let distro = Bernoulli::new((prob as f64 / 100.0).clamp(0.0, 1.0))
			.expect("invalid probability, wtf???");
		let mut output = String::with_capacity(input.len() * replacement.len()); // Allocate for worst case scenario.
		input.chars().for_each(|c| {
			if distro.sample(rng) {
				output.push_str(&replacement);
			} else {
				output.push(c);
			}
		});
		output
	})
}
