// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::app::{App, UserEvent};
use crate::error::Result;
use crate::log::log;

use std::env::args;

impl App {
	pub(super) fn init(&mut self) -> Result<()> {
		self.set_terminate_handler()?;

		if let Some(level) = args().nth(0x1) {
			let level = self.load_level(&level)?;
			self.level = level;
		}

		self.players.clear();

		self.players.push(Default::default());

		self.regenerate_level();

		self.respawn_players()?;

		Ok(())
	}

	fn set_terminate_handler(&self) -> Result<()> {
		log!(debug, "setting terminate handler");

		let event_loop = self.event_loop_proxy.clone();

		ctrlc::set_handler(move || {
			event_loop
				.send_event(UserEvent::Terminate)
				.expect("unable to send terminate event");
		}).expect("unable to set terminate handler");

		// TODO
		Ok(())
	}
}
