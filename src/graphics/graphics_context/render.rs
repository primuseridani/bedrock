// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::graphics::{GraphicsContext, MAX_VIEW_SCALE};
use crate::level::Map;

use rand::{Rng, rng};
use std::iter;
use wgpu::{
	CommandEncoderDescriptor,
	LoadOp,
	Operations,
	Origin3d,
	RenderPassColorAttachment,
	RenderPassDescriptor,
	StoreOp,
	TexelCopyTextureInfo,
	TextureAspect,
	TexelCopyBufferLayout,
	TextureViewDescriptor,
};
use zerocopy::IntoBytes;

impl GraphicsContext {
	pub fn render(&mut self, _map: &Map, _scale: u32) {
		let output = match self.surface.get_current_texture() {
			Ok(output) => output,

			Err(e) => panic!("unable to get current texture: {e}"),
		};

		let view = {
			let descriptor = TextureViewDescriptor {
				label: Some("main"),

				..Default::default()
			};

			output.texture.create_view(&descriptor)
		};

		rng().fill(self.texture_buf.as_mut_bytes());

		self.queue.write_texture(
			TexelCopyTextureInfo {
				texture:   &self.texture,
				mip_level: 0x0,
				origin:    Origin3d::ZERO,
				aspect:    TextureAspect::All,
			},
			self.texture_buf.as_bytes(),
			TexelCopyBufferLayout {
				offset:         0x0,
				bytes_per_row:  Some(MAX_VIEW_SCALE * 0x4),
				rows_per_image: Some(MAX_VIEW_SCALE),
			},
			Self::TEXTURE_EXTENT,
		);

		let mut encoder = {
			let descriptor = CommandEncoderDescriptor {
				label: Some("main"),
			};

			self.device.create_command_encoder(&descriptor)
		};

		{
			let descriptor = RenderPassDescriptor {
				label: Some("main"),

				color_attachments: &[
					Some(RenderPassColorAttachment {
						view:           &view,
						resolve_target: None,

						ops: Operations {
							load: LoadOp::Load,

							store: StoreOp::Store,
						},
					}),
				],

				..Default::default()
			};

			let mut pass = encoder.begin_render_pass(&descriptor);

			pass.set_bind_group(0x0, &self.texture_bind_group, Default::default());
			pass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint16);
			pass.set_vertex_buffer(0x0, self.vertex_buf.slice(..));
			pass.set_pipeline(&self.pipeline);

			pass.draw_indexed(0x0..self.index_count, 0x0, 0x0..0x1);
		}

		self.queue.submit(iter::once(encoder.finish()));

		output.present();
	}
}
