// Copyright 2025 Gabriel Bjørnager Jensen.

use rand::random;

use crate::app::App;
use crate::level::Block;

use std::mem::swap;

impl App {
	pub(super) fn tick(&mut self) {
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
