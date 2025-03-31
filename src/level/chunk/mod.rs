// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::Layer;

use std::borrow::Cow;

/// A level chunk.
///
/// Levels are made up of a non-zero amount of segments that are called *chunks*.
/// Each chunk defines additional *layers* itself (see [`Layer`]).
#[derive(Clone, Debug)]
pub struct Chunk {
	/// The width of the chunk, relative to the map width.
	pub width: f64,

	/// Denotes whether players can spawn in the chunk.
	pub is_spawnable: bool,

	/// The layers of the chunk.
	pub layers: Cow<'static, [Layer]>,
}
