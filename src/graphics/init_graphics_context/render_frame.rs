// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::graphics::InitGraphicsContext;

use polywave::www::Html;
use std::iter;
use zerocopy::IntoBytes;

impl InitGraphicsContext {
	pub fn render_frame(&mut self, background: Html) {
		let background = {
			let to_f64 = |colour: u8| -> f64 {
				f64::from(colour) / f64::from(u8::MAX)
			};

			let (colour, a) = background.to_s_rgba().detach();

			let a = to_f64(a);

			let (r, g, b) = colour
				.map(to_f64)
				.untransfer()
				.get();

			wgpu::Color { r, g, b, a }
		};

		let output = match self.surface.get_current_texture() {
			Ok(output) => output,

			Err(e) => panic!("unable to get current texture: {e}"),
		};

		let view = {
			let descriptor = wgpu::TextureViewDescriptor {
				label: Some("texture view"),

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
				bytes_per_row:  Some(size_of::<Html>() as u32 * Self::TEXTURE_WIDTH),
				rows_per_image: Some(Self::TEXTURE_WIDTH),
			},
			Self::TEXTURE_EXTENT,
		);

		let mut encoder = {
			let descriptor = wgpu::CommandEncoderDescriptor {
				label: Some("command encoder"),
			};

			self.device.create_command_encoder(&descriptor)
		};

		{
			let descriptor = wgpu::RenderPassDescriptor {
				label: Some("render pass"),

				color_attachments: &[
					Some(wgpu::RenderPassColorAttachment {
						view:           &view,
						resolve_target: None,

						ops: wgpu::Operations {
							load: wgpu::LoadOp::Clear(background),

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
