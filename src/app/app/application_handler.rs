// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::app::{App, UserEvent};
use crate::log::log;

use std::hint::cold_path;
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::event::{StartCause, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

impl ApplicationHandler<UserEvent> for App {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		self.graphics_context.init_with(event_loop, |context| {
			context.draw_map(&self.map, self.view_pan, self.view_scale);
		});
	}

	fn window_event(
		&mut self,
		event_loop: &ActiveEventLoop,
		_window_id: WindowId,
		event:      WindowEvent,
	) {
		match event {
			WindowEvent::CloseRequested => {
				log!("got close request");

				event_loop.exit();
			}

			WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
				self.handle_keyboard(event_loop, device_id, event, is_synthetic);
			}

			WindowEvent::ModifiersChanged(modifiers) => {
				self.keyboard_modifiers = modifiers;
			}

			WindowEvent::MouseWheel { device_id, delta, phase } => {
				self.handle_mouse_wheel(event_loop, device_id, delta, phase);
			}

			WindowEvent::RedrawRequested => {
				let graphics_context = self.graphics_context.unwrap_mut();
				graphics_context.render_frame(self.level.background);
			}

			WindowEvent::Resized(size) => {
				let graphics_context = self.graphics_context.unwrap_mut();
				graphics_context.resize((size.width, size.height));
			}

			// Ignore by default.

			_ => { }
		}
	}

	fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
		let StartCause::Poll = cause else { return };

		if Instant::now() >= self.next_tick {
			cold_path();

			self.next_tick = Instant::now() + Duration::from_nanos(1_000_000_000 / self.config.tps as u64);

			self.tick();

			let graphics_context = self.graphics_context.unwrap_mut();
			graphics_context.draw_map(&self.map, self.view_pan, self.view_scale);
		}

		let graphics_context = self.graphics_context.unwrap_mut();
		graphics_context.redraw_window();
	}

	fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
		match event {
			UserEvent::Terminate => {
				log!("got terminate");

				event_loop.exit();
			}

			UserEvent::RedrawMap => {
				let graphics_context = self.graphics_context.unwrap_mut();
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
