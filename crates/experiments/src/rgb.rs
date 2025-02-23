// SPDX-License-Identifier: MPL-2.0
use crate::util::{or_if_zero, round_to};
use meowtonin::{ByondResult, ByondValue};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rgb {
	pub r: f32,
	pub g: f32,
	pub b: f32,
}

impl Rgb {
	pub fn read_byond(
		value: &ByondValue,
		var_r: &str,
		var_g: &str,
		var_b: &str,
	) -> ByondResult<Self> {
		let r = value.read_var(var_r)?;
		let g = value.read_var(var_g)?;
		let b = value.read_var(var_b)?;
		Ok(Self { r, g, b })
	}

	pub fn write_byond(
		self,
		value: &mut ByondValue,
		var_r: &str,
		var_g: &str,
		var_b: &str,
	) -> ByondResult<()> {
		value.write_var(var_r, self.r)?;
		value.write_var(var_g, self.g)?;
		value.write_var(var_b, self.b)?;
		Ok(())
	}

	pub fn clamp(self, min: f32, max: f32) -> Self {
		Self {
			r: self.r.clamp(min, max),
			g: self.g.clamp(min, max),
			b: self.b.clamp(min, max),
		}
	}

	pub fn largest_channel(self) -> f32 {
		self.r.max(self.g).max(self.b)
	}

	pub fn round_to(self, denominator: f32) -> Self {
		Self {
			r: round_to(self.r, denominator),
			g: round_to(self.g, denominator),
			b: round_to(self.b, denominator),
		}
	}

	pub fn or_if_zero(self, if_zero: f32) -> Self {
		Self {
			r: or_if_zero(self.r, if_zero),
			g: or_if_zero(self.g, if_zero),
			b: or_if_zero(self.b, if_zero),
		}
	}

	pub fn to_u32(self) -> (u32, u32, u32) {
		(
			(self.r as u32) & 0x00FF_FFFF,
			(self.g as u32) & 0x00FF_FFFF,
			(self.b as u32) & 0x00FF_FFFF,
		)
	}
}

impl From<f32> for Rgb {
	fn from(value: f32) -> Self {
		Self {
			r: value,
			g: value,
			b: value,
		}
	}
}

impl From<(f32, f32, f32)> for Rgb {
	fn from((r, g, b): (f32, f32, f32)) -> Self {
		Self { r, g, b }
	}
}

impl Add<f32> for Rgb {
	type Output = Self;

	fn add(self, rhs: f32) -> Self::Output {
		Self {
			r: self.r + rhs,
			g: self.g + rhs,
			b: self.b + rhs,
		}
	}
}

impl AddAssign<f32> for Rgb {
	fn add_assign(&mut self, rhs: f32) {
		self.r += rhs;
		self.g += rhs;
		self.b += rhs;
	}
}

impl Sub<f32> for Rgb {
	type Output = Self;

	fn sub(self, rhs: f32) -> Self::Output {
		Self {
			r: self.r - rhs,
			g: self.g - rhs,
			b: self.b - rhs,
		}
	}
}

impl SubAssign<f32> for Rgb {
	fn sub_assign(&mut self, rhs: f32) {
		self.r -= rhs;
		self.g -= rhs;
		self.b -= rhs;
	}
}

impl Mul<f32> for Rgb {
	type Output = Self;

	fn mul(self, rhs: f32) -> Self::Output {
		let rhs = rhs as f64;
		Self {
			r: ((self.r as f64) * rhs) as f32,
			g: ((self.g as f64) * rhs) as f32,
			b: ((self.b as f64) * rhs) as f32,
		}
	}
}

impl MulAssign<f32> for Rgb {
	fn mul_assign(&mut self, rhs: f32) {
		let rhs = rhs as f64;
		self.r = ((self.r as f64) * rhs) as f32;
		self.g = ((self.g as f64) * rhs) as f32;
		self.b = ((self.b as f64) * rhs) as f32;
	}
}

impl Div<f32> for Rgb {
	type Output = Self;

	fn div(self, rhs: f32) -> Self::Output {
		let rhs = rhs as f64;
		Self {
			r: ((self.r as f64) / rhs) as f32,
			g: ((self.g as f64) / rhs) as f32,
			b: ((self.b as f64) / rhs) as f32,
		}
	}
}

impl DivAssign<f32> for Rgb {
	fn div_assign(&mut self, rhs: f32) {
		let rhs = rhs as f64;
		self.r = ((self.r as f64) / rhs) as f32;
		self.g = ((self.g as f64) / rhs) as f32;
		self.b = ((self.b as f64) / rhs) as f32;
	}
}
