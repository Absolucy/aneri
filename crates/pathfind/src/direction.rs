// SPDX-License-Identifier: MPL-2.0
use bitflags::bitflags;

bitflags! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
	pub struct Direction: u8 {
		const NORTH = 1;
		const SOUTH = 2;
		const EAST = 4;
		const WEST = 8;
		const NORTHEAST = Self::NORTH.bits() | Self::EAST.bits();
		const NORTHWEST = Self::NORTH.bits() | Self::WEST.bits();
		const SOUTHEAST = Self::SOUTH.bits() | Self::EAST.bits();
		const SOUTHWEST = Self::SOUTH.bits() | Self::WEST.bits();
	}
}
