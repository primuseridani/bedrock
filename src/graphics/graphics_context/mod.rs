// Copyright 2025 Gabriel Bjørnager Jensen.

mod render;

use crate::graphics::{MAIN_SHADER, Vec4, Vertex};
use crate::version::Version;

use pollster::block_on;
use std::pin::Pin;
use wgpu::{
	BlendState,
	Buffer,
	BufferUsages,
	ColorTargetState,
	ColorWrites,
	Device,
	Face,
	FragmentState,
	FrontFace,
	Instance,
	PolygonMode,
	PowerPreference,
	PresentMode,
	PrimitiveState,
	PrimitiveTopology,
	Queue,
	RenderPipeline,
	RenderPipelineDescriptor,
	RequestAdapterOptions,
	Surface,
	SurfaceConfiguration,
	SurfaceTargetUnsafe,
	TextureUsages,
	VertexState,
};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};
use zerocopy::IntoBytes;

#[derive(Debug)]
pub struct GraphicsContext {
	surface_config: SurfaceConfiguration,

	vertex_count: u32,
	vertex_buf:   Buffer,

	pipeline: RenderPipeline,

	queue:  Queue,
	device: Device,

	surface: Surface<'static>,

	window: Pin<Box<Window>>,
}

impl GraphicsContext {
	#[must_use]
	pub fn new(event_loop: &ActiveEventLoop) -> Self {
		eprintln!("creating graphics context");

		let size = PhysicalSize {
			width:  0x200,
			height: 0x180,
		};

		eprintln!("opening window");

		let window = {
			let title = format!("Bedrock {}", Version::CURRENT);

			let attrs = WindowAttributes::default()
				.with_inner_size(size)
				.with_min_inner_size(size)
				.with_title(&title);

			match event_loop.create_window(attrs) {
				Ok(window) => Pin::new(Box::new(window)),

				Err(e) => panic!("unable to open window: {e}"),
			}
		};

		eprintln!("creating wgpu instance");

		let instance = Instance::new(&Default::default());

		eprintln!("creating surface");

		let surface = unsafe {
			let target = match SurfaceTargetUnsafe::from_window(&*window) {
				Ok(target) => target,

				Err(e) => panic!("unable to create surface target: {e}"),
			};

			match instance.create_surface_unsafe(target) {
				Ok(surface) => surface,

				Err(e) => panic!("unable to create surface: {e}"),
			}
		};

		eprintln!("creating adapter");

		let adapter = {
			let options = RequestAdapterOptions {
				power_preference:   PowerPreference::LowPower,
				compatible_surface: Some(&surface),

				..Default::default()
			};

			block_on(instance.request_adapter(&options))
				.expect("no adapter available")
		};

		let surface_capabilities = surface.get_capabilities(&adapter);

		let surface_format = surface_capabilities
			.formats
			.iter()
			.find(|f| f.is_srgb())
			.copied()
			.expect("unable to find srgb surface format");

		eprintln!("creating device and queue");

		let (device, queue) = {
			match block_on(adapter.request_device(&Default::default(), None)) {
				Ok((device, queue)) => (device, queue),

				Err(e) => panic!("unable to find device: {e}"),
			}
		};

		eprintln!("configuring surface");

		let surface_config = SurfaceConfiguration {
			usage:                         TextureUsages::RENDER_ATTACHMENT,
			format:                        surface_format,
			width:                         size.width,
			height:                        size.height,
			present_mode:                  PresentMode::Fifo,
			desired_maximum_frame_latency: 0x2,
			alpha_mode:                    surface_capabilities.alpha_modes[0x0],
			view_formats:                  Default::default(),
		};

		surface.configure(&device, &surface_config);

		eprintln!("creating shader module");

		let shader = device.create_shader_module(MAIN_SHADER);

		eprintln!("creating render pipeline");

		let pipeline = {
			let layout = device.create_pipeline_layout(&Default::default());

			let vertex = VertexState {
				module:              &shader,
				entry_point:         Some("vertex_main"),
				buffers:             &[Vertex::LAYOUT],
				compilation_options: Default::default(),
			};

			let fragment = FragmentState {
				module:              &shader,
				entry_point:         Some("fragment_main"),

				targets: &[
					Some(ColorTargetState {
						format:     surface_config.format,
						blend:      Some(BlendState::ALPHA_BLENDING),
						write_mask: ColorWrites::ALL,
					})
				],

				compilation_options: Default::default(),
			};

			let primitive = PrimitiveState {
				topology:     PrimitiveTopology::TriangleList,
				front_face:   FrontFace::Ccw,
				cull_mode:    Some(Face::Back),
				polygon_mode: PolygonMode::Fill,

				..Default::default()
			};

			let descriptor = RenderPipelineDescriptor {
				label:         Some("main"),
				layout:        Some(&layout),
				vertex,
				fragment:      Some(fragment),
				primitive,
				depth_stencil: Default::default(),
				multisample:   Default::default(),
				multiview:     Default::default(),
				cache:         Default::default(),
			};

			device.create_render_pipeline(&descriptor)
		};

		eprintln!("creating vertex buffer");

		let (vertex_count, vertex_buf) = {
			let vertices = [
				Vertex {
					position: Vec4::new( 0.0,  1.0,  1.0, 1.0),
					colour:   Vec4::new( 1.0,  0.0,  0.0, 1.0),
				},

				Vertex {
					position: Vec4::new(-0.5,  0.0,  0.0, 1.0),
					colour:   Vec4::new( 0.0,  1.0,  0.0, 1.0),
				},

				Vertex {
					position: Vec4::new( 0.5,  0.0,  0.0, 1.0),
					colour:   Vec4::new( 0.0,  0.0,  1.0, 1.0),
				},

				Vertex {
					position: Vec4::new(-0.5,  0.0,  0.0, 1.0),
					colour:   Vec4::new( 0.0,  1.0,  1.0, 1.0),
				},

				Vertex {
					position: Vec4::new(-1.0, -1.0,  0.0, 1.0),
					colour:   Vec4::new( 1.0,  0.0,  1.0, 1.0),
				},

				Vertex {
					position: Vec4::new( 0.0, -1.0,  0.0, 1.0),
					colour:   Vec4::new( 1.0,  1.0,  0.0, 1.0),
				},

				Vertex {
					position: Vec4::new( 0.5,  0.0,  0.0, 1.0),
					colour:   Vec4::new( 1.0,  1.0,  1.0, 1.0),
				},

				Vertex {
					position: Vec4::new( 0.0, -1.0,  0.0, 1.0),
					colour:   Vec4::new( 0.0,  0.0,  0.0, 1.0),
				},

				Vertex {
					position: Vec4::new( 1.0, -1.0,  0.0, 1.0),
					colour:   Vec4::new( 0.0,  0.0,  0.0, 1.0),
				},
			];

			let descriptor = BufferInitDescriptor {
				label:    Some("main"),
				contents: vertices.as_bytes(),
				usage:    BufferUsages::VERTEX,
			};

			let count = vertices.len() as u32;
			let buf   = device.create_buffer_init(&descriptor);

			(count, buf)
		};

		Self {
			surface_config,

			vertex_count,
			vertex_buf,

			pipeline,

			queue,
			device,

			surface,

			window,
		}
	}

	#[inline]
	pub fn resize(&mut self, width: u32, height: u32) {
		self.surface_config.width  = width;
		self.surface_config.height = height;

		eprintln!("resizing graphics context to `{width}*{height}`");

		self.surface.configure(&self.device, &self.surface_config);
	}

	#[inline(always)]
	pub fn request_redraw(&mut self) {
		self.window.request_redraw();
	}
}
