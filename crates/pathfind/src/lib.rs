use bitflags::bitflags;
use ordered_float::OrderedFloat;
use pathfinding::prelude::*;
use std::collections::HashMap;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
	x: i32,
	y: i32,
}

pub struct Tile {
	weight: OrderedFloat<f32>,
	passable_directions: Direction,
	condition: Option<Box<dyn Fn() -> bool>>,
}

pub struct GameMap {
	tiles: HashMap<Position, Tile>,
	width: i32,
	height: i32,
}

pub struct GameState {
	// Add any game-specific state here
	// For example: player_level: u32, inventory: Vec<Item>, etc.
}

impl GameMap {
	pub fn new(width: i32, height: i32) -> Self {
		GameMap {
			tiles: HashMap::new(),
			width,
			height,
		}
	}

	pub fn set_tile(&mut self, pos: Position, tile: Tile) {
		self.tiles.insert(pos, tile);
	}

	pub fn get_tile(&self, pos: &Position) -> Option<&Tile> {
		self.tiles.get(pos)
	}

	pub fn is_within_bounds(&self, pos: &Position) -> bool {
		pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
	}
}

pub struct Pathfinder<'a> {
	map: &'a GameMap,
}

impl<'a> Pathfinder<'a> {
	pub fn new(map: &'a GameMap) -> Self {
		Pathfinder { map }
	}

	pub fn find_path(
		&self,
		start: Position,
		goal: Position,
	) -> Option<(Vec<Position>, OrderedFloat<f32>)> {
		astar(
			&start,
			|p| self.successors(p),
			|p| self.estimate_cost(p, &goal),
			|p| *p == goal,
		)
	}

	fn successors(&self, pos: &Position) -> Vec<(Position, OrderedFloat<f32>)> {
		let directions = [
			(Direction::NORTH, (0, -1)),
			(Direction::SOUTH, (0, 1)),
			(Direction::EAST, (1, 0)),
			(Direction::WEST, (-1, 0)),
			(Direction::NORTHEAST, (1, -1)),
			(Direction::NORTHWEST, (-1, -1)),
			(Direction::SOUTHEAST, (1, 1)),
			(Direction::SOUTHWEST, (-1, 1)),
		];

		directions
			.iter()
			.filter_map(|(dir, (dx, dy))| {
				let new_pos = Position {
					x: pos.x + dx,
					y: pos.y + dy,
				};

				if !self.map.is_within_bounds(&new_pos) {
					return None;
				}
				let tile = self.map.get_tile(&new_pos)?;

				if !tile.passable_directions.contains(*dir) {
					return None;
				}
				if let Some(condition) = &tile.condition {
					if condition() {
						return Some((new_pos, tile.weight));
					}
				} else {
					return Some((new_pos, tile.weight));
				}

				None
			})
			.collect()
	}

	fn estimate_cost(&self, pos: &Position, goal: &Position) -> OrderedFloat<f32> {
		let dx = (pos.x - goal.x).abs() as f32;
		let dy = (pos.y - goal.y).abs() as f32;
		OrderedFloat((dx * dx + dy * dy).sqrt())
	}
}

/*
// Example usage
pub fn main() {
	let mut map = GameMap::new(10, 10);

	// Set up some tiles
	map.set_tile(
		Position { x: 0, y: 0 },
		Tile {
			weight: 1.0,
			passable_directions: Direction::all(),
			condition: None,
		},
	);

	map.set_tile(
		Position { x: 1, y: 1 },
		Tile {
			weight: 2.0,
			passable_directions: Direction::NORTH | Direction::SOUTH,
			condition: Some(Box::new(|game_state| {
				// Add your condition here
				true
			})),
		},
	);

	let game_state = GameState {
		// Initialize your game state here
	};

	let pathfinder = Pathfinder::new(&map, &game_state);

	let start = Position { x: 0, y: 0 };
	let goal = Position { x: 5, y: 5 };

	if let Some((path, cost)) = pathfinder.find_path(start, goal) {
		println!("Path found: {:?}", path);
		println!("Total cost: {}", cost);
	} else {
		println!("No path found");
	}
}
*/
