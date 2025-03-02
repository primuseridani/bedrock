// Copyright 2022-2025 Gabriel Bjørnager Jensen.

use crate::app::App;

use winit::event::{ElementState, KeyEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};

impl App {
	#[inline]
	pub(super) fn handle_key_event(&mut self, event_loop: &ActiveEventLoop, event: KeyEvent) {
		if event.repeat { return };

		if matches!(event.state, ElementState::Released) { return };

		if matches!(event.physical_key, PhysicalKey::Code(KeyCode::Escape)) {
			event_loop.exit();
		}
	}
}
