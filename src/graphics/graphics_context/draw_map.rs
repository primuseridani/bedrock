// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::graphics::{GraphicsContext, Rgba};
use crate::level::{Block, Material};
use crate::map::Map;

impl GraphicsContext {
	pub fn draw_map(&mut self, map: &Map, (pan_x, pan_y): (u32, u32), scale: u32) {
		self.texture_buf.fill(Default::default());

		let global_scale = f64::from(Self::TEXTURE_WIDTH);
		let local_scale  = f64::from(scale);

		let off_x = f64::from(pan_x) - local_scale / 2.0;
		let off_y = f64::from(pan_y) - local_scale / 2.0;

		let (x_range, y_range) = {
			let (map_width, map_height) = map.size().get();

			let max_x = f64::from(map_width);
			let max_y = f64::from(map_height);

			let x_range = 0.0..max_x;
			let y_range = 0.0..max_y;

			(x_range, y_range)
		};

		for y in 0x0..Self::TEXTURE_WIDTH {
			for x in 0x0..Self::TEXTURE_WIDTH {
				let read_block = || -> Option<Block> {
					let mut x = f64::from(x);
					let mut y = global_scale - f64::from(y);

					x *= local_scale / global_scale;
					y *= local_scale / global_scale;

					x += off_x;
					y += off_y;

					if !x_range.contains(&x) {
						return None;
					}

					if !y_range.contains(&y) {
						return None;
					}

					let x = x as u32;
					let y = y as u32;

					map.get(x, y).copied()
				};

				let Some(block) = read_block() else {
					continue;
				};

				let colour = block_colour(block);

				let index = y as usize * Self::TEXTURE_WIDTH as usize + x as usize;

				self.texture_buf[index] = colour;
			}
		}
	}
}

#[inline]
#[must_use]
const fn block_colour(block: Block) -> Rgba {
	let colours: [_; 0x4] = match block.material() {
		Material::Air => [
			Rgba::from_u32(0x00000000),
			Rgba::from_u32(0x00000000),
			Rgba::from_u32(0x00000000),
			Rgba::from_u32(0x00000000),
		],

		Material::Basalt => [
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
		],

		Material::Bedrock => [
			Rgba::from_u32(0x252525FF),
			Rgba::from_u32(0xD7D7D7FF),
			Rgba::from_u32(0x4B4B4BFF),
			Rgba::from_u32(0xA2A2A2FF),
		],

		Material::Clay => [
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
		],

		Material::Dirt => [
			Rgba::from_u32(0x8E613DFF),
			Rgba::from_u32(0x885533FF),
			Rgba::from_u32(0x856543FF),
			Rgba::from_u32(0xA26839FF),
		],

		Material::Fire => [
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
		],

		Material::Glass => [
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
		],

		Material::Granite => [
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
		],

		Material::Grass => [
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
		],

		Material::Gravel => [
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
		],

		Material::Ice => [
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
		],

		Material::Limestone => [
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
		],

		Material::Magma => [
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
		],

		Material::Marble => [
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
		],

		Material::Sand => [
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
		],

		Material::Stone => [
			Rgba::from_u32(0x6D6D6DFF),
			Rgba::from_u32(0x797979FF),
			Rgba::from_u32(0x616161FF),
			Rgba::from_u32(0x595959FF),
		],

		Material::Water => [
			Rgba::from_u32(0x2368BCBF),
			Rgba::from_u32(0x306BABBF),
			Rgba::from_u32(0x4C75B5BF),
			Rgba::from_u32(0x3C64ABBF),
		],

		Material::Wood => [
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
			Rgba::from_u32(0xFF00FFFF),
		],
	};

	let seed = block.seed();
	colours[seed as usize]
}
