// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::graphics::{GraphicsContext, Vertex, MAIN_SHADER};

use crate::log::log;
use crate::version::Version;

use pollster::block_on;
use std::borrow::Cow;
use std::pin::Pin;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowAttributes;
use zerocopy::IntoBytes;

impl GraphicsContext {
	#[must_use]
	pub fn new(event_loop: &ActiveEventLoop) -> Self {
		log!(debug, "creating graphics context");

		let size = PhysicalSize {
			width:  Self::DEFAULT_SIZE.0,
			height: Self::DEFAULT_SIZE.1,
		};

		log!(debug, "opening window");

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

		log!(debug, "creating wgpu instance");

		let instance = wgpu::Instance::new(&Default::default());

		log!(debug, "creating surface");

		let surface = unsafe {
			let target = match wgpu::SurfaceTargetUnsafe::from_window(&*window) {
				Ok(target) => target,

				Err(e) => panic!("unable to create surface target: {e}"),
			};

			match instance.create_surface_unsafe(target) {
				Ok(surface) => surface,

				Err(e) => panic!("unable to create surface: {e}"),
			}
		};

		log!(debug, "creating adapter");

		let adapter = {
			let options = wgpu::RequestAdapterOptions {
				power_preference:   wgpu::PowerPreference::LowPower,
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

		log!(debug, "creating device and queue");

		let (device, queue) = {
			match block_on(adapter.request_device(&Default::default(), None)) {
				Ok((device, queue)) => (device, queue),

				Err(e) => panic!("unable to find device: {e}"),
			}
		};

		log!(debug, "configuring surface");

		let surface_config = wgpu::SurfaceConfiguration {
			usage:                         wgpu::TextureUsages::RENDER_ATTACHMENT,
			format:                        surface_format,
			width:                         size.width,
			height:                        size.height,
			present_mode:                  wgpu::PresentMode::Fifo,
			desired_maximum_frame_latency: 0x2,
			alpha_mode:                    surface_capabilities.alpha_modes[0x0],
			view_formats:                  Default::default(),
		};

		surface.configure(&device, &surface_config);

		log!(debug, "creating shader module");

		let shader = {
			let descriptor = wgpu::ShaderModuleDescriptor {
				label:  Some("main"),
				source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(MAIN_SHADER)),
			};

			device.create_shader_module(descriptor)
		};

		log!(debug, "creating texture");

		let texture = {
			let descriptor = wgpu::TextureDescriptor {
				label:           Some("main"),
				size:            Self::TEXTURE_EXTENT,
				mip_level_count: 0x1,
				sample_count:    0x1,
				dimension:       wgpu::TextureDimension::D2,
				format:          wgpu::TextureFormat::Rgba8UnormSrgb,
				usage:           wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
				view_formats:    Default::default(),
			};

			device.create_texture(&descriptor)
		};

		let texture_view = {
			let descriptor = wgpu::TextureViewDescriptor {
				label: Some("main"),

				..Default::default()
			};

			texture.create_view(&descriptor)
		};

		let texture_sampler = {
			let descriptor = wgpu::SamplerDescriptor {
				label:        Some("main"),
				border_color: Some(wgpu::SamplerBorderColor::OpaqueBlack),

				..Default::default()
			};

			device.create_sampler(&descriptor)
		};

		let texture_bind_group_layout = {
			let descriptor = wgpu::BindGroupLayoutDescriptor {
				label: Some("main"),

				entries: &[
					wgpu::BindGroupLayoutEntry {
						binding:    0x0,
						visibility:	wgpu::ShaderStages::FRAGMENT,

						ty: wgpu::BindingType::Texture {
							sample_type: wgpu::TextureSampleType::Float {
								filterable: false,
							},

							view_dimension: wgpu::TextureViewDimension::D2,
							multisampled:   false,
						},

						count: None,
					},

					wgpu::BindGroupLayoutEntry {
						binding:    0x1,
						visibility:	wgpu::ShaderStages::FRAGMENT,
						ty:         wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
						count:      None,
					},
				],
			};

			device.create_bind_group_layout(&descriptor)
		};

		let texture_bind_group = {
			let descriptor = wgpu::BindGroupDescriptor {
				label:  Some("main"),
				layout: &texture_bind_group_layout,

				entries: &[
					wgpu::BindGroupEntry {
						binding:  0x0,
						resource: wgpu::BindingResource::TextureView(&texture_view),
					},

					wgpu::BindGroupEntry {
						binding:  0x1,
						resource: wgpu::BindingResource::Sampler(&texture_sampler),
					},
				],
			};

			device.create_bind_group(&descriptor)
		};

		let texture_buf = vec![Default::default(); Self::TEXTURE_WIDTH as usize * Self::TEXTURE_WIDTH as usize].into();

		log!(debug, "creating vertex buffer");

		let vertex_buf = {
			let vertices = [Vertex::default(); 0x8];

			let descriptor = BufferInitDescriptor {
				label:    Some("main"),
				contents: vertices.as_bytes(),
				usage:    wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::VERTEX,
			};

			device.create_buffer_init(&descriptor)
		};

		log!(debug, "creating index buffer");

		let (index_count, index_buf) = {
			let indices: [u16; _] = [
				0x0, 0x1, 0x2,
				0x3, 0x0, 0x2,
			];

			let descriptor = BufferInitDescriptor {
				label:    Some("main"),
				contents: indices.as_bytes(),
				usage:    wgpu::BufferUsages::INDEX,
			};

			let count = indices.len() as u32;
			let buf   = device.create_buffer_init(&descriptor);

			(count, buf)
		};

		log!(debug, "creating render pipeline");

		let pipeline = {
			let descriptor = wgpu::PipelineLayoutDescriptor {
				label:              Some("main"),
				bind_group_layouts: &[&texture_bind_group_layout],

				..Default::default()
			};

			let layout = device.create_pipeline_layout(&descriptor);

			let vertex = wgpu::VertexState {
				module:              &shader,
				entry_point:         Some("vertex_main"),
				buffers:             &[Vertex::LAYOUT],
				compilation_options: Default::default(),
			};

			let fragment = wgpu::FragmentState {
				module:              &shader,
				entry_point:         Some("fragment_main"),

				targets: &[
					Some(wgpu::ColorTargetState {
						format:     surface_config.format,
						blend:      Some(wgpu::BlendState::ALPHA_BLENDING),
						write_mask: wgpu::ColorWrites::ALL,
					})
				],

				compilation_options: Default::default(),
			};

			let primitive = wgpu::PrimitiveState {
				topology:     wgpu::PrimitiveTopology::TriangleList,
				front_face:   wgpu::FrontFace::Ccw,
				cull_mode:    Some(wgpu::Face::Back),
				polygon_mode: wgpu::PolygonMode::Fill,

				..Default::default()
			};

			let descriptor = wgpu::RenderPipelineDescriptor {
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

		let mut this = Self {
			pipeline,

			index_count,
			index_buf,
			vertex_buf,

			texture_buf,
			texture_bind_group,
			texture,

			queue,
			device,

			surface_config,
			surface,

			window,
		};

		this.resize(size.width, size.height);

		this
	}
}
