// SPDX-License-Identifier: MPL-2.0
use meowtonin::byond_fn;

#[byond_fn]
pub fn levenshtein(a: String, b: String) -> usize {
	strsim::levenshtein(&a, &b)
}

#[byond_fn]
pub fn damerau_levenshtein(a: String, b: String) -> usize {
	strsim::damerau_levenshtein(&a, &b)
}

#[byond_fn]
pub fn normalized_levenshtein(a: String, b: String) -> f32 {
	strsim::normalized_levenshtein(&a, &b) as f32
}

#[byond_fn]
pub fn normalized_damerau_levenshtein(a: String, b: String) -> f32 {
	strsim::normalized_damerau_levenshtein(&a, &b) as f32
}

#[byond_fn]
pub fn hamming(a: String, b: String) -> Option<usize> {
	strsim::hamming(&a, &b).ok()
}

#[byond_fn]
pub fn deunicode(string: String, placeholder: Option<String>) -> String {
	match placeholder {
		Some(placeholder) => deunicode::deunicode_with_tofu(&string, &placeholder),
		None => deunicode::deunicode(&string),
	}
}
