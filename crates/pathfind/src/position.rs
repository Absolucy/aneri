// SPDX-License-Identifier: MPL-2.0

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position {
	x: u16,
	y: u16,
}

impl Position {
	/// Creates a new [`Position`], from 0-indexed x and y coordinates.
	pub const fn new(x: u16, y: u16) -> Self {
		Self { x, y }
	}

	/// Creates a new [`Position`], from 1-indexed x and y coordinates.
	pub const fn new_1i(x: u16, y: u16) -> Self {
		Self::new(x.saturating_sub(1), y.saturating_sub(1))
	}

	/// Gets the x coordinate of this [`Position`], as a 0-indexed [`u16`].
	pub const fn x(&self) -> u16 {
		self.x
	}

	/// Gets the y coordinate of this [`Position`], as a 0-indexed [`u16`].
	pub const fn y(&self) -> u16 {
		self.y
	}

	/// Gets the x coordinate of this [`Position`], as a 1-indexed [`u16`].
	pub const fn x_1(&self) -> u16 {
		self.x().saturating_add(1)
	}

	/// Gets the y coordinate of this [`Position`], as a 1-indexed [`u16`].
	pub const fn y_1(&self) -> u16 {
		self.y().saturating_add(1)
	}
}
