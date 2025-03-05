// Copyright 2025 Gabriel Bjørnager Jensen.

mod new;
mod render;
mod resize;

use crate::graphics::{MAX_VIEW_SCALE, Rgba};

use std::pin::Pin;
use winit::window::Window;

#[derive(Debug)]
pub struct GraphicsContext {
	pipeline: wgpu::RenderPipeline,

	index_count: u32,
	index_buf:   wgpu::Buffer,
	vertex_buf:  wgpu::Buffer,

	texture_buf:        Box<[Rgba]>,
	texture_bind_group: wgpu::BindGroup,
	texture:            wgpu::Texture,

	queue:  wgpu::Queue,
	device: wgpu::Device,

	surface_config: wgpu::SurfaceConfiguration,
	surface:        wgpu::Surface<'static>,

	window: Pin<Box<Window>>,
}

impl GraphicsContext {
	const DEFAULT_SIZE: (u32, u32) = (0x0200, 0x0180);

	const TEXTURE_EXTENT: wgpu::Extent3d = wgpu::Extent3d {
		width:                 MAX_VIEW_SCALE,
		height:                MAX_VIEW_SCALE,
		depth_or_array_layers: 0x1,
	};

	#[inline(always)]
	pub fn request_redraw(&mut self) {
		self.window.request_redraw();
	}
}
