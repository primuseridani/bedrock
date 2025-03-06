// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::graphics::{GraphicsContext, Vec2, Vertex, MAX_VIEW_SCALE};
use crate::level::Map;

use rand::{Rng, rng};
use std::iter;
use zerocopy::IntoBytes;

impl GraphicsContext {
	pub fn render(&mut self, _map: &Map, scale: u32) {
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

		rng().fill(self.texture_buf.as_mut_bytes());

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
				bytes_per_row:  Some(MAX_VIEW_SCALE * 0x4),
				rows_per_image: Some(MAX_VIEW_SCALE),
			},
			Self::TEXTURE_EXTENT,
		);

		let (view_width, view_height) = {
			let xy = scale as f32;

			let (x_factor, y_factor) = self.scale_factor;

			let x = xy * x_factor;
			let y = xy * y_factor;

			(x, y)
		};

		let top_left     = Vec2::new(       0.0,         0.0);
		let bottom_left  = Vec2::new(       0.0, view_height);
		let bottom_right = Vec2::new(view_width, view_height);
		let top_right    = Vec2::new(view_width,         0.0);

		let vertices = [
			Vertex {
				clip:    Vec2::new(-1.0,  1.0),
				texture: top_left,
				..Default::default()
			},

			Vertex {
				clip:    Vec2::new(-1.0, -1.0),
				texture: bottom_left,
				..Default::default()
			},

			Vertex {
				clip:    Vec2::new( 1.0, -1.0),
				texture: bottom_right,
				..Default::default()
			},

			Vertex {
				clip:    Vec2::new( 1.0,  1.0),
				texture: top_right,
				..Default::default()
			},
		];

		self.queue.write_buffer(&self.vertex_buf, 0x0, vertices.as_bytes());

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
							load: wgpu::LoadOp::Load,

							store: wgpu::StoreOp::Store,
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
