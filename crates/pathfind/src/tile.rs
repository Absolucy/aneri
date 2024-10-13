// SPDX-License-Identifier: MPL-2.0
use crate::direction::Direction;
use ordered_float::OrderedFloat;

pub struct Tile {
	pub weight: OrderedFloat<f32>,
	pub passable_directions: Direction,
	pub condition: Option<Box<dyn Fn() -> bool>>,
}
