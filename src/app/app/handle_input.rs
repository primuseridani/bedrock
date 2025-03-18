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

		match event.physical_key {
			| PhysicalKey::Code(KeyCode::Minus)
			| PhysicalKey::Code(KeyCode::Equal)
			=> {
				let off: i16 = match event.physical_key {
					PhysicalKey::Code(KeyCode::Minus) => -0x1,
					PhysicalKey::Code(KeyCode::Equal) =>  0x1,

					_ => unreachable!(),
				};

				let tps = self.config.tps.saturating_add_signed(off).max(0x1);

				log!(note, "new tps is clamped at `{tps}`");

				self.config.tps = tps;
			}

			PhysicalKey::Code(KeyCode::Escape) => {
				event_loop.exit();
			}

			_ => { }
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

			self.create_user_event(UserEvent::RedrawMap);
		} else if modifiers_state.shift_key() {
			// Pan up or down.

			let pan = get_raw_pan(self.view_pan.1, y);

			log!(note, "new y pan is unclamped at `{pan}`");

			let pan = pan.clamp(0x0, self.config.map_size.height() - 0x1);

			log!(note, "new y pan is clamped to `{pan}`");

			self.view_pan.1 = pan;

			self.create_user_event(UserEvent::RedrawMap);
		} else if modifiers_state.control_key() {
			// Zoom in out out.

			let raw_view_scale = self.raw_view_scale - f64::from(y) / 2.0;

			log!(note, "new raw view scale is unclamped at `{raw_view_scale}`");

			// Clamp the raw view scale such that the derived
			// ("actual") view scale is in the allowed range:
			//
			// let:
			//   v     : view scale
			//   v_min : minimum allowed view scale
			//   v_max : maximum allowed view scale
			//   f     : view scale factor (from mouse wheel)
			//
			// for:
			//   v             ≡ v_min * 2^(f / 2)
			//   v / v_min     = 2^(f / 2)
			//   lb(z / v_min) = f / 2

			// lb(v_min / v_min) = lb(1) = 0, v_min in real and v_min > 0
			let min_raw_view_scale = 0.0;

			let max_raw_view_scale = {
				// Bound the view scale such that it does not ex-
				// tend beyond the largest dimension of the map.

				let (width, height) = self.map.size().get();

				let max_view_scale = width.max(height).min(Self::MAX_VIEW_SCALE);

				(f64::from(max_view_scale) / f64::from(Self::MIN_VIEW_SCALE)).log2()
			};

			// The view scale should not be updated if bounded
			// -- as to guarantee gradient increments.

			if raw_view_scale >= min_raw_view_scale && raw_view_scale <= max_raw_view_scale {
				let view_scale = (f64::from(Self::MIN_VIEW_SCALE) * 2.0f64.powf(raw_view_scale)) as u32;

				log!(note, "new view scale is `{view_scale}`");

				self.raw_view_scale = raw_view_scale;
				self.view_scale     = view_scale;

				self.create_user_event(UserEvent::RedrawMap);
			} else {
				log!(note, "view scale remains unchanged");
			}
		}
	}
}
