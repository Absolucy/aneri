// SPDX-License-Identifier: MPL-2.0
use meowtonin::{ByondResult, ByondValue, FromByond};
use slotmap::{new_key_type, Key, KeyData};

new_key_type! { pub struct ByondSlotKey; }

impl ByondSlotKey {
	const HIGH_VAR: &'static str = "__aneri_key_high";
	const LOW_VAR: &'static str = "__aneri_key_low";

	pub fn save(&self, value: &mut ByondValue) -> ByondResult<()> {
		let data = self.data().as_ffi();
		value.write_var(Self::HIGH_VAR, f32::from_bits((data >> 32) as u32))?;
		value.write_var(Self::LOW_VAR, f32::from_bits((data & 0xFFFFFFFF) as u32))?;
		Ok(())
	}
}

impl FromByond for ByondSlotKey {
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		let high = value.read_var::<_, f32>(Self::HIGH_VAR)?.to_bits();
		let low = value.read_var::<_, f32>(Self::LOW_VAR)?.to_bits();
		Ok(KeyData::from_ffi(((high as u64) << 32) | (low as u64)).into())
	}
}
