// Copyright 2025 Gabriel Bjørnager Jensen.

mod render;

use crate::version::Version;

use std::pin::Pin;
use wgpu::{
	Device,
	Instance,
	PresentMode,
	PowerPreference,
	Queue,
	RequestAdapterOptions,
	Surface,
	SurfaceConfiguration,
	SurfaceTargetUnsafe,
	TextureUsages,
};
use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

#[derive(Debug)]
pub struct GraphicsContext {
	surface_config: SurfaceConfiguration,

	queue:  Queue,
	device: Device,

	surface: Surface<'static>,

	window: Pin<Box<Window>>,
}

impl GraphicsContext {
	#[must_use]
	pub async fn new(event_loop: &ActiveEventLoop) -> Self {
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

			instance
				.request_adapter(&options)
				.await
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
			match adapter.request_device(&Default::default(), None).await {
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

		Self {
			surface_config,

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
