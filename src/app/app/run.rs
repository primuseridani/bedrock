// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::app::{App, UserEvent};
use crate::error::{Error, Result};
use crate::log::log;
use crate::version::Version;

use std::fs::{create_dir_all, write};
use std::path::PathBuf;
use std::time::Instant;
use winit::event_loop::{ControlFlow, EventLoop};

use std::env::home_dir;

impl App {
	pub fn run() -> Result<()> {
		eprintln!();
		eprintln!("\u{001B}[001mYOU HAVE NOW HIT BEDROCK!\u{001B}[022m");
		eprintln!("\u{001B}[002mbedrock-{}\u{001B}[022m", Version::CURRENT);
		eprintln!();
		eprintln!("\u{001B}[002mCopyright \u{00A9} 2025 Gabriel Bj\u{00F8}rnager Jensen.\u{001B}[022m");
		eprintln!();

		log!(debug, "creating event loop");

		let event_loop = match EventLoop::with_user_event().build() {
			Ok(event_loop) => event_loop,

			Err(e) => panic!("unable to create event loop: {e}"),
		};

		event_loop.set_control_flow(ControlFlow::Poll);

		let event_loop_proxy = event_loop.create_proxy();

		let mut this = Self {
			event_loop_proxy,

			graphics_context: Default::default(),

			keyboard_modifiers: Default::default(),

			data_dir: Self::get_data_dir()?,
			config:   Default::default(),
			level:    Default::default(),

			map: Default::default(),

			raw_view_scale: Default::default(),

			view_pan:   Default::default(),
			view_scale: Self::MIN_VIEW_SCALE,

			next_tick: Instant::now(),
		};

		this.init()?;

		event_loop.run_app(&mut this).unwrap();

		Ok(())
	}

	fn get_data_dir() -> Result<PathBuf> {
		let mut data_dir = home_dir().ok_or(Error::MissingDataDir)?;

		#[cfg(target_family = "unix")]
		let data_dir = {
			data_dir.push(".local");
			data_dir.push("share");
			data_dir.push("bedrock");

			data_dir
		};

		#[cfg(target_family = "windows")]
		let data_dir = {
			data_dir.push("AppData");
			data_dir.push("Roaming");
			data_dir.push("Bedrock");

			data_dir
		};

		log!("creating data directory at \"{}\"", data_dir.display());

		create_dir_all(&data_dir)
			.map_err(|_| Error::MissingDataDir)?;

		let subdirs = [
			"level",
		];

		for subdir in subdirs {
			let subdir = data_dir.join(subdir);

			log!(debug, "creating subdirectory at \"{}\"", subdir.display());

			create_dir_all(&subdir)
				.map_err(|_| Error::MissingDataDir)?;
		}

		let test_level_path = {
			let mut path = data_dir.to_owned();

			path.push("level");
			path.push("test");
			path.set_extension("toml");

			path
		};

		log!(debug, "writing test level to \"{}\"", test_level_path.display());

		// We do not care whether this succeeds or not.
		let _ = write(test_level_path, include_str!("test_level.toml"));

		Ok(data_dir)
	}

	fn init(&mut self) -> Result<()> {
		self.set_terminate_handler()?;

		self.regenerate_level();

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
