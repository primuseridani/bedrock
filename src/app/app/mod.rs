// Copyright 2025 Gabriel Bjørnager Jensen.

mod application_handler;
mod handle_keyboard;
mod handle_mouse_wheel;
mod load_level;
mod regenerate_level;
mod run;
mod tick;

use crate::app::UserEvent;
use crate::config::Config;
use crate::graphics::GraphicsContext;
use crate::level::Level;
use crate::map::Map;

use std::path::PathBuf;
use std::time::Instant;
use winit::event::Modifiers;
use winit::event_loop::EventLoopProxy;

#[derive(Debug)]
pub struct App {
	event_loop_proxy: EventLoopProxy<UserEvent>,

	graphics_context: GraphicsContext,

	keyboard_modifiers: Modifiers,

	data_dir: PathBuf,
	config:   Config,
	level:    Level,

	map: Map,

	raw_view_scale: f64,

	view_pan:   (u32, u32),
	view_scale: u32,

	next_tick: Instant,

	is_paused: bool,
}

impl App {
	pub const MIN_VIEW_SCALE: u32 = 0x0040;
	pub const MAX_VIEW_SCALE: u32 = 0x1000;

	#[inline]
	#[track_caller]
	fn create_user_event(&self, event: UserEvent) {
		if let Err(e) = self.event_loop_proxy.send_event(event) {
			panic!("unable to create event: {e}");
		}
	}
}
