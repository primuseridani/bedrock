// Copyright 2025 Gabriel Bjørnager Jensen.

mod application_handler;
mod handle_input;
mod load_level;
mod regenerate_level;
mod run;
mod tick;

use crate::config::Config;
use crate::error::{Error, Result};
use crate::graphics::{GraphicsContext, MIN_VIEW_SCALE};
use crate::level::Map;
use crate::log::log;

use std::fs::{create_dir_all, write};
use std::path::PathBuf;

#[allow(deprecated)]
use std::env::home_dir;

#[derive(Debug)]
pub struct App {
	data_dir: PathBuf,

	config: Config,

	map: Map,

	graphics_context: Option<GraphicsContext>,

	view_scale: u32,
}

impl App {
	pub fn new() -> Result<Self> {
		let data_dir = Self::get_data_dir()?;

		let this = Self {
			data_dir,

			config: Default::default(),

			map: Default::default(),

			view_scale: MIN_VIEW_SCALE,

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

		let _ = write(test_level_path, include_str!("test_level.toml"));

		Ok(data_dir)
	}
}
