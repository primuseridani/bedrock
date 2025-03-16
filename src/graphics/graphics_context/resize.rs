// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::graphics::{GraphicsContext, Vec2, Vertex};
use crate::log::log;

use zerocopy::IntoBytes;

impl GraphicsContext {
	#[inline]
	pub fn resize(&mut self, width: u32, height: u32) {
		log!(debug, "resizing graphics context to `{width}*{height}`");

		self.surface_config.width  = width;
		self.surface_config.height = height;

		self.surface.configure(&self.device, &self.surface_config);

		let (x_factor, y_factor) = if width >= height {
			let x_factor = 1.0;
			let y_factor = f64::from(height) / f64::from(width);

			(x_factor, y_factor)
		} else {
			let x_factor = f64::from(width) / f64::from(height);
			let y_factor = 1.0;

			(x_factor, y_factor)
		};

		let x_radius = (x_factor / 2.0) as f32;
		let y_radius = (y_factor / 2.0) as f32;

		let top_left     = Vec2::new(0.5 - x_radius, 0.5 - y_radius);
		let bottom_left  = Vec2::new(0.5 - x_radius, 0.5 + y_radius);
		let bottom_right = Vec2::new(0.5 + x_radius, 0.5 + y_radius);
		let top_right    = Vec2::new(0.5 + x_radius, 0.5 - y_radius);

		let vertices = [
			Vertex {
				clip:    Vec2::new(-1.0,  1.0),
				texture: top_left,
			},

			Vertex {
				clip:    Vec2::new(-1.0, -1.0),
				texture: bottom_left,
			},

			Vertex {
				clip:    Vec2::new( 1.0, -1.0),
				texture: bottom_right,
			},

			Vertex {
				clip:    Vec2::new( 1.0,  1.0),
				texture: top_right,
			},
		];

		self.queue.write_buffer(&self.vertex_buf, 0x0, vertices.as_bytes());
	}
}
