// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::graphics::InitGraphicsContext;
use crate::level::{Block, Material};
use crate::map::Map;

use polywave::colour::Css;

impl InitGraphicsContext {
	pub fn draw_map(&mut self, map: &Map, (pan_x, pan_y): (u32, u32), scale: u32) {
		self.texture_buf.fill(Css::TRANSPARENT);

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
const fn block_colour(block: Block) -> Css {
	const DEFAULT_COLOUR: Css = Css::from_u32(0xFF00FFFF);

	let colours: [_; 0x4] = match block.material() {
		Material::Air => [
			Css::from_u32(0x00000000),
			Css::from_u32(0x00000000),
			Css::from_u32(0x00000000),
			Css::from_u32(0x00000000),
		],

		Material::Basalt => [
			Css::from_u32(0x171717FF),
			Css::from_u32(0x3A3A3AFF),
			Css::from_u32(0x2A2A2AFF),
			Css::from_u32(0x1F1F1FFF),
		],

		Material::Bedrock => [
			Css::from_u32(0x252525FF),
			Css::from_u32(0xD7D7D7FF),
			Css::from_u32(0x4B4B4BFF),
			Css::from_u32(0xA2A2A2FF),
		],

		Material::Clay => [
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
		],

		Material::Dirt => [
			Css::from_u32(0x4F2D11FF),
			Css::from_u32(0x4F341DFF),
			Css::from_u32(0x53361DFF),
			Css::from_u32(0x4C2F16FF),
		],

		Material::Fire => [
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
		],

		Material::Glass => [
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
		],

		Material::Granite => [
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
		],

		Material::Grass => [
			Css::from_u32(0x9AB34EFF),
			Css::from_u32(0x6D913FFF),
			Css::from_u32(0x98AA39FF),
			Css::from_u32(0xB3CC60FF),
		],

		Material::Gravel => [
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
		],

		Material::Ice => [
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
		],

		Material::Limestone => [
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
		],

		Material::Magma => [
			Css::from_u32(0xFF4800FF),
			Css::from_u32(0xFF8200FF),
			Css::from_u32(0xFFA000FF),
			Css::from_u32(0xFEB300FF),
		],

		Material::Marble => [
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
		],

		Material::Sand => [
			Css::from_u32(0xF5D88FFF),
			Css::from_u32(0xF8E5B4FF),
			Css::from_u32(0xFCEDC5FF),
			Css::from_u32(0xF7D479FF),
		],

		Material::Stone => [
			Css::from_u32(0x6D6D6DFF),
			Css::from_u32(0x797979FF),
			Css::from_u32(0x616161FF),
			Css::from_u32(0x595959FF),
		],

		Material::Water => [
			Css::from_u32(0x286DC3BF),
			Css::from_u32(0x2565B8BF),
			Css::from_u32(0x1F69BCBF),
			Css::from_u32(0x2566B4BF),
		],

		Material::Wood => [
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
		],
	};

	let seed = block.seed();
	colours[seed as usize]
}
