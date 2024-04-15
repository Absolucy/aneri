// SPDX-License-Identifier: MPL-2.0
#![warn(
	clippy::correctness,
	clippy::suspicious,
	clippy::complexity,
	clippy::perf,
	clippy::style
)]
#[macro_use]
extern crate meowtonin;

pub mod json;
pub mod string;
pub mod toml;
pub mod uuid;

pub fn cleanup() {}
