// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::Material;

/// A level layer.
#[derive(Clone, Debug)]
pub struct Layer {
	/// The height of the layer, relative to the map height.
	pub height: f64,

	/// The material used in the layer.
	pub material: Material,
}
