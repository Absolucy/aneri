// SPDX-License-Identifier: MPL-2.0
use meowtonin::byond_fn;

/// How many single-character inserts, deletes or substitutions
/// turn `a` into `b`.
#[byond_fn]
pub fn levenshtein(a: String, b: String) -> usize {
	strsim::levenshtein(&a, &b)
}

/// Like `aneri_levenshtein`,
/// but swapping two characters next to each other also counts as one edit.
#[byond_fn]
pub fn damerau_levenshtein(a: String, b: String) -> usize {
	strsim::damerau_levenshtein(&a, &b)
}

/// `aneri_levenshtein` scaled from 0 to 1,
/// where 1 means the strings are the same.
#[byond_fn]
pub fn normalized_levenshtein(a: String, b: String) -> f32 {
	strsim::normalized_levenshtein(&a, &b) as f32
}

/// `aneri_damerau_levenshtein` scaled from 0 to 1,
/// where 1 means the strings are the same.
#[byond_fn]
pub fn normalized_damerau_levenshtein(a: String, b: String) -> f32 {
	strsim::normalized_damerau_levenshtein(&a, &b) as f32
}

/// How many positions have a different character in `a` and `b`.
/// Returns null if they aren't the same length.
#[byond_fn]
pub fn hamming(a: String, b: String) -> Option<usize> {
	strsim::hamming(&a, &b).ok()
}

/// Turns Unicode text into plain ASCII, like `Æneid` into `AEneid`.
#[byond_fn]
pub fn deunicode(
	string: String,
	/// Text for characters it can't convert, default `[?]`.
	placeholder: Option<String>,
) -> String {
	match placeholder {
		Some(placeholder) => deunicode::deunicode_with_tofu(&string, &placeholder),
		None => deunicode::deunicode(&string),
	}
}
