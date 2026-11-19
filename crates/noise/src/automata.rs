// SPDX-License-Identifier: MPL-2.0
#![allow(clippy::too_many_arguments)]
use crate::make_rng;
use meowtonin::byond_fn;
use rand::{Rng, RngExt};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Cell {
	Wall,
	Floor,
	// fixed cells are skipped by the automaton, so rooms and prefabs survive it
	FixedWall,
	FixedFloor,
}

impl Cell {
	pub(crate) fn is_floor(self) -> bool {
		matches!(self, Self::Floor | Self::FixedFloor)
	}

	pub(crate) fn is_fixed(self) -> bool {
		matches!(self, Self::FixedWall | Self::FixedFloor)
	}
}

/// Row-major grid with a one-cell fixed border around it, so counting the
/// neighbors of an edge cell needs no bounds checks.
pub(crate) struct Grid {
	pub(crate) width: usize,
	pub(crate) height: usize,
	cells: Vec<Cell>,
	scratch: Vec<Cell>,
}

impl Grid {
	/// Every inner cell starts as a wall.
	/// The border counts as floor if `edges_alive`.
	pub(crate) fn new(width: usize, height: usize, edges_alive: bool) -> Self {
		let border = if edges_alive {
			Cell::FixedFloor
		} else {
			Cell::FixedWall
		};
		let mut cells = vec![border; (width + 2) * (height + 2)];
		for y in 0..height {
			let start = Self::index_in(width, 0, y);
			cells[start..start + width].fill(Cell::Wall);
		}
		let scratch = cells.clone();
		Self {
			width,
			height,
			cells,
			scratch,
		}
	}

	fn index_in(width: usize, x: usize, y: usize) -> usize {
		(y + 1) * (width + 2) + x + 1
	}

	pub(crate) fn get(&self, x: usize, y: usize) -> Cell {
		self.cells[Self::index_in(self.width, x, y)]
	}

	pub(crate) fn set(&mut self, x: usize, y: usize, cell: Cell) {
		self.cells[Self::index_in(self.width, x, y)] = cell;
	}

	/// Turns every non-fixed cell into a floor with chance `floor_chance` (0 to 1).
	pub(crate) fn scatter(&mut self, floor_chance: f32, rng: &mut impl Rng) {
		let floor_chance = f64::from(floor_chance.clamp(0.0, 1.0));
		for y in 0..self.height {
			for x in 0..self.width {
				if !self.get(x, y).is_fixed() {
					let cell = if rng.random_bool(floor_chance) {
						Cell::Floor
					} else {
						Cell::Wall
					};
					self.set(x, y, cell);
				}
			}
		}
	}

	/// One automaton step: a floor stays a floor with at least `survival_limit`
	/// floor neighbors (of 8), a wall becomes a floor with at least `birth_limit`.
	pub(crate) fn step(&mut self, birth_limit: u32, survival_limit: u32) {
		let stride = self.width + 2;
		self.scratch.copy_from_slice(&self.cells);
		for y in 1..=self.height {
			for x in 1..=self.width {
				let index = y * stride + x;
				let limit = match self.cells[index] {
					Cell::FixedWall | Cell::FixedFloor => continue,
					Cell::Floor => survival_limit,
					Cell::Wall => birth_limit,
				};
				let (above, below) = (index - stride, index + stride);
				let neighbors = [
					above - 1,
					above,
					above + 1,
					index - 1,
					index + 1,
					below - 1,
					below,
					below + 1,
				]
				.into_iter()
				.filter(|&neighbor| self.cells[neighbor].is_floor())
				.count() as u32;
				self.scratch[index] = if neighbors >= limit {
					Cell::Floor
				} else {
					Cell::Wall
				};
			}
		}
		std::mem::swap(&mut self.cells, &mut self.scratch);
	}

	/// 1 for floor, 0 for wall, row by row without the border.
	pub(crate) fn to_flags(&self) -> Vec<u8> {
		(0..self.height)
			.flat_map(|y| (0..self.width).map(move |x| (x, y)))
			.map(|(x, y)| u8::from(self.get(x, y).is_floor()))
			.collect()
	}
}

/// Generates a cellular automata noise grid, which can be used in procedural generation methods.
///
/// Returns a list of 1 (alive cell) and 0 (dead cell), row by row: cell `(x, y)`,
/// counting from 1, is at index `(y - 1) * width + x`.
///
/// Arguments:
/// * width: The width of the grid.
/// * height: The height of the grid.
/// * fill_percent: The chance (0 to 100) that a cell starts alive.
/// * iterations: The amount of iterations the cellular automata simulates before returning the results.
/// * birth_limit: A dead cell with at least this many alive neighbors (of 8) is born.
/// * survival_limit: An alive cell with fewer alive neighbors than this dies.
/// * edges_alive: Whether cells outside the grid count as alive (default false).
/// * seed: The same seed always gives the same grid.
///   Random if left out.
///
/// rust-g's `cnoise_generate` used strict comparisons, so its `birth_limit`
/// needs +1 here, and its `death_limit` is this `survival_limit`.
#[byond_fn(name = "aneri_raw_noise_cellular_automata")]
pub fn noise_cellular_automata(
	width: u32,
	height: u32,
	fill_percent: f32,
	iterations: u32,
	birth_limit: u32,
	survival_limit: u32,
	edges_alive: Option<bool>,
	seed: Option<u32>,
) -> Vec<u8> {
	let mut rng = make_rng(seed);
	let mut grid = Grid::new(
		width as usize,
		height as usize,
		edges_alive.unwrap_or(false),
	);
	grid.scatter(fill_percent / 100.0, &mut rng);
	for _ in 0..iterations {
		grid.step(birth_limit, survival_limit);
	}
	grid.to_flags()
}
