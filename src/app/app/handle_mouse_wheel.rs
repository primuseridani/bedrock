// Copyright 2022-2025 Gabriel Bjørnager Jensen.

use crate::app::{App, UserEvent};
use crate::log::log;

use winit::event::{DeviceId, MouseScrollDelta, TouchPhase};
use winit::event_loop::ActiveEventLoop;

impl App {
	pub(super) fn handle_mouse_wheel(
		&mut self,
		_event_loop: &ActiveEventLoop,
		_device_id:  DeviceId,
		delta:       MouseScrollDelta,
		_phase:      TouchPhase,
	) {
		let MouseScrollDelta::LineDelta(x, y) = delta else { return };

		let modifiers_state = self.keyboard_modifiers.state();

		if x.abs() > 0.0 {
			self.handle_vertical_pan(x);

			return;
		}

		if modifiers_state.shift_key() {
			self.handle_vertical_pan(y);
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

			// The raw view scale should not be updated if
			// bounded -- as to guarantee gradient increments.

			if raw_view_scale >= min_raw_view_scale && raw_view_scale <= max_raw_view_scale {
				let view_scale = (f64::from(Self::MIN_VIEW_SCALE) * raw_view_scale.exp2()) as u32;

				log!(note, "new view scale is `{view_scale}`");

				self.raw_view_scale = raw_view_scale;
				self.view_scale     = view_scale;

				self.create_user_event(UserEvent::RedrawMap);
			} else {
				log!(note, "view scale remains unchanged");
			}
		} else if modifiers_state.is_empty() {
			// Pan to the left or to the right.

			let pan = get_raw_pan(self.view_pan.0, y, self.view_scale);

			log!(note, "new x pan is unclamped at `{pan}`");

			let pan = pan.clamp(0x0, self.config.map_size.width() - 0x1);

			log!(note, "new x pan is clamped to `{pan}`");

			self.view_pan.0 = pan;

			self.create_user_event(UserEvent::RedrawMap);
		}
	}

	#[inline]
	#[track_caller]
	fn handle_vertical_pan(&mut self, distance: f32) {
		// Pan up or down.

		let pan = get_raw_pan(self.view_pan.1, distance, self.view_scale);

		log!(note, "new y pan is unclamped at `{pan}`");

		let pan = pan.clamp(0x0, self.config.map_size.height() - 0x1);

		log!(note, "new y pan is clamped to `{pan}`");

		self.view_pan.1 = pan;

		self.create_user_event(UserEvent::RedrawMap);
	}
}

#[inline]
#[must_use]
fn get_raw_pan(base: u32, raw_factor: f32, view_scale: u32) -> u32 {
	let pan_factor = (raw_factor.ceil() as i32).clamp(-0x1, 0x1);

	#[expect(clippy::cast_possible_wrap)]
	let pan_distance = ((view_scale / 0x10) as i32) * pan_factor;

	base.saturating_add_signed(pan_distance)
}
