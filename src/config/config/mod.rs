// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::map::MapSize;

#[derive(Clone, Debug, Default)]
pub struct Config {
	pub map_size: MapSize,

	pub tps: u16 = 0x8,
}
