// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::{graphics::{GraphicsContext, MAX_VIEW_SCALE}, log::log};

impl GraphicsContext {
	#[inline]
	pub fn resize(&mut self, width: u32, height: u32) {
		log!(debug, "resizing graphics context to `{width}*{height}`");

		let (mut x_factor, mut y_factor) = if width >= height {
			let x_factor = 1.0;
			let y_factor = f64::from(height) / f64::from(width);

			(x_factor, y_factor)
		} else {
			let x_factor = f64::from(width) / f64::from(height);
			let y_factor = 1.0;

			(x_factor, y_factor)
		};

		let pixel_width = 1.0 / f64::from(MAX_VIEW_SCALE);

		x_factor *= pixel_width;
		y_factor *= pixel_width;

		self.scale_factor = (x_factor as f32, y_factor as f32);

		self.surface_config.width  = width;
		self.surface_config.height = height;

		self.surface.configure(&self.device, &self.surface_config);
	}
}
