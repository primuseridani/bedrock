// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::app::App;
use crate::error::{Error, Result};
use crate::level::Level;

use std::path::Path;

impl App {
	pub(super) fn load_level(base_dir: &Path, name: &str) -> Result<Level> {
		// Firstly check if the level is a built-in.

		if let Some(level) = Level::load_builtin(name) { return Ok(level) };

		let path = {
			let mut path = base_dir.to_owned();

			path.push("level");
			path.push(name);
			path.set_extension("toml");

			path
		};

		eprintln!("loading level at \"{}\"", path.display());

		// Do not actually load it for now.

		Err(Error::UnknownLevel { name: name.into() })
	}
}
