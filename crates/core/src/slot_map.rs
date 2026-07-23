// SPDX-License-Identifier: MPL-2.0
use crate::ByondSlotKey;
use parking_lot::RwLock;
use slotmap::SlotMap;
use std::sync::LazyLock;

const DEFAULT_CAPACITY: usize = 16;

pub struct ByondSlotMap<T> {
	instances: LazyLock<RwLock<SlotMap<ByondSlotKey, T>>>,
}

impl<T> Default for ByondSlotMap<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T> ByondSlotMap<T> {
	pub const fn new() -> Self {
		Self {
			instances: LazyLock::new(|| {
				RwLock::new(SlotMap::with_capacity_and_key(DEFAULT_CAPACITY))
			}),
		}
	}

	pub fn insert(&self, value: T) -> ByondSlotKey {
		self.instances.write().insert(value)
	}

	pub fn remove(&self, key: ByondSlotKey) -> Option<T> {
		self.instances.write().remove(key)
	}

	pub fn with<Output>(&self, key: ByondSlotKey, f: impl FnOnce(&T) -> Output) -> Option<Output> {
		self.instances.read().get(key).map(f)
	}

	pub fn with_mut<Output>(
		&self,
		key: ByondSlotKey,
		f: impl FnOnce(&mut T) -> Output,
	) -> Option<Output> {
		self.instances.write().get_mut(key).map(f)
	}

	pub fn clear(&self) {
		let mut instances = self.instances.write();
		if instances.capacity() > DEFAULT_CAPACITY {
			// don't use clear(), so we reclaim memory
			*instances = SlotMap::with_capacity_and_key(DEFAULT_CAPACITY);
		} else {
			// if we're at the default capacity, it's a waste of time to reallocate
			instances.clear();
		}
	}
}
