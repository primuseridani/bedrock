// Copyright 2022-2025 Gabriel Bjørnager Jensen.

use crate::app::App;
use crate::log::log;

use winit::event::{DeviceId, ElementState, KeyEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};

impl App {
	pub(super) fn handle_keyboard(
		&mut self,
		event_loop:    &ActiveEventLoop,
		_device_id:    DeviceId,
		event:         KeyEvent,
		_is_synthetic: bool,
	) {
		if event.repeat {
			return;
		}

		if matches!(event.state, ElementState::Released) {
			return;
		}

		match event.physical_key {
			PhysicalKey::Code(
				| KeyCode::Minus
				| KeyCode::Equal
			) => {
				let off: i16 = match event.physical_key {
					PhysicalKey::Code(KeyCode::Minus) => -0x1,
					PhysicalKey::Code(KeyCode::Equal) =>  0x1,

					_ => unreachable!(),
				};

				let tps = self
					.preset
					.tps
					.saturating_add_signed(off)
					.max(0x1);

				log!(note, "new tps is clamped at `{tps}`");

				self.preset.tps = tps;
			}

			PhysicalKey::Code(KeyCode::Escape) => {
				self.is_paused = !self.is_paused;

				if self.is_paused {
					log!("game has been paused");
				} else {
					log!("game has been unpaused");
				}
			}

			PhysicalKey::Code(KeyCode::KeyQ)
			if self.is_paused
			=> {
				event_loop.exit();
			}

			_ => { }
		}
	}
}
