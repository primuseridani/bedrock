// Copyright 2025 Gabriel Bjørnager Jensen.

use rand::random;

use crate::app::App;
use crate::level::Material;

use std::mem::swap;

impl App {
	pub(super) fn tick(&mut self) {
		let seed = random::<u32>();

		for column in self.map.columns_mut() {
			for blocks in column.chunks_exact_mut(0x2) {
				let [block, next_cell] = blocks else { unreachable!() };

				match (block.material(), next_cell.material()) {
					(Material::Air, other)
					if other != Material::Air
					=> {
						swap(block, next_cell);
					}

					(Material::Grass, other)
					if other != Material::Air
					&& seed >= 0x3FFFFFFF
					=> {
						block.set_material(Material::Dirt);
					}

					(Material::Magma, Material::Air)
					if seed >= 0x3FFFFFFF
					=> {
						block.set_material(Material::Basalt);
					}

					_ => { }
				}
			}
		}
	}
}
