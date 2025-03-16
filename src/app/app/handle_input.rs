// Copyright 2022-2025 Gabriel Bjørnager Jensen.

use crate::app::{App, UserEvent};
use crate::log::log;

use winit::event::{
	DeviceId,
	ElementState,
	KeyEvent,
	MouseScrollDelta,
	TouchPhase,
};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};

impl App {
	#[inline]
	pub(super) fn handle_keyboard_input(
		&mut self,
		event_loop:    &ActiveEventLoop,
		_device_id:    DeviceId,
		event:         KeyEvent,
		_is_synthetic: bool,
	) {
		if event.repeat { return };

		if matches!(event.state, ElementState::Released) { return };

		if matches!(event.physical_key, PhysicalKey::Code(KeyCode::Escape)) {
			event_loop.exit();
		}
	}

	#[inline]
	pub(super) fn handle_mouse_wheel(
		&mut self,
		_event_loop: &ActiveEventLoop,
		_device_id:  DeviceId,
		delta:       MouseScrollDelta,
		_phase:      TouchPhase,
	) {
		let MouseScrollDelta::LineDelta(_, y) = delta else { return };

		let modifiers_state = self.keyboard_modifiers.state();

		let get_raw_pan = |base: u32, raw_factor: f32| -> u32 {
			let pan_factor = (raw_factor.ceil() as i32).clamp(-0x1, 0x1);

			let pan_distance = ((self.view_scale / 0x10) as i32) * pan_factor;

			base.saturating_add_signed(pan_distance)
		};

		if modifiers_state.alt_key() {
			// Pan to the left or the right.

			let pan = get_raw_pan(self.view_pan.0, y);

			log!(note, "new x pan is unclamped at `{pan}`");

			let pan = pan.clamp(0x0, self.config.map_size.width() - 0x1);

			log!(note, "new x pan is clamped to `{pan}`");

			self.view_pan.0 = pan;

			self.create_event(UserEvent::RedrawMap);
		} else if modifiers_state.shift_key() {
			// Pan up or down.

			let pan = get_raw_pan(self.view_pan.1, y);

			log!(note, "new y pan is unclamped at `{pan}`");

			let pan = pan.clamp(0x0, self.config.map_size.height() - 0x1);

			log!(note, "new y pan is clamped to `{pan}`");

			self.view_pan.1 = pan;

			self.create_event(UserEvent::RedrawMap);
		} else if modifiers_state.control_key() {
			// Zoom in out out.

			let view_scale = f64::from(self.view_scale) * 2.0f64.powf((-y).into());

			log!(note, "new view scale is unclamped at `{view_scale}`");

			let max_view_scale = {
				let (width, height) = self.map.size().get();

				width.max(height).min(Self::MAX_VIEW_SCALE)
			};

			let view_scale = view_scale.clamp(Self::MIN_VIEW_SCALE.into(), max_view_scale.into()) as u32;

			log!(note, "new view scale is clamped to `{view_scale}`");

			self.view_scale = view_scale;

			self.create_event(UserEvent::RedrawMap);
		}
	}
}
