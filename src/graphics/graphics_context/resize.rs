// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::graphics::GraphicsContext;

impl GraphicsContext {
	#[inline]
	pub fn resize(&mut self, width: u32, height: u32) {
		eprintln!("resizing graphics context to `{width}*{height}`");

		self.surface_config.width  = width;
		self.surface_config.height = height;

		self.surface.configure(&self.device, &self.surface_config);
	}
}
