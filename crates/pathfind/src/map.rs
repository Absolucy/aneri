// SPDX-License-Identifier: MPL-2.0
use crate::{position::Position, tile::Tile};
use grid::Grid;

pub struct GameMap {
	grid: Grid<Option<Tile>>,
}

impl GameMap {
	pub fn new(width: usize, height: usize) -> Self {
		Self {
			grid: Grid::new(width, height),
		}
	}

	pub fn has_tile(&self, pos: Position) -> bool {
		matches!(self.grid.get(pos.x(), pos.y()), Some(Some(_)))
	}

	pub fn get_tile(&self, pos: Position) -> Option<&Tile> {
		self.grid
			.get(pos.x(), pos.y())
			.and_then(|tile| tile.as_ref())
	}

	pub fn get_tiles<T>(&self, pos_list: T) -> Vec<Option<&Tile>>
	where
		T: AsRef<[Position]>,
	{
		pos_list
			.as_ref()
			.iter()
			.map(|&pos| self.get_tile(pos))
			.collect()
	}

	pub fn set_tile<T>(&mut self, pos: Position, tile: T) -> Option<Tile>
	where
		T: Into<Option<Tile>>,
	{
		let tile_ref = self.grid.get_mut(pos.x(), pos.y())?;
		let mut tile = tile.into();
		std::mem::swap(tile_ref, &mut tile);
		tile
	}

	pub fn width(&self) -> usize {
		self.grid.rows()
	}

	pub fn height(&self) -> usize {
		self.grid.cols()
	}
}
