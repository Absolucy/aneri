// SPDX-License-Identifier: MPL-2.0
#![cfg_attr(debug_assertions, allow(unused_imports))]
#![warn(
	clippy::correctness,
	clippy::suspicious,
	clippy::complexity,
	clippy::perf,
	clippy::style
)]
#[macro_use]
extern crate meowtonin;
#[macro_use]
extern crate static_init;

pub mod opts;
pub mod pool;
pub mod util;
