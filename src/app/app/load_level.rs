// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::app::App;
use crate::error::{Error, Result};
use crate::level::{Chunk, Level};
use crate::log::log;

use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Debug, Deserialize)]
struct LevelHelper {
	pub level: LevelLevelHelper,

	pub chunk: Vec<LevelChunkHelper>,
}

#[derive(Debug, Deserialize)]
struct LevelLevelHelper {
	pub name:        String,
	pub creatour:    String,
	pub description: String,

	pub background: String,
}

#[derive(Debug, Deserialize)]
struct LevelChunkHelper {
	pub terrain_height: f64,
	pub ground:         String,
}

impl App {
	pub(super) fn load_level(&self, name: &str) -> Result<Level> {
		// Firstly check if the level is a built-in.

		log!("loading level \"{name}\"");

		let level = if let Some(level) = Level::load_builtin(name) {
			log!(debug, "loading built-in level \"{name}\"");

			level
		} else {
			log!(note, "level is not a built-in");

			let path = {
				let mut path = self.data_dir.to_owned();

				path.push("level");
				path.push(name);
				path.set_extension("toml");

				path
			};

			log!(debug, "loading level at \"{}\"", path.display());

			let file = read_to_string(&path)
				.map_err(|e| Error::UnknownLevel { path: path.clone().into(), source: Box::new(e) })?;

			let helper = toml::from_str::<LevelHelper>(&file)
				.map_err(|e| Error::UnknownLevel { path: path.clone().into(), source: Box::new(e) })?;

			let chunks = helper
				.chunk
				.into_iter()
				.map(|chunk_helper| {
					let chunk = Chunk {
						terrain_height: chunk_helper.terrain_height,

						ground: {
							chunk_helper
								.ground
								.parse()
								.map_err(|e| Error::UnknownLevel { path: path.clone().into(), source: Box::new(e) })?
						}
					};

					Ok(chunk)
				})
				.collect::<Result<_>>()?;

			let background = helper
				.level
				.background
				.parse()
				.map_err(|e| Error::UnknownLevel { path: path.clone().into(), source: Box::new(e) })?;

			Level {
				name:        helper.level.name.into(),
				creatour:    helper.level.creatour.into(),
				description: helper.level.description.into(),

				background,

				chunks,
			}
		};

		log!(note, "loaded level:\n```\n{level:#?}\n```");

		Ok(level)
	}
}
