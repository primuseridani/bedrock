// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::graphics::{InitGraphicsContext, Vec2, Vertex};
use crate::log::log;

use zerocopy::IntoBytes;

impl InitGraphicsContext {
	#[inline]
	pub fn resize(&mut self, (width, height): (u32, u32)) {
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

		let x_radius = (x_factor / 2.0 * 3.0) as f32;
		let y_radius = (y_factor / 2.0 * 3.0) as f32;

		let clip_top             = Vec2::new( 0.0,  3.0);
		let clip_bottom_left     = Vec2::new(-3.0, -3.0);
		let clip_bottom_right    = Vec2::new( 3.0, -3.0);

		let texture_top          = Vec2::new(0.5,            0.5 - y_radius);
		let texture_bottom_left  = Vec2::new(0.5 - x_radius, 0.5 + y_radius);
		let texture_bottom_right = Vec2::new(0.5 + x_radius, 0.5 + y_radius);

		let vertices = [
			Vertex {
				clip:    clip_top,
				texture: texture_top,
			},

			Vertex {
				clip:    clip_bottom_left,
				texture: texture_bottom_left,
			},

			Vertex {
				clip:    clip_bottom_right,
				texture: texture_bottom_right,
			},
		];

		self.queue.write_buffer(&self.vertex_buf, 0x0, vertices.as_bytes());
	}
}
