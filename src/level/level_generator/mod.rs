// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::{Block, Map};

#[derive(Debug)]
pub struct LevelGenerator {
	pub terrain_height: u32,

	pub ground: Block,
}

impl LevelGenerator {
	pub const FIELD: Self = Self {
		terrain_height: 0x30,

		ground: Block::Dirt,
	};

	pub fn generate(&self, width: u32, height: u32) -> Map {
		assert!(width  > 0x0);
		assert!(height > 0x0);

		let mut map = Map::new(width, height);

		for (_, y, cell) in &mut map {
			if y == height - 0x1 {
				*cell = Block::Bedrock;
			} else if y < self.terrain_height {
				*cell = self.ground;
			}
		}

		map
	}
}

impl Default for LevelGenerator {
	#[inline(always)]
	fn default() -> Self {
		Self::FIELD
	}
}
