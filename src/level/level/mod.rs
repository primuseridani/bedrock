// Copyright 2025 Gabriel Bjørnager Jensen.

mod load_builtin;

use crate::graphics::Rgba;
use crate::level::Chunk;

#[derive(Clone, Debug)]
pub struct Level {
	pub name:        String,
	pub creatour:    String,
	pub description: String,

	pub background: Rgba,

	pub chunks: Vec<Chunk>,
}

impl Default for Level {
	#[inline(always)]
	fn default() -> Self {
		Self::load_builtin("lava_lake").unwrap()
	}
}
