// Copyright 2025 Gabriel Bjørnager Jensen.

mod graphics_context;
mod init_graphics_context;
mod vec2;
mod vertex;

pub use graphics_context::GraphicsContext;
pub use init_graphics_context::InitGraphicsContext;
pub use vec2::Vec2;

use vertex::Vertex;

const MAIN_SHADER: &str = concat!(
	include_str!("prelude.wgsl"),
	include_str!("main.vert.wgsl"),
	include_str!("main.frag.wgsl"),
);
