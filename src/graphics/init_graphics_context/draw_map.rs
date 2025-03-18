// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::graphics::{InitGraphicsContext, Rgba};
use crate::level::{Block, Material};
use crate::map::Map;

impl InitGraphicsContext {
	pub fn draw_map(&mut self, map: &Map, (pan_x, pan_y): (u32, u32), scale: u32) {
		self.texture_buf.fill(Rgba::TRANSPARENT);

		let global_scale = f64::from(Self::TEXTURE_WIDTH);
		let local_scale  = f64::from(scale);

		let off_x = f64::from(pan_x) - local_scale / 2.0;
		let off_y = f64::from(pan_y) - local_scale / 2.0;

		for y in 0x0..Self::TEXTURE_WIDTH {
			for x in 0x0..Self::TEXTURE_WIDTH {
				let Some(block) = ({
					let mut x = f64::from(x);
					let mut y = global_scale - f64::from(y);

					x *= local_scale / global_scale;
					y *= local_scale / global_scale;

					x += off_x;
					y += off_y;

					map.sample(x, y)
				}) else {
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
			Rgba::from_u32(0x171717FF),
			Rgba::from_u32(0x3A3A3AFF),
			Rgba::from_u32(0x2A2A2AFF),
			Rgba::from_u32(0x1F1F1FFF),
		],

		Material::Bedrock => [
			Rgba::from_u32(0x252525FF),
			Rgba::from_u32(0xD7D7D7FF),
			Rgba::from_u32(0x4B4B4BFF),
			Rgba::from_u32(0xA2A2A2FF),
		],

		Material::Clay => [
			Rgba::DEFAULT,
			Rgba::DEFAULT,
			Rgba::DEFAULT,
			Rgba::DEFAULT,
		],

		Material::Dirt => [
			Rgba::from_u32(0x4F2D11FF),
			Rgba::from_u32(0x4F341DFF),
			Rgba::from_u32(0x53361DFF),
			Rgba::from_u32(0x4C2F16FF),
		],

		Material::Fire => [
			Rgba::DEFAULT,
			Rgba::DEFAULT,
			Rgba::DEFAULT,
			Rgba::DEFAULT,
		],

		Material::Glass => [
			Rgba::DEFAULT,
			Rgba::DEFAULT,
			Rgba::DEFAULT,
			Rgba::DEFAULT,
		],

		Material::Granite => [
			Rgba::DEFAULT,
			Rgba::DEFAULT,
			Rgba::DEFAULT,
			Rgba::DEFAULT,
		],

		Material::Grass => [
			Rgba::from_u32(0x9AB34EFF),
			Rgba::from_u32(0x6D913FFF),
			Rgba::from_u32(0x98AA39FF),
			Rgba::from_u32(0xB3CC60FF),
		],

		Material::Gravel => [
			Rgba::DEFAULT,
			Rgba::DEFAULT,
			Rgba::DEFAULT,
			Rgba::DEFAULT,
		],

		Material::Ice => [
			Rgba::DEFAULT,
			Rgba::DEFAULT,
			Rgba::DEFAULT,
			Rgba::DEFAULT,
		],

		Material::Limestone => [
			Rgba::DEFAULT,
			Rgba::DEFAULT,
			Rgba::DEFAULT,
			Rgba::DEFAULT,
		],

		Material::Magma => [
			Rgba::from_u32(0xFF4800FF),
			Rgba::from_u32(0xFF8200FF),
			Rgba::from_u32(0xFFA000FF),
			Rgba::from_u32(0xFEB300FF),
		],

		Material::Marble => [
			Rgba::DEFAULT,
			Rgba::DEFAULT,
			Rgba::DEFAULT,
			Rgba::DEFAULT,
		],

		Material::Sand => [
			Rgba::from_u32(0xF5D88FFF),
			Rgba::from_u32(0xF8E5B4FF),
			Rgba::from_u32(0xFCEDC5FF),
			Rgba::from_u32(0xF7D479FF),
		],

		Material::Stone => [
			Rgba::from_u32(0x6D6D6DFF),
			Rgba::from_u32(0x797979FF),
			Rgba::from_u32(0x616161FF),
			Rgba::from_u32(0x595959FF),
		],

		Material::Water => [
			Rgba::from_u32(0x286DC3BF),
			Rgba::from_u32(0x2565B8BF),
			Rgba::from_u32(0x1F69BCBF),
			Rgba::from_u32(0x2566B4BF),
		],

		Material::Wood => [
			Rgba::DEFAULT,
			Rgba::DEFAULT,
			Rgba::DEFAULT,
			Rgba::DEFAULT,
		],
	};

	let seed = block.seed();
	colours[seed as usize]
}
