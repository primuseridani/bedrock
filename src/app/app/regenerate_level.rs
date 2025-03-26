// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::app::App;
use crate::level::{Block, Material};
use crate::log::log;

use rand::random;

impl App {
	pub(super) fn regenerate_level(&mut self) {
		log!(debug, "generating level \"{}\"", self.level.name);

		log!(note, "config is: {:?}", self.config);
		log!(note, "level is: {:?}", self.level);

		assert!(!self.level.chunks.is_empty());

		assert!(self.level.chunks.len() <= u8::MAX as usize);

		self.map.resize(self.config.map_size);

		let mut chunks = self.level.chunks.iter();

		let chunk_count = chunks.len() as u8;

		let mut chunk_index = 0x0u8;
		let mut chunk_stop  = 0x0u32;
		let mut chunk       = None;

		let columns = self
			.map
			.columns_mut()
			.enumerate()
			.map(|(x, column)| (x as u32, column));

		for (x, column) in columns {
			while x >= chunk_stop {
				chunk_index += 0x1;

				// NOTE: This cannot overflow as even with ex-
				// tremes:
				//
				//           FFFFFFFE u32
				// *               FF u8
				// = 000000FEFFFFFE02 u64
				//
				// ... which nicely fits in `u64`. `chunk_count`
				// can additionally not be null. Now also remember
				// that it is tied to `chunk_index`; if the latter
				// is great then the former must also be great,
				// cancelling each other out:
				//
				//   000000FEFFFFFE02 u64
				// /               FF u8
				// =         FFFFFFFE u32
				chunk_stop = (self.config.map_size.width() as u64 * chunk_index as u64 / chunk_count as u64) as u32;

				chunk = chunks.next();
			}

			let chunk = chunk.unwrap();

			let terrain_height = (f64::from(self.config.map_size.height()) * chunk.terrain_height.clamp(0.0, 1.0)) as u32;

			let blocks = column
				.iter_mut()
				.enumerate()
				.map(|(y, block)| (y as u32, block));

			for (y, slot) in blocks {
				let mut block = Block::new(Default::default(), random());

				if y == 0x0 {
					block.set_material(Material::Bedrock);
				} else if y < terrain_height {
					block.set_material(chunk.ground);
				}

				*slot = block;
			}
		}
	}
}
