// Copyright 2025 Gabriel Bjørnager Jensen.

mod draw_map;
mod new;
mod render_frame;
mod resize;

use polywave::www::Html;
use std::borrow::Cow;
use std::pin::Pin;
use winit::window::Window;

#[derive(Debug)]
pub struct InitGraphicsContext {
	pipeline: wgpu::RenderPipeline,

	vertex_count: u32,
	vertex_buf:   wgpu::Buffer,

	texture_buf:        Box<[Html]>,
	texture_bind_group: wgpu::BindGroup,
	texture:            wgpu::Texture,

	queue:  wgpu::Queue,
	device: wgpu::Device,

	surface_config: wgpu::SurfaceConfiguration,
	surface:        wgpu::Surface<'static>,

	window: Pin<Box<Window>>,
}

impl InitGraphicsContext {
	const DEFAULT_SIZE: (u32, u32) = (0x280, 0x1E0);

	const TEXTURE_WIDTH: u32 = {
		if Self::DEFAULT_SIZE.0 >= Self::DEFAULT_SIZE.1 {
			Self::DEFAULT_SIZE.1
		} else {
			Self::DEFAULT_SIZE.0
		}
	};

	const TEXTURE_EXTENT: wgpu::Extent3d = wgpu::Extent3d {
		width:                 Self::TEXTURE_WIDTH,
		height:                Self::TEXTURE_WIDTH,
		depth_or_array_layers: 0x1,
	};

	const MAIN_SHADER: wgpu::ShaderSource<'_> = get_main_shader();

	#[inline]
	pub fn redraw_window(&mut self) {
		self.window.request_redraw();
	}

	#[expect(unused)]
	#[inline(always)]
	#[must_use]
	pub const fn size(&self) -> (u32, u32) {
		let wgpu::SurfaceConfiguration { width, height, .. } = self.surface_config;
		(width, height)
	}
}

const fn get_main_shader() -> wgpu::ShaderSource<'static> {
	let contents = concat!(
		include_str!("../prelude.wgsl"),
		include_str!("../main.vert.wgsl"),
		include_str!("../main.frag.wgsl"),
	);

	let contents =  Cow::Borrowed(contents);
	wgpu::ShaderSource::Wgsl(contents)
}
