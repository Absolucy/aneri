// SPDX-License-Identifier: MPL-2.0
use fastnoise_lite::{
	CellularDistanceFunction, CellularReturnType, FastNoiseLite, FractalType, NoiseType,
};
use meowtonin::{ByondError, ByondResult, byond_fn};
use std::{f32::consts::SQRT_2, num::NonZeroUsize, thread};

struct Sampler {
	noise: FastNoiseLite,
	// raw * scale + bias lands in 0..1: simplex gives -1..1, perlin only about -0.71..0.71, and
	// worley is 0..1 minus one
	scale: f32,
	bias: f32,
}

impl Sampler {
	fn new(
		kind: &str,
		seed: u32,
		frequency: Option<f32>,
		octaves: Option<u32>,
	) -> ByondResult<Self> {
		let mut noise = FastNoiseLite::with_seed(seed as i32);
		let (scale, bias) = match kind {
			"perlin" => {
				noise.set_noise_type(Some(NoiseType::Perlin));
				(SQRT_2 / 2.0, 0.5)
			}
			"simplex" => {
				noise.set_noise_type(Some(NoiseType::OpenSimplex2));
				(0.5, 0.5)
			}
			"worley" => {
				noise.set_noise_type(Some(NoiseType::Cellular));
				noise.set_cellular_distance_function(Some(CellularDistanceFunction::Euclidean));
				noise.set_cellular_return_type(Some(CellularReturnType::Distance2Sub));
				(1.0, 1.0)
			}
			other => {
				return Err(ByondError::boxed(std::io::Error::other(format!(
					"unknown noise kind {other:?}, expected \"perlin\", \"simplex\" or \"worley\""
				))));
			}
		};
		noise.set_frequency(frequency);
		if let Some(octaves @ 2..) = octaves {
			noise.set_fractal_type(Some(FractalType::FBm));
			noise.set_fractal_octaves(Some(octaves as i32));
		}
		Ok(Self { noise, scale, bias })
	}

	fn at(&self, x: f32, y: f32) -> f32 {
		(self.noise.get_noise_2d(x, y) * self.scale + self.bias).clamp(0.0, 1.0)
	}
}

/// Noise value at one point, from 0 to 1.
/// `kind` is "perlin", "simplex" or "worley" (distance to the second-nearest
/// feature point minus the nearest, so it is near 0 along cell borders).
/// `frequency` is how many noise features fit in one unit, default 0.01.
/// `octaves` above 1 layers that many passes of finer detail.
/// Runtimes if `kind` is anything else.
#[byond_fn]
pub fn noise_at(
	kind: String,
	seed: u32,
	x: f32,
	y: f32,
	frequency: Option<f32>,
	octaves: Option<u32>,
) -> ByondResult<f32> {
	Sampler::new(&kind, seed, frequency, octaves).map(|sampler| sampler.at(x, y))
}

/// Generates a grid of noise values, each from 0 to 1.
///
/// Returns a list, row by row: cell `(x, y)`, counting from 1, is at index
/// `(y - 1) * width + x` and matches `aneri_noise_at(kind, seed, x, y, frequency, octaves)`.
///
/// Arguments:
/// * kind: "perlin", "simplex" or "worley".
///   Runtimes for anything else.
/// * seed: Seed for the noise.
/// * width: The width of the grid.
/// * height: The height of the grid.
/// * frequency: How many noise features fit in one unit (default 0.01).
/// * octaves: Above 1, layers that many passes of finer detail.
///
/// Rows are computed in parallel.
#[byond_fn(name = "aneri_raw_noise_grid")]
pub fn noise_grid(
	kind: String,
	seed: u32,
	width: u32,
	height: u32,
	frequency: Option<f32>,
	octaves: Option<u32>,
) -> ByondResult<Vec<f32>> {
	let sampler = Sampler::new(&kind, seed, frequency, octaves)?;
	let (width, height) = (width as usize, height as usize);
	let mut values = vec![0.0; width * height];
	if values.is_empty() {
		return Ok(values);
	}

	let workers = thread::available_parallelism().map_or(1, NonZeroUsize::get);
	let rows_per_worker = height.div_ceil(workers);
	thread::scope(|scope| {
		for (index, chunk) in values.chunks_mut(rows_per_worker * width).enumerate() {
			let sampler = &sampler;
			scope.spawn(move || {
				let first_cell = index * rows_per_worker * width;
				for (offset, value) in chunk.iter_mut().enumerate() {
					let cell = first_cell + offset;
					*value = sampler.at((cell % width + 1) as f32, (cell / width + 1) as f32);
				}
			});
		}
	});
	Ok(values)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn grid_matches_single_points() {
		for kind in ["perlin", "simplex", "worley"] {
			let (width, height) = (37, 23);
			let grid = noise_grid(kind.to_owned(), 7, width, height, Some(0.07), Some(3)).unwrap();
			assert_eq!(grid.len(), (width * height) as usize);
			for y in 1..=height {
				for x in 1..=width {
					let expected =
						noise_at(kind.to_owned(), 7, x as f32, y as f32, Some(0.07), Some(3))
							.unwrap();
					assert_eq!(
						grid[((y - 1) * width + x - 1) as usize],
						expected,
						"{kind} at {x},{y}"
					);
				}
			}
		}
	}
}
