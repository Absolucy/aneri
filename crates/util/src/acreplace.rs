// SPDX-License-Identifier: MPL-2.0

use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind, StartKind};
use aneri_core::ByondSlotKey;
use meowtonin::{ByondResult, ByondValue};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use slotmap::SlotMap;

#[cfg_attr(target_pointer_width = "32", repr(align(64)))]
#[cfg_attr(target_pointer_width = "64", repr(align(128)))]
struct Replacements {
	automaton: AhoCorasick,
	replacements: Vec<String>,
}

static INSTANCES: Lazy<RwLock<SlotMap<ByondSlotKey, Replacements>>> = Lazy::new(RwLock::default);

pub(crate) fn free_instances() {
	*INSTANCES.write() = SlotMap::with_key();
}

#[byond_fn]
pub fn acreplace_new(mut src: ByondValue) -> ByondResult<()> {
	INSTANCES.write().insert(todo!()).save(&mut src)
}

#[byond_fn]
pub fn acreplace_del(src: ByondSlotKey) -> bool {
	INSTANCES.write().remove(src).is_some()
}
