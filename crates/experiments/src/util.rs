// SPDX-License-Identifier: MPL-2.0
#[inline]
pub fn round_to(value: f32, denominator: f32) -> f32 {
	let value = value as f64;
	let denominator = denominator as f64;
	((value * denominator).round() / denominator) as f32
}

#[inline]
pub fn or_if_zero(value: f32, if_zero: f32) -> f32 {
	if value == 0.0 {
		if_zero
	} else {
		value
	}
}
