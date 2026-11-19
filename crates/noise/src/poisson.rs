// SPDX-License-Identifier: MPL-2.0
use crate::make_rng;
use meowtonin::byond_fn;
use rand::{Rng, RngExt};
use std::f32::consts::{SQRT_2, TAU};

// how many tries a point gets to place a neighbor before it is retired
const ATTEMPTS: usize = 30;
const EMPTY: u32 = u32::MAX;

/// Generates a 2D poisson disk distribution ('blue noise'), which is relatively uniform.
///
/// Returns a list of `list(x, y)` points, counting from 1.
///
/// Arguments:
/// * width: Width of the noisemap (see world.maxx).
/// * height: Height of the noisemap (see world.maxy).
/// * radius: Minimum distance between points.
///   Under 1 is treated as 1.
/// * seed: The same seed always gives the same points.
///   Random if left out.
#[byond_fn(name = "aneri_raw_noise_poisson")]
pub fn noise_poisson(width: u32, height: u32, radius: f32, seed: Option<u32>) -> Vec<[u32; 2]> {
	let mut rng = make_rng(seed);
	let (width, height) = (width as usize, height as usize);
	let points = poisson_points(width, height, radius.max(1.0), &mut rng);
	points
		.into_iter()
		.map(|(x, y)| [x as u32 + 1, y as u32 + 1])
		.collect()
}

/// Bridson's algorithm on whole tiles (0-based).
/// `radius` must be at least 1.
fn poisson_points(
	width: usize,
	height: usize,
	radius: f32,
	rng: &mut impl Rng,
) -> Vec<(usize, usize)> {
	if width == 0 || height == 0 {
		return Vec::new();
	}

	// one cell can hold at most one point, because its diagonal is exactly `radius`
	let cell_size = radius / SQRT_2;
	let columns = (width as f32 / cell_size).ceil() as usize;
	let rows = (height as f32 / cell_size).ceil() as usize;
	let cell_of = |(x, y): (usize, usize)| {
		(
			(x as f32 / cell_size) as usize,
			(y as f32 / cell_size) as usize,
		)
	};
	let mut cells = vec![EMPTY; columns * rows];
	let mut points = Vec::new();
	let mut active = Vec::new();

	let first = (rng.random_range(0..width), rng.random_range(0..height));
	let (column, row) = cell_of(first);
	cells[row * columns + column] = 0;
	points.push(first);
	active.push(0_usize);

	while !active.is_empty() {
		let slot = rng.random_range(0..active.len());
		let (parent_x, parent_y) = points[active[slot]];
		let found = (0..ATTEMPTS).find_map(|_| {
			let angle = rng.random_range(0.0..TAU);
			let distance = rng.random_range(radius..radius * 2.0);
			let x = (parent_x as f32 + angle.cos() * distance).round();
			let y = (parent_y as f32 + angle.sin() * distance).round();
			if x < 0.0 || y < 0.0 || x >= width as f32 || y >= height as f32 {
				return None;
			}
			let candidate = (x as usize, y as usize);
			let (column, row) = cell_of(candidate);
			// radius / cell_size is sqrt(2), so anything closer than 2 cells away is far enough
			let near =
				|center: usize, limit: usize| center.saturating_sub(2)..(center + 3).min(limit);
			let too_close = near(row, rows).any(|row| {
				near(column, columns).any(|column| {
					let other = cells[row * columns + column];
					other != EMPTY && {
						let (other_x, other_y) = points[other as usize];
						let (dx, dy) =
							(other_x.abs_diff(candidate.0), other_y.abs_diff(candidate.1));
						((dx * dx + dy * dy) as f32) < radius * radius
					}
				})
			});
			(!too_close).then_some(candidate)
		});
		match found {
			Some(candidate) => {
				let (column, row) = cell_of(candidate);
				cells[row * columns + column] = points.len() as u32;
				active.push(points.len());
				points.push(candidate);
			}
			None => {
				active.swap_remove(slot);
			}
		}
	}
	points
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn points_are_in_bounds_and_spaced() {
		let radius = 6.5;
		let points = poisson_points(80, 50, radius, &mut make_rng(Some(3)));
		assert!(points.len() > 20);
		for (index, &(x, y)) in points.iter().enumerate() {
			assert!(x < 80 && y < 50);
			for &(other_x, other_y) in &points[index + 1..] {
				let (dx, dy) = (x.abs_diff(other_x), y.abs_diff(other_y));
				assert!(((dx * dx + dy * dy) as f32) >= radius * radius);
			}
		}
	}
}
