// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::Level;
use crate::map::MapSize;

#[derive(Clone, Debug, Default)]
pub struct Preset {
	pub map_size: MapSize,
	pub level:    Level,
}
