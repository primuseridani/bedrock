// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::app::Config;
use crate::level::{LevelGenerator, Map};

use std::mem::swap;
use std::thread::sleep_until;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct App {
	config: Config,

	level_generator: LevelGenerator,
}

impl App {
	#[must_use]
	pub fn new(config: Config) -> Self {
		assert!(config.map_size.0 % 0x2 == 0x0);
		assert!(config.map_size.1 % 0x2 == 0x0);

		let level_generator = Self::load_level_generator(&config.level_generator);

		Self {
			config,

			level_generator,
		}
	}

	fn load_level_generator(name: &str) -> LevelGenerator {
		match name {
			"field" => LevelGenerator::FIELD,

			_ => panic!("invalid level generator \"{name}\""),
		}
	}

	pub fn run(self) {
		eprintln!("you have hit bedrock");

		let mut map = self.level_generator.generate(self.config.map_size.0, self.config.map_size.1);

		const TICK_DURATION: Duration = Duration::from_millis(250);

		let mut tick_start;

		loop {
			tick_start = Instant::now();

			let next_tick = tick_start + TICK_DURATION;

			self.tick(&mut map);

			sleep_until(next_tick);
		}
	}

	fn tick(&self, map: &mut Map) {
		for column in map.columns_mut() {
			for cells in column.chunks_exact_mut(0x2) {
				let [cell, next_cell] = cells else { unreachable!() };

				if *next_cell == Default::default() {
					swap(next_cell, cell);
				}
			}
		}
	}
}
