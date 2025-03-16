// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::Material;

#[derive(Clone, Debug)]
pub struct Chunk {
	pub terrain_height: f64,

	pub ground: Material,
}
