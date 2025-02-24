// Copyright 2025 Gabriel Bjørnager Jensen.

#[derive(Debug)]
pub struct Config {
	pub map_size: (u32, u32),

	pub level_generator: String,
}

impl Default for Config {
	#[inline(always)]
	fn default() -> Self {
		Self {
			map_size: (0x100, 0x90),

			level_generator: "field".into(),
		}
	}
}
