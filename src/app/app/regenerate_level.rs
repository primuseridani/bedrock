// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::app::App;
use crate::level::{Block, Chunk, Layer, Material};
use crate::log::{self, log};
use crate::map::Map;

use rand::random;

impl App {
	pub(super) fn regenerate_level(&mut self) {
		log!(debug, "generating level \"{}\"", self.level.name);

		log!(note, "config is: {:?}", self.config);
		log!(note, "level is: {:?}", self.level);

		assert!(self.level.chunks.len() <= u8::MAX as usize);

		self.map.resize(self.config.map_size);

		roll_seeds(&mut self.map);

		let mut chunks = self.level.chunks.iter();

		let mut current_chunk = None::<&Chunk>;

		let mut chunk_stop = 0x0u32;

		let columns = self
			.map
			.columns_mut()
			.enumerate()
			.map(|(x, column)| (x as u32, column));

		for (x, column) in columns {
			let chunk = if x >= chunk_stop {
				let Some(chunk) = chunks.next() else {
					break;
				};

				let chunk_start = x;
				let chunk_width = (f64::from(self.config.map_size.width()) * chunk.width) as u32;

				chunk_stop = chunk_start + chunk_width;

				chunk
			} else if let Some(chunk) = current_chunk {
				chunk
			} else {
				break;
			};

			generate_column(column, self.config.map_size.height(), chunk);

			current_chunk = Some(chunk);
		}

		fill_bedrock(&mut self.map);
	}
}
fn roll_seeds(map: &mut Map) {
	for cell in map.columns_mut().flat_map(<[_]>::iter_mut) {
		let seed = random();
		cell.set_seed(seed);
	}
}

fn generate_column(column: &mut [Block], map_height: u32, chunk: &Chunk) {
	let mut layers = chunk.layers.iter();

	let mut current_layer = None::<&Layer>;

	let mut layer_stop = 0x0u32;

	let cells = column
		.iter_mut()
		.enumerate()
		.map(|(y, block)| (y as u32, block));

	for (y, cell) in cells {
		let layer = if y >= layer_stop {
			let Some(layer) = layers.next() else {
				break;
			};

			let layer_start = y;
			let layer_width = (f64::from(map_height) * layer.height) as u32;

			layer_stop = layer_start + layer_width;

			layer
		} else if let Some(layer) = current_layer {
			layer
		} else {
			break;
		};

		cell.set_material(layer.material);

		current_layer = Some(layer);
	}
}

fn fill_bedrock(map: &mut Map) {
	for column in map.columns_mut() {
		for cell in column.iter_mut().take(0x1) {
			cell.set_material(Material::Bedrock);
		}
	}
}
