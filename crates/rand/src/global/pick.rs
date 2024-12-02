// SPDX-License-Identifier: MPL-2.0
use super::global;
use crate::shared;
use meowtonin::{byond_fn, ByondResult, ByondValue};

#[byond_fn]
pub fn pick(options: ByondValue, secure: Option<bool>) -> ByondResult<ByondValue> {
	shared::pick(&mut global(secure), options)
}

#[byond_fn]
pub fn pick_weighted(options: ByondValue, secure: Option<bool>) -> ByondResult<ByondValue> {
	shared::pick_weighted(&mut global(secure), options)
}
