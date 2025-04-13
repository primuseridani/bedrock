// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::graphics::InitGraphicsContext;
use crate::level::{Block, Material};
use crate::map::Map;

use polywave::www::Html;
use zerocopy::IntoBytes;

impl InitGraphicsContext {
	pub fn draw_map(&mut self, map: &Map, (pan_x, pan_y): (u32, u32), scale: u32) {
		self.texture_buf.fill(Html::TRANSPARENT);

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

		self.queue.write_texture(
			wgpu::TexelCopyTextureInfo {
				texture:   &self.texture,
				mip_level: 0x0,
				origin:    wgpu::Origin3d::ZERO,
				aspect:    wgpu::TextureAspect::All,
			},
			self.texture_buf.as_bytes(),
			wgpu::TexelCopyBufferLayout {
				offset:         0x0,
				bytes_per_row:  Some(size_of::<Html>() as u32 * Self::TEXTURE_WIDTH),
				rows_per_image: Some(Self::TEXTURE_WIDTH),
			},
			Self::TEXTURE_EXTENT,
		);
	}
}

#[expect(clippy::match_same_arms)]
#[inline]
#[must_use]
const fn block_colour(block: Block) -> Html {
	const DEFAULT_COLOUR: Html = Html::from_u32(0xFF00FFFF);

	let colours: [_; 0x4] = match block.material() {
		Material::Air => [
			Html::from_u32(0x00000000),
			Html::from_u32(0x00000000),
			Html::from_u32(0x00000000),
			Html::from_u32(0x00000000),
		],

		Material::Basalt => [
			Html::from_u32(0x171717FF),
			Html::from_u32(0x3A3A3AFF),
			Html::from_u32(0x2A2A2AFF),
			Html::from_u32(0x1F1F1FFF),
		],

		Material::Bedrock => [
			Html::from_u32(0x252525FF),
			Html::from_u32(0xD7D7D7FF),
			Html::from_u32(0x4B4B4BFF),
			Html::from_u32(0xA2A2A2FF),
		],

		Material::Clay => [
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
		],

		Material::Dirt => [
			Html::from_u32(0x4F2D11FF),
			Html::from_u32(0x4F341DFF),
			Html::from_u32(0x53361DFF),
			Html::from_u32(0x4C2F16FF),
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
			Html::from_u32(0x9AB34EFF),
			Html::from_u32(0x6D913FFF),
			Html::from_u32(0x98AA39FF),
			Html::from_u32(0xB3CC60FF),
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
			Html::from_u32(0xFF4800FF),
			Html::from_u32(0xFF8200FF),
			Html::from_u32(0xFFA000FF),
			Html::from_u32(0xFEB300FF),
		],

		Material::Marble => [
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
			DEFAULT_COLOUR,
		],

		Material::Sand => [
			Html::from_u32(0xF5D88FFF),
			Html::from_u32(0xF8E5B4FF),
			Html::from_u32(0xFCEDC5FF),
			Html::from_u32(0xF7D479FF),
		],

		Material::Rock => [
			Html::from_u32(0x6D6D6DFF),
			Html::from_u32(0x797979FF),
			Html::from_u32(0x616161FF),
			Html::from_u32(0x595959FF),
		],

		Material::Water => [
			Html::from_u32(0x286DC3BF),
			Html::from_u32(0x2565B8BF),
			Html::from_u32(0x1F69BCBF),
			Html::from_u32(0x2566B4BF),
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
