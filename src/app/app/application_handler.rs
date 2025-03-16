// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::app::{App, UserEvent};
use crate::graphics::GraphicsContext;
use crate::log::log;

use winit::application::ApplicationHandler;
use winit::event::{StartCause, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

impl ApplicationHandler<UserEvent> for App {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		let _ = self.graphics_context.get_or_insert_with(|| {
			let mut context = GraphicsContext::new(event_loop);

			context.draw_map(&self.map, self.view_pan, self.view_scale);

			context
		});
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

			WindowEvent::ModifiersChanged(modifiers) => {
				log!(note, "keyboard modifiers changed: `{modifiers:?}`");
				self.keyboard_modifiers = modifiers;
			}

			WindowEvent::MouseWheel { device_id, delta, phase } => {
				self.handle_mouse_wheel(event_loop, device_id, delta, phase);
			}

			WindowEvent::RedrawRequested => {
				let graphics_context = self.graphics_context.as_mut().unwrap();
				graphics_context.render(self.config.level.background);
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

	fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
		match event {
			UserEvent::Terminate => {
				log!("got terminate");

				event_loop.exit();
			}

			UserEvent::RedrawMap => {
				let graphics_context = self.graphics_context.as_mut().unwrap();
				graphics_context.draw_map(&self.map, self.view_pan, self.view_scale);
			}
		}
	}

	fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
		log!("goodbye <3");
	}

	fn memory_warning(&mut self, _event_loop: &ActiveEventLoop) {
		log!("got low memory warning");
		log!(note, "we don't know what to do");
	}
}
