// Copyright 2025 Gabriel Bjørnager Jensen.

mod graphics_context;
mod rgba;
mod vec2;
mod vertex;

pub use graphics_context::GraphicsContext;

use rgba::Rgba;
use vec2::Vec2;
use vertex::Vertex;

pub const MAX_VIEW_SCALE: u32 = 0x100;

const MAIN_SHADER: &str = concat!(
	include_str!("prelude.wgsl"),
	include_str!("main.vert.wgsl"),
	include_str!("main.frag.wgsl"),
);
