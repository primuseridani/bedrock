// Copyright 2025 Gabriel Bjørnager Jensen.

mod graphics_context;
mod vec2;
mod rgba;
mod rgba_from_str_error;
mod vertex;

pub use graphics_context::GraphicsContext;
pub use rgba::Rgba;
pub use rgba_from_str_error::RgbaFromStrError;
pub use vec2::Vec2;

use vertex::Vertex;

const MAIN_SHADER: &str = concat!(
	include_str!("prelude.wgsl"),
	include_str!("main.vert.wgsl"),
	include_str!("main.frag.wgsl"),
);
