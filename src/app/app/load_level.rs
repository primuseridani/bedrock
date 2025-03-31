// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::app::App;
use crate::error::{Error, Result};
use crate::level::{Chunk, Layer, Level};
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
	pub width: f64,

	pub is_spawnable: bool,

	pub layer: Vec<LevelChunkLayerHelper>,
}

#[derive(Debug, Deserialize)]
struct LevelChunkLayerHelper {
	pub height:   f64,
	pub material: String,
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
				let mut path = self.data_dir.clone();

				path.push("level");
				path.push(name);
				path.set_extension("toml");

				path
			};

			log!(debug, "loading level at \"{}\"", path.display());

			let file = read_to_string(&path)
				.map_err(|e| Error::UnknownLevel { path: path.clone().into(), source: Box::new(e) })?;

			let parse_chunk_layer = |helper: LevelChunkLayerHelper| -> Result<Layer> {
				let material = helper
					.material
					.parse()
					.map_err(|e| Error::UnknownLevel { path: path.clone().into(), source: Box::new(e) })?;

				let layer = Layer {
					height:   helper.height,
					material,
				};

				Ok(layer)
			};

			let parse_chunk = |helper: LevelChunkHelper| -> Result<Chunk> {
				let layers = helper
					.layer
					.into_iter()
					.map(parse_chunk_layer)
					.collect::<Result<_>>()?;

				let chunk = Chunk {
					width: helper.width,

					is_spawnable: helper.is_spawnable,

					layers,
				};

				Ok(chunk)
			};

			let helper = toml::from_str::<LevelHelper>(&file)
				.map_err(|e| Error::UnknownLevel { path: path.clone().into(), source: Box::new(e) })?;

			let chunks = helper
				.chunk
				.into_iter()
				.map(parse_chunk)
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
