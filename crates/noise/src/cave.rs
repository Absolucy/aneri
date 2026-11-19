// SPDX-License-Identifier: MPL-2.0
use crate::{
	automata::{Cell, Grid},
	make_rng,
};
use meowtonin::{ByondError, ByondResult, ByondValue, byond_fn};
use rand::{Rng, RngExt};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
struct CaveSettings {
	min_bsp_size: usize,
	max_ratio: f32,
	padding: usize,
	room_fill_percent: f32,
	corridor_width: usize,
	loop_percent: f32,
	noise_percent: f32,
	ca_steps: u32,
	birth_limit: u32,
	survival_limit: u32,
	edges_alive: bool,
}

impl Default for CaveSettings {
	fn default() -> Self {
		Self {
			min_bsp_size: 20,
			max_ratio: 2.5,
			padding: 2,
			room_fill_percent: 80.0,
			corridor_width: 1,
			loop_percent: 15.0,
			noise_percent: 48.0,
			ca_steps: 6,
			birth_limit: 5,
			survival_limit: 4,
			edges_alive: false,
		}
	}
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Prefab {
	x: usize,
	y: usize,
	width: usize,
	height: usize,
	#[serde(default)]
	enclosed: bool,
}

#[derive(Clone, Copy)]
struct Rect {
	x: usize,
	y: usize,
	width: usize,
	height: usize,
}

struct Edge {
	a: usize,
	b: usize,
	length: f32,
}

/// Generates a procedural dungeon map using BSP tree partitioning, prefab placement,
/// MST corridor generation, and cellular automata smoothing.
///
/// Returns a list of 1 (floor) and 0 (wall), row by row: cell `(x, y)`, counting
/// from 1, is at index `(y - 1) * width + x`.
/// Every floor is connected to a room, a corridor, or an open prefab.
/// Rooms, corridors and prefabs are never changed by the smoothing.
///
/// Arguments:
/// * width: Grid width.
/// * height: Grid height.
/// * settings: Assoc list, every key optional (default in brackets).
///   An unknown key runtimes.
///   * min_bsp_size: Minimum BSP leaf dimension [20]
///   * max_ratio: Maximum aspect ratio for BSP splits [2.5]
///   * padding: Room edge padding within a BSP leaf [2]
///   * room_fill_percent: How much of each BSP leaf a room fills, 0 to 100 [80]
///   * corridor_width: Width of corridors between rooms [1]
///   * loop_percent: Chance to add extra MST edges for loops, 0 to 100 [15]
///   * noise_percent: Initial random floor density, 0 to 100 [48]
///   * ca_steps: Cellular automata smoothing iterations [6]
///   * birth_limit: Neighbors to create floor (>=) [5]
///   * survival_limit: Neighbors to survive as floor (>=) [4]
///   * edges_alive: Whether out-of-bounds cells count as floor for neighbor counts [FALSE]
/// * prefabs: List of `list("x" = , "y" = , "width" = , "height" = , "enclosed" = )`.
///   x and y are the bottom-left turf, counting from 1.
///   An enclosed prefab is kept solid wall, an open one is kept as floor.
/// * seed: The same seed always gives the same map.
///   Random if left out.
#[byond_fn(name = "aneri_raw_noise_cave_system")]
pub fn noise_cave_system(
	width: u32,
	height: u32,
	settings: Option<ByondValue>,
	prefabs: Option<ByondValue>,
	seed: Option<u32>,
) -> ByondResult<Vec<u8>> {
	let settings = settings.map_or_else(
		|| Ok(CaveSettings::default()),
		|value| meowtonin_serde::deserialize(value).map_err(ByondError::boxed),
	)?;
	let prefabs = prefabs.map_or_else(
		|| Ok(Vec::new()),
		|value| meowtonin_serde::deserialize(value).map_err(ByondError::boxed),
	)?;
	let mut rng = make_rng(seed);
	Ok(generate(
		width as usize,
		height as usize,
		&settings,
		&prefabs,
		&mut rng,
	)
	.to_flags())
}

fn generate(
	width: usize,
	height: usize,
	settings: &CaveSettings,
	prefabs: &[Prefab],
	rng: &mut impl Rng,
) -> Grid {
	let mut grid = Grid::new(width, height, settings.edges_alive);
	if width == 0 || height == 0 {
		return grid;
	}

	for prefab in prefabs {
		let cell = if prefab.enclosed {
			Cell::FixedWall
		} else {
			Cell::FixedFloor
		};
		let x = prefab.x.saturating_sub(1);
		let y = prefab.y.saturating_sub(1);
		for y in y..(y + prefab.height).min(height) {
			for x in x..(x + prefab.width).min(width) {
				grid.set(x, y, cell);
			}
		}
	}

	let area = Rect {
		x: 0,
		y: 0,
		width,
		height,
	};
	let leaves = split_into_leaves(area, settings.min_bsp_size.max(1), settings.max_ratio, rng);
	let room_scale = (settings.room_fill_percent / 100.0).clamp(0.0, 1.0);
	let rooms = leaves
		.iter()
		.map(|&leaf| make_room(leaf, settings.padding, room_scale, rng))
		.collect::<Vec<_>>();

	for room in rooms.iter().flatten() {
		for y in room.y..room.y + room.height {
			for x in room.x..room.x + room.width {
				if !grid.get(x, y).is_fixed() {
					grid.set(x, y, Cell::FixedFloor);
				}
			}
		}
	}

	let corridor_width = settings.corridor_width.max(1);
	for Edge { a, b, .. } in join_rooms(&leaves, &rooms, settings.loop_percent, rng) {
		if let (Some(from), Some(to)) = (rooms[a], rooms[b]) {
			carve_corridor(&mut grid, center(from), center(to), corridor_width, rng);
		}
	}

	grid.scatter(settings.noise_percent / 100.0, rng);
	for _ in 0..settings.ca_steps {
		grid.step(settings.birth_limit, settings.survival_limit);
	}
	remove_islands(&mut grid);
	grid
}

fn center(room: Rect) -> (usize, usize) {
	(room.x + room.width / 2, room.y + room.height / 2)
}

/// Binary space partition: keeps cutting areas in two until none can be cut again.
fn split_into_leaves(area: Rect, min_size: usize, max_ratio: f32, rng: &mut impl Rng) -> Vec<Rect> {
	let mut leaves = Vec::new();
	let mut pending = vec![area];
	while let Some(rect) = pending.pop() {
		let can_split_x = rect.width > min_size * 2;
		let can_split_y = rect.height > min_size * 2;
		let split_x = match (can_split_x, can_split_y) {
			(false, false) => {
				leaves.push(rect);
				continue;
			}
			(true, false) => true,
			(false, true) => false,
			(true, true) => {
				let ratio = rect.width as f32 / rect.height as f32;
				if ratio >= max_ratio {
					true
				} else if ratio.recip() >= max_ratio {
					false
				} else {
					rng.random_bool(0.5)
				}
			}
		};
		if split_x {
			let cut = rng.random_range(min_size..rect.width - min_size);
			pending.push(Rect { width: cut, ..rect });
			pending.push(Rect {
				x: rect.x + cut,
				width: rect.width - cut,
				..rect
			});
		} else {
			let cut = rng.random_range(min_size..rect.height - min_size);
			pending.push(Rect {
				height: cut,
				..rect
			});
			pending.push(Rect {
				y: rect.y + cut,
				height: rect.height - cut,
				..rect
			});
		}
	}
	leaves
}

/// A room somewhere inside `leaf`, or none if the leaf is too small after padding.
fn make_room(leaf: Rect, padding: usize, scale: f32, rng: &mut impl Rng) -> Option<Rect> {
	let max_width = leaf.width.saturating_sub(padding * 2);
	let max_height = leaf.height.saturating_sub(padding * 2);
	if max_width < 3 || max_height < 3 {
		return None;
	}
	let width = room_size(max_width, scale, rng);
	let height = room_size(max_height, scale, rng);
	Some(Rect {
		x: leaf.x + room_offset(leaf.width, width, padding, rng),
		y: leaf.y + room_offset(leaf.height, height, padding, rng),
		width,
		height,
	})
}

fn room_offset(leaf_size: usize, room_size: usize, padding: usize, rng: &mut impl Rng) -> usize {
	let last = leaf_size.saturating_sub(room_size + padding).max(padding);
	rng.random_range(padding..=last)
}

fn room_size(max: usize, scale: f32, rng: &mut impl Rng) -> usize {
	let low = ((max as f32 * 0.3) as usize).max(1);
	let high = (max as f32 * scale) as usize;
	let size = if high > low {
		rng.random_range(low..high)
	} else {
		low
	};
	size.clamp(3, max)
}

fn touching(a: Rect, b: Rect) -> bool {
	let (a_right, a_bottom) = (a.x + a.width, a.y + a.height);
	let (b_right, b_bottom) = (b.x + b.width, b.y + b.height);
	let overlap_y = a.y < b_bottom && b.y < a_bottom;
	let overlap_x = a.x < b_right && b.x < a_right;
	((a.x == b_right || b.x == a_right) && overlap_y)
		|| ((a.y == b_bottom || b.y == a_bottom) && overlap_x)
}

/// Shortest set of corridors that connects every room next to another (Kruskal),
/// plus each leftover corridor with chance `loop_percent`, to make loops.
fn join_rooms(
	leaves: &[Rect],
	rooms: &[Option<Rect>],
	loop_percent: f32,
	rng: &mut impl Rng,
) -> Vec<Edge> {
	let mut candidates = Vec::new();
	for (a, (leaf_a, room_a)) in leaves.iter().zip(rooms).enumerate() {
		for (b, (leaf_b, room_b)) in leaves.iter().zip(rooms).enumerate().skip(a + 1) {
			let (Some(room_a), Some(room_b)) = (room_a, room_b) else {
				continue;
			};
			if !touching(*leaf_a, *leaf_b) {
				continue;
			}
			let ((ax, ay), (bx, by)) = (center(*room_a), center(*room_b));
			let (dx, dy) = (ax.abs_diff(bx) as f32, ay.abs_diff(by) as f32);
			candidates.push(Edge {
				a,
				b,
				length: dx.hypot(dy),
			});
		}
	}
	candidates.sort_by(|left, right| left.length.total_cmp(&right.length));

	let loop_chance = f64::from((loop_percent / 100.0).clamp(0.0, 1.0));
	let mut parent = (0..leaves.len()).collect::<Vec<_>>();
	let mut chosen = Vec::new();
	for edge in candidates {
		let (root_a, root_b) = (
			find_root(&mut parent, edge.a),
			find_root(&mut parent, edge.b),
		);
		if root_a != root_b {
			parent[root_a] = root_b;
			chosen.push(edge);
		} else if rng.random_bool(loop_chance) {
			chosen.push(edge);
		}
	}
	chosen
}

fn find_root(parent: &mut [usize], mut node: usize) -> usize {
	while parent[node] != node {
		parent[node] = parent[parent[node]];
		node = parent[node];
	}
	node
}

/// L-shaped corridor, going along x first or y first at random.
fn carve_corridor(
	grid: &mut Grid,
	from: (usize, usize),
	to: (usize, usize),
	corridor_width: usize,
	rng: &mut impl Rng,
) {
	let (mut x, mut y) = from;
	let x_first = rng.random_bool(0.5);
	for along_x in if x_first {
		[true, false]
	} else {
		[false, true]
	} {
		if along_x {
			while x != to.0 {
				x = one_step_toward(x, to.0);
				paint(grid, x, y, corridor_width);
			}
		} else {
			while y != to.1 {
				y = one_step_toward(y, to.1);
				paint(grid, x, y, corridor_width);
			}
		}
	}
}

fn one_step_toward(from: usize, to: usize) -> usize {
	if from < to { from + 1 } else { from - 1 }
}

fn paint(grid: &mut Grid, x: usize, y: usize, size: usize) {
	for y in y..(y + size).min(grid.height) {
		for x in x..(x + size).min(grid.width) {
			if grid.get(x, y) != Cell::FixedWall {
				grid.set(x, y, Cell::FixedFloor);
			}
		}
	}
}

/// Walls off every floor that has no path to a fixed floor (room, corridor or open prefab).
/// Floor reaching a fixed floor is kept whole, so natural caves around prefabs survive.
fn remove_islands(grid: &mut Grid) {
	let (width, height) = (grid.width, grid.height);
	let mut reached = vec![false; width * height];
	let mut stack = Vec::new();
	for y in 0..height {
		for x in 0..width {
			if grid.get(x, y) == Cell::FixedFloor {
				reached[y * width + x] = true;
				stack.push((x, y));
			}
		}
	}
	// nothing fixed to anchor to, so there is no telling which floor is an island
	if stack.is_empty() {
		return;
	}

	while let Some((x, y)) = stack.pop() {
		let neighbors = [
			(x.wrapping_sub(1), y),
			(x + 1, y),
			(x, y.wrapping_sub(1)),
			(x, y + 1),
		];
		for (nx, ny) in neighbors {
			if nx < width && ny < height && !reached[ny * width + nx] && grid.get(nx, ny).is_floor()
			{
				reached[ny * width + nx] = true;
				stack.push((nx, ny));
			}
		}
	}

	for y in 0..height {
		for x in 0..width {
			if !reached[y * width + x] && grid.get(x, y) == Cell::Floor {
				grid.set(x, y, Cell::Wall);
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn settings() -> CaveSettings {
		CaveSettings {
			min_bsp_size: 10,
			..CaveSettings::default()
		}
	}

	#[test]
	fn same_seed_same_map() {
		let make = || generate(70, 60, &settings(), &[], &mut make_rng(Some(11))).to_flags();
		assert_eq!(make(), make());
	}

	#[test]
	fn floor_is_one_connected_cave() {
		let (width, height) = (70, 60);
		for seed in 0..8 {
			let flags =
				generate(width, height, &settings(), &[], &mut make_rng(Some(seed))).to_flags();
			let start = flags
				.iter()
				.position(|&flag| flag == 1)
				.expect("some floor");
			let mut seen = vec![false; flags.len()];
			let mut stack = vec![start];
			seen[start] = true;
			while let Some(index) = stack.pop() {
				let (x, y) = (index % width, index / width);
				let around = [
					(x > 0).then(|| index - 1),
					(x + 1 < width).then(|| index + 1),
					(y > 0).then(|| index - width),
					(y + 1 < height).then(|| index + width),
				];
				for next in around.into_iter().flatten() {
					if flags[next] == 1 && !seen[next] {
						seen[next] = true;
						stack.push(next);
					}
				}
			}
			let disconnected = (0..flags.len())
				.filter(|&i| flags[i] == 1 && !seen[i])
				.count();
			assert_eq!(disconnected, 0, "seed {seed}");
		}
	}

	#[test]
	fn enclosed_prefab_stays_solid() {
		let prefab = Prefab {
			x: 20,
			y: 20,
			width: 12,
			height: 9,
			enclosed: true,
		};
		for seed in 0..8 {
			let flags = generate(
				70,
				60,
				&settings(),
				std::slice::from_ref(&prefab),
				&mut make_rng(Some(seed)),
			)
			.to_flags();
			for y in 19..28 {
				for x in 19..31 {
					assert_eq!(flags[y * 70 + x], 0, "seed {seed} at {x},{y}");
				}
			}
		}
	}
}
