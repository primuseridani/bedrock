// Copyright 2025 Gabriel Bjørnager Jensen.

mod graphics_context;
mod vec4;
mod vertex;

use vec4::Vec4;
use vertex::Vertex;

pub use graphics_context::GraphicsContext;

use wgpu::{include_wgsl, ShaderModuleDescriptor};

const MAIN_SHADER: ShaderModuleDescriptor = include_wgsl!("main.wgsl");
