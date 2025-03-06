// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::app::{App, Event};
use crate::graphics::GraphicsContext;
use crate::log::log;

use winit::application::ApplicationHandler;
use winit::event::{StartCause, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

impl ApplicationHandler<Event> for App {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		if self.graphics_context.is_some() { return };

		let graphics_context = GraphicsContext::new(event_loop);
		self.graphics_context = Some(graphics_context);
	}

	fn window_event(
		&mut self,
		event_loop: &ActiveEventLoop,
		_window_id: WindowId,
		event: WindowEvent,
	) {
		match event {
			WindowEvent::CloseRequested => {
				log!("got close request");

				event_loop.exit();
			}

			WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
				self.handle_keyboard_input(event_loop, device_id, event, is_synthetic);
			}

			WindowEvent::MouseWheel { device_id, delta, phase } => {
				self.handle_mouse_wheel(event_loop, device_id, delta, phase);
			}

			WindowEvent::RedrawRequested => {
				let graphics_context = self.graphics_context.as_mut().unwrap();

				graphics_context.render(&self.map, self.view_scale);
			}

			WindowEvent::Resized(size) => {
				let graphics_context = self.graphics_context.as_mut().unwrap();

				graphics_context.resize(size.width, size.height);
			}

			// Ignore by default.

			_ => { }
		}
	}

	fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
		let StartCause::Poll = cause else { return };

		let graphics_context = self.graphics_context.as_mut().unwrap();

		graphics_context.request_redraw();
	}

	fn user_event(&mut self, event_loop: &ActiveEventLoop, event: Event) {
		let Event::Terminate = event;

		log!("got terminate");

		event_loop.exit();
	}

	fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
		log!("goodbye <3");
	}

	fn memory_warning(&mut self, _event_loop: &ActiveEventLoop) {
		log!("got low memory warning");
		log!(note, "we don't know what to do");
	}
}
