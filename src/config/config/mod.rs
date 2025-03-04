// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::{Level, MapSize};

#[derive(Clone, Debug, Default)]
pub struct Config {
	pub map_size: MapSize,
	pub level:    Level,
}
