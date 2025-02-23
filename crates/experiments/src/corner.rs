// SPDX-License-Identifier: MPL-2.0
use crate::rgb::Rgb;
use meowtonin::{ByondResult, ByondValue};

pub struct CornerRgb {
	pub red: Rgb,
	pub green: Rgb,
	pub blue: Rgb,
	pub alpha: Rgb,
}

impl CornerRgb {
	pub fn read_byond(
		corner_red: &ByondValue,
		corner_green: &ByondValue,
		corner_blue: &ByondValue,
		corner_alpha: &ByondValue,
		var_r: &str,
		var_g: &str,
		var_b: &str,
	) -> ByondResult<Self> {
		Ok(Self {
			red: Rgb::read_byond(corner_red, var_r, var_g, var_b)?,
			green: Rgb::read_byond(corner_green, var_r, var_g, var_b)?,
			blue: Rgb::read_byond(corner_blue, var_r, var_g, var_b)?,
			alpha: Rgb::read_byond(corner_alpha, var_r, var_g, var_b)?,
		})
	}
}
