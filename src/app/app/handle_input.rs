// Copyright 2022-2025 Gabriel Bjørnager Jensen.

use crate::app::App;
use crate::graphics::{MAX_VIEW_SCALE, MIN_VIEW_SCALE};
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

		let view_scale = f64::from(self.view_scale) * 2.0f64.powf((-y).into());

		log!(note, "new view scale is unclamped at `{view_scale}`");

		let view_scale = view_scale.clamp(MIN_VIEW_SCALE.into(), MAX_VIEW_SCALE.into()) as u32;

		log!(note, "new view scale is clamped to `{view_scale}`");

		self.view_scale = view_scale;
	}
}
