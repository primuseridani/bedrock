// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::{Block, Chunk, Level};

impl Level {
	pub fn load_builtin(name: &str) -> Option<Self> {
		match name {
			"field" => Some(Self {
				name:        "Field".into(),
				creatour:    "Achernar".into(),
				description: "A flat field.".into(),

				chunks: vec![
					Chunk {
						terrain_height: 1.0 / 3.0,

						ground: Block::Dirt,
					},
				],
			}),

			"mountain" => Some(Self {
				name:        "Mountain".into(),
				creatour:    "Achernar".into(),
				description: "A simple mountain.".into(),

				chunks: vec![
					Chunk {
						terrain_height: 1.0 / 3.0,

						ground: Block::Stone,
					},

					Chunk {
						terrain_height: 0.5,

						ground: Block::Stone,
					},
				],
			}),

			"valley" => Some(Self {
				name:        "Valley".into(),
				creatour:    "Achernar".into(),
				description: "A simple valley.".into(),

				chunks: vec![
					Chunk {
						terrain_height: 0.5,

						ground: Block::Stone,
					},

					Chunk {
						terrain_height: 0.125,

						ground: Block::Dirt,
					},

					Chunk {
						terrain_height: 0.5,

						ground: Block::Stone,
					},
				],
			}),

			_ => None,
		}
	}
}
