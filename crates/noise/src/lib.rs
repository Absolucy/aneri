// SPDX-License-Identifier: MPL-2.0
use rand::{SeedableRng, rngs::SmallRng};

pub mod automata;
pub mod cave;
pub mod gradient;
pub mod poisson;

fn make_rng(seed: Option<u32>) -> SmallRng {
	seed.map_or_else(rand::make_rng, |seed| SmallRng::seed_from_u64(seed.into()))
}
