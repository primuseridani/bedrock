// Copyright 2025 Gabriel Bjørnager Jensen.

mod application_handler;
mod handle_input;
mod load_level;
mod regenerate_level;
mod run;
mod tick;

use crate::app::UserEvent;
use crate::graphics::GraphicsContext;
use crate::map::Map;
use crate::preset::Preset;

use std::path::PathBuf;
use winit::event::Modifiers;
use winit::event_loop::EventLoopProxy;

#[derive(Debug)]
pub struct App {
	data_dir: PathBuf,

	config: Preset,

	map: Map,

	event_loop_proxy: EventLoopProxy<UserEvent>,

	graphics_context: Option<GraphicsContext>,

	keyboard_modifiers: Modifiers,

	view_pan:   (u32, u32),
	view_scale: u32,
}

impl App {
	pub const MIN_VIEW_SCALE: u32 = 0x0040;
	pub const MAX_VIEW_SCALE: u32 = 0x1000;

	#[inline]
	#[track_caller]
	fn create_event(&self, event: UserEvent) {
		if let Err(e) = self.event_loop_proxy.send_event(event) {
			panic!("unable to create event: {e}");
		}
	}
}
