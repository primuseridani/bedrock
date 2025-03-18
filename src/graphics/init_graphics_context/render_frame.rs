// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::graphics::{InitGraphicsContext, Rgba};

use std::iter;
use zerocopy::IntoBytes;

impl InitGraphicsContext {
	pub fn render_frame(&mut self, background: Rgba) {
		let output = match self.surface.get_current_texture() {
			Ok(output) => output,

			Err(e) => panic!("unable to get current texture: {e}"),
		};

		let view = {
			let descriptor = wgpu::TextureViewDescriptor {
				label: Some("main"),

				..Default::default()
			};

			output.texture.create_view(&descriptor)
		};

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
				bytes_per_row:  Some(size_of::<Rgba>() as u32 * Self::TEXTURE_WIDTH),
				rows_per_image: Some(Self::TEXTURE_WIDTH),
			},
			Self::TEXTURE_EXTENT,
		);

		let mut encoder = {
			let descriptor = wgpu::CommandEncoderDescriptor {
				label: Some("main"),
			};

			self.device.create_command_encoder(&descriptor)
		};

		{
			let descriptor = wgpu::RenderPassDescriptor {
				label: Some("main"),

				color_attachments: &[
					Some(wgpu::RenderPassColorAttachment {
						view:           &view,
						resolve_target: None,

						ops: wgpu::Operations {
							load: wgpu::LoadOp::Clear(background.to_wgpu_color_lossy()),

							store: wgpu::StoreOp::Store,
						},
					}),
				],

				..Default::default()
			};

			let mut pass = encoder.begin_render_pass(&descriptor);

			pass.set_bind_group(0x0, &self.texture_bind_group, Default::default());
			pass.set_vertex_buffer(0x0, self.vertex_buf.slice(..));
			pass.set_pipeline(&self.pipeline);

			pass.draw(0x0..self.vertex_count, 0x0..0x1);
		}

		self.queue.submit(iter::once(encoder.finish()));

		output.present();
	}
}
