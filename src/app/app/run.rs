// Copyright 2025 Gabriel Bjørnager Jensen.

use rand::random;

use crate::app::App;
use crate::error::Result;
use crate::level::Block;

use std::mem::swap;
use std::thread::sleep_until;
use std::time::{Duration, Instant};

impl App {
	pub fn run(mut self) -> Result<()> {
		eprintln!("you have hit bedrock");

		Self::regenerate_level(
			&mut self.map,
			&self.config.level,
			self.config.map_size,
		);

		const TICK_DURATION: Duration = Duration::from_millis(250);

		let mut tick_start;

		loop {
			tick_start = Instant::now();

			let next_tick = tick_start + TICK_DURATION;

			self.tick();

			sleep_until(next_tick);
		}
	}

	fn tick(&mut self) {
		let seed = random::<u32>();

		for column in self.map.columns_mut() {
			for cells in column.chunks_exact_mut(0x2) {
				let [cell, next_cell] = cells else { unreachable!() };

				match (*cell, *next_cell) {
					(Block::Air, other)
					if other != Block::Air
					=> {
						swap(cell, next_cell);
					}

					(Block::Grass, other)
					if other != Block::Air
					&& seed >= 0x3FFFFFFF
					=> {
						*cell = Block::Dirt;
					}

					(Block::Magma, Block::Air)
					if seed >= 0x3FFFFFFF
					=> {
						*cell = Block::Basalt;
					}

					_ => { }
				}
			}
		}
	}
}
