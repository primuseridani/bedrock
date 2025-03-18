// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::app::App;
use crate::level::Material;

use rand::random;
use std::mem::swap;

const fn min_seed(chance_num: u32, chance_den: u32) -> u32 {
	assert!(chance_den != 0x0);
	assert!(chance_den >= chance_num);

	let chance_num: u32 = chance_num;
	let chance_den: u32 = chance_den;

	let num = u32::MAX as u64 * chance_num as u64;
	let den = chance_den as u64;

	u32::MAX - (num / den) as u32
}

macro_rules! select_events {
	{
		Meta {
			seed: $seed:expr$(,)?
		}$(,)?

		$(
			Event {
				lhs:    $lhs_expr:expr,
				rhs:    $rhs_expr:expr,
				chance: $chance_num:literal / $chance_den:literal$(,)?
			} => $op:block$(,)?
		)*
	} => {{
		let seed: u32 = $seed;

		$({
			let min_seed = ::bedrock::app::app::tick::min_seed($chance_num, $chance_den);

			if seed >= min_seed && $lhs_expr && $rhs_expr {
				$op;
			}
		})*
	}};
}

impl App {
	pub(super) fn tick(&mut self) {
		let mut columns = self.map.columns_mut();

		while let Some(mut windows) = columns.next_as_windows_mut() {
			while let Some([block, next_block]) = windows.next() {
				select_events! {
					Meta {
						seed: random(),
					}

					Event {
						lhs:    block.is_emtpy() || block.is_liquid(),
						rhs:    !next_block.is_static() && !next_block.is_liquid(),
						chance: 0x1F / 0x20,
					} => {
						swap(block, next_block);
					}

					Event {
						lhs:    block.is_liquid(),
						rhs:    !next_block.is_static() && next_block.is_liquid(),
						chance: 0x1 / 0x8,
					} => {
						swap(block, next_block);

						// Skip the next block.
						let _ = windows.next();
					}

					Event {
						lhs:    block.material() == Material::Dirt,
						rhs:    next_block.is_emtpy(),
						chance: 0x1 / 0x40,
					} => {
						block.set_material(Material::Grass);
					}

					Event {
						lhs:    block.material() == Material::Grass,
						rhs:    !next_block.is_emtpy(),
						chance: 0x1 / 0x80,
					} => {
						block.set_material(Material::Dirt);
					}

					Event {
						lhs:    block.material() == Material::Fire,
						rhs:    true,
						chance: 0x1 / 0x20,
					} => {
						block.set_material(Material::Air);
					}
				}
			}
		}
	}
}
