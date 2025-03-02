// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::app::{App, Event};
use crate::error::Result;

use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};

impl App {
	pub fn run(mut self) -> Result<()> {
		self.print_welcome_message();

		let level = self.load_level("valley")?;
		self.regenerate_level(&level, self.config.map_size);

		eprintln!("creating event loop");

		let event_loop = match EventLoop::with_user_event().build() {
			Ok(event_loop) => event_loop,

			Err(e) => panic!("unable to create event loop: {e}"),
		};

		event_loop.set_control_flow(ControlFlow::Poll);

		Self::set_terminate_handler(event_loop.create_proxy());

		event_loop.run_app(&mut self).unwrap();

		Ok(())
	}

	fn set_terminate_handler(event_loop: EventLoopProxy<Event>) {
		eprintln!("setting terminate handler");

		ctrlc::set_handler(move || {
			event_loop
				.send_event(Event::Terminate)
				.expect("unable to send terminate event");
		}).expect("unable to set terminate handler");
	}
}
