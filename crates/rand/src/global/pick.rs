// SPDX-License-Identifier: MPL-2.0
use super::global;
use crate::shared;
use meowtonin::ByondResult;

#[byond_fn]
pub fn pick_weighted(options: Vec<f32>, secure: Option<bool>) -> ByondResult<Option<usize>> {
	shared::pick_weighted(&mut global(secure), options)
}
