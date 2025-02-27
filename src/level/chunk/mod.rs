// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::Block;

#[derive(Clone, Debug)]
pub struct Chunk {
	pub terrain_height: f64,

	pub ground: Block,
}
