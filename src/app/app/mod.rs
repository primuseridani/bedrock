// Copyright 2025 Gabriel Bjørnager Jensen.

mod application_handler;
mod handle_key_event;
mod load_level;
mod print_welcome_message;
mod regenerate_level;
mod run;
mod tick;

use crate::config::Config;
use crate::error::{Error, Result};
use crate::graphics::GraphicsContext;
use crate::level::Map;

use std::fs::{create_dir_all, write};
use std::path::PathBuf;

#[allow(deprecated)]
use std::env::home_dir;

#[derive(Debug)]
pub struct App {
	config: Config,

	data_dir: PathBuf,

	map: Map,

	graphics_context: Option<GraphicsContext>,
}

impl App {
	pub fn new() -> Result<Self> {
		let data_dir = Self::get_data_dir()?;

		let this = Self {
			config: Default::default(),

			data_dir,

			map: Default::default(),

			graphics_context: Default::default(),
		};

		Ok(this)
	}

	#[allow(deprecated)]
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

		eprintln!("creating data directory at \"{}\"", data_dir.display());

		create_dir_all(&data_dir)
			.map_err(|_| Error::MissingDataDir)?;

		let subdirs = [
			"level",
		];

		for subdir in subdirs {
			let subdir = data_dir.join(subdir);

			eprintln!("creating subdirectory at \"{}\"", subdir.display());

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

		eprintln!("writing test level to \"{}\"", test_level_path.display());

		let _ = write(test_level_path, include_str!("test_level.toml"));

		Ok(data_dir)
	}
}
