// SPDX-License-Identifier: MPL-2.0
mod corner;
mod rgb;
pub(crate) mod util;

use self::{corner::CornerRgb, rgb::Rgb, util::round_to};
use meowtonin::{ByondResult, ByondValue, FromByond, byond_fn};

const LIGHTING_SOFT_THRESHOLD: f32 = 0.05;
const LIGHTING_ROUND_VALUE: f32 = 128.0;

//fn get_old_values(lighting_corner: ByondValue)

#[byond_fn]
pub fn experiment_lighting_corner_update_objects(
	mut lighting_corner: ByondValue,
	mut no_update_ptr: ByondValue,
) -> f32 {
	let old_cache = Rgb::read_byond(&lighting_corner, "cache_r", "cache_g", "cache_b")
		.expect("failed to read cache_r/g/b vars");
	let old_add = Rgb::read_byond(&lighting_corner, "add_r", "add_g", "add_b")
		.expect("failed to read add_r/g/b vars");

	let lum = Rgb::read_byond(&lighting_corner, "lum_r", "lum_g", "lum_b")
		.expect("failed to read lum_r/g/b vars");

	let largest_color_luminosity = lum.largest_channel();

	let factor = if largest_color_luminosity > 1.0 {
		(1.0 / (largest_color_luminosity as f64)) as f32
	} else if largest_color_luminosity < LIGHTING_SOFT_THRESHOLD {
		0.0
	} else {
		1.0
	};

	assert!(factor.is_finite());

	let cache = (lum * factor)
		.round_to(LIGHTING_ROUND_VALUE)
		.or_if_zero(LIGHTING_SOFT_THRESHOLD);

	cache
		.write_byond(&mut lighting_corner, "cache_r", "cache_g", "cache_b")
		.expect("failed to write cache_r/g/b");

	let add = ((lum - 1.35) * 0.3).clamp(0.0, 0.22);

	let applying_additive = add.largest_channel() > 0.03;
	let largest_color_luminosity = round_to(largest_color_luminosity, LIGHTING_ROUND_VALUE);

	assert!(largest_color_luminosity.is_finite());

	add.write_byond(&mut lighting_corner, "add_r", "add_g", "add_b")
		.expect("failed to write add_r/g/b");
	lighting_corner
		.write_var("applying_additive", applying_additive)
		.expect("could not write applying_additive");
	lighting_corner
		.write_var("largest_color_luminosity", largest_color_luminosity)
		.expect("could not write largest_color_luminosity");

	if old_cache != cache || old_add != add {
		no_update_ptr
			.write_pointer(false)
			.expect("failed to write *no_update_ptr = false");
	} else {
		no_update_ptr
			.write_pointer(true)
			.expect("failed to write *no_update_ptr = true");
	}

	factor
}

#[byond_fn]
pub fn experiment_lighting_object_update(
	red_corner: ByondValue,
	green_corner: ByondValue,
	blue_corner: ByondValue,
	alpha_corner: ByondValue,
	mut transparent_ptr: ByondValue,
	mut set_luminosity_ptr: ByondValue,
	mut light_underlay_color_ptr: ByondValue,
) {
	let max = {
		let (r, g, b, a) = read_from_4::<f32>(
			"largest_color_luminosity",
			&red_corner,
			&green_corner,
			&blue_corner,
			&alpha_corner,
		)
		.expect("failed to read largest_color_luminosity from corners");
		r.max(g).max(b).max(a)
	};

	let set_luminosity = max > LIGHTING_SOFT_THRESHOLD;
	set_luminosity_ptr
		.write_pointer(set_luminosity)
		.expect("failed to write set_luminosity_ptr");

	let cache = CornerRgb::read_byond(
		&red_corner,
		&green_corner,
		&blue_corner,
		&alpha_corner,
		"cache_r",
		"cache_g",
		"cache_b",
	)
	.expect("failed to read cache_r/g/b from corners");

	let (red_cache_r, red_cache_g, red_cache_b) = cache.red.to_u32();
	let (green_cache_r, green_cache_g, green_cache_b) = cache.green.to_u32();
	let (blue_cache_r, blue_cache_g, blue_cache_b) = cache.blue.to_u32();
	let (alpha_cache_r, alpha_cache_g, alpha_cache_b) = cache.alpha.to_u32();

	let transparent = (red_cache_r & green_cache_r & blue_cache_r & alpha_cache_r) != 0
		&& (red_cache_g
			+ green_cache_g
			+ blue_cache_g
			+ alpha_cache_g
			+ red_cache_b
			+ green_cache_b
			+ blue_cache_b
			+ alpha_cache_b
			== 8);
	transparent_ptr
		.write_pointer(transparent)
		.expect("failed to write *transparent_ptr");

	if !transparent && set_luminosity {
		let light_underlay_color = ByondValue::new_value([
			cache.red.r,
			cache.red.g,
			cache.red.b,
			0.0,
			cache.green.r,
			cache.green.g,
			cache.green.b,
			0.0,
			cache.blue.r,
			cache.blue.g,
			cache.blue.b,
			0.0,
			cache.alpha.r,
			cache.alpha.g,
			cache.alpha.b,
			0.0,
			0.0,
			0.0,
			0.0,
			1.0,
		])
		.expect("failed to init light_underlay_color");
		light_underlay_color_ptr
			.write_pointer(light_underlay_color)
			.expect("failed to write *light_underlay_color_ptr");
	}
}

fn read_from_4<Value>(
	name: &str,
	a: &ByondValue,
	b: &ByondValue,
	c: &ByondValue,
	d: &ByondValue,
) -> ByondResult<(Value, Value, Value, Value)>
where
	Value: FromByond,
{
	Ok((
		a.read_var(name)?,
		b.read_var(name)?,
		c.read_var(name)?,
		d.read_var(name)?,
	))
}
