// SPDX-License-Identifier: MPL-2.0
use rand::{
	distributions::{Alphanumeric, Bernoulli, Distribution},
	Rng,
};

pub(crate) fn random_string_alphanumeric<Gen>(rng: &mut Gen, length: usize) -> String
where
	Gen: Rng,
{
	(0..=length)
		.map(|_| rng.sample(Alphanumeric) as char)
		.collect()
}

pub(crate) fn replace_chars_prob<Gen>(
	rng: &mut Gen,
	input: String,
	replacement: String,
	prob: f32,
	skip_whitespace: Option<bool>,
) -> String
where
	Gen: Rng,
{
	if !prob.is_normal() || !prob.is_sign_positive() {
		return input;
	}
	let skip_whitespace = skip_whitespace.unwrap_or(false);
	let distro =
		Bernoulli::new((prob as f64 / 100.0).clamp(0.0, 1.0)).expect("invalid probability, wtf???");
	let mut output = String::with_capacity(input.len() * replacement.len()); // Allocate for worst case scenario.
	input.chars().for_each(|c| {
		if (!skip_whitespace || !c.is_whitespace()) && distro.sample(rng) {
			output.push_str(&replacement);
		} else {
			output.push(c);
		}
	});
	output
}
