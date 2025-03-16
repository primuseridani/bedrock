// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::graphics::Rgba;
use crate::level::{Chunk, Level, Material};

impl Level {
	pub fn load_builtin(name: &str) -> Option<Self> {
		match name {
			"field" => Some(Self {
				name:        "Field".into(),
				creatour:    "Achernar".into(),
				description: "A flat field.".into(),

				background: Rgba::from_u32(0x9DD8FEFF),

				chunks: vec![
					Chunk {
						terrain_height: 1.0 / 3.0,

						ground: Material::Dirt,
					},
				],
			}),

			"mountain" => Some(Self {
				name:        "Mountain".into(),
				creatour:    "Achernar".into(),
				description: "A simple mountain.".into(),

				background: Rgba::from_u32(0xD0D0D0FF),

				chunks: vec![
					Chunk {
						terrain_height: 1.0 / 3.0,

						ground: Material::Stone,
					},

					Chunk {
						terrain_height: 0.5,

						ground: Material::Stone,
					},
				],
			}),

			"valley" => Some(Self {
				name:        "Valley".into(),
				creatour:    "Achernar".into(),
				description: "A simple valley.".into(),

				background: Rgba::from_u32(0x017DA9FF),

				chunks: vec![
					Chunk {
						terrain_height: 0.5,

						ground: Material::Stone,
					},

					Chunk {
						terrain_height: 0.25,

						ground: Material::Dirt,
					},

					Chunk {
						terrain_height: 0.5,

						ground: Material::Stone,
					},
				],
			}),

			"lake" => Some(Self {
				name:        "Lake".into(),
				creatour:    "Achernar".into(),
				description: "A nice lake.".into(),

				background: Rgba::from_u32(0xD84F01FF),

				chunks: vec![
					Chunk {
						terrain_height: 0.25,

						ground: Material::Dirt,
					},

					Chunk {
						terrain_height: 0.125,

						ground: Material::Water,
					},

					Chunk {
						terrain_height: 0.25,

						ground: Material::Dirt,
					},
				],
			}),

			_ => None,
		}
	}
}
