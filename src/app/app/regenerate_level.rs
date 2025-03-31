// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::app::App;
use crate::level::{Block, Chunk, Layer, Level, Material};
use crate::log::log;
use crate::map::{Map, MapSize};

use rand::random;
use std::borrow::BorrowMut;

trait Segment {
	#[must_use]
	fn size(&self) -> f64;
}

impl Segment for Chunk {
	#[inline(always)]
	fn size(&self) -> f64 {
		self.width
	}
}

impl Segment for Layer {
	#[inline(always)]
	fn size(&self) -> f64 {
		self.height
	}
}

impl<T: Segment> Segment for &T {
	#[inline(always)]
	fn size(&self) -> f64 {
		(*self).size()
	}
}

#[must_use]
#[derive(Debug, Default)]
struct SegmentTracker<I>
where
	I:       Iterator,
	I::Item: Segment,
{
	iter:    I,
	current: Option<I::Item>,

	total_size: f64,
	next:       u32,
}

impl<I> SegmentTracker<I>
where
	I:       Iterator,
	I::Item: Segment,
{
	#[inline]
	pub fn new<J>(iter: J, total_size: u32) -> Self
	where
		J: IntoIterator<IntoIter = I>,
	{
		Self {
			iter:    iter.into_iter(),
			current: Default::default(),

			total_size: total_size.into(),
			next:       Default::default(),
		}
	}

	pub fn next(&mut self, coordinate: u32) -> Option<I::Item>
	where
		I::Item: Copy,
	{
		if coordinate >= self.next {
			let segment = self.iter.next()?;

			assert!(segment.size() >= 0.0);
			assert!(segment.size() <= 1.0);

			let width = (self.total_size * segment.size()) as u32;

			self.next = coordinate + width;

			self.current = Some(segment);

			Some(segment)
		} else {
			self.current
		}
	}
}

impl App {
	pub(super) fn regenerate_level(&mut self) {
		log!(debug, "generating level \"{}\"", self.level.name);

		log!(note, "config is: {:?}", self.config);
		log!(note, "level is: {:?}", self.level);

		assert!(self.level.chunks.len() <= u8::MAX as usize);

		self.map.resize(self.config.map_size);

		roll_seeds(&mut self.map);

		generate_columns(
			self.map.columns_mut(),
			&self.level,
			self.config.map_size,
		);

		fill_bedrock(&mut self.map);
	}
}

fn roll_seeds(map: &mut Map) {
	for cell in map.columns_mut().flat_map(<[_]>::iter_mut) {
		let seed = random();
		cell.set_seed(seed);
	}
}

fn generate_columns<I: IntoIterator<Item: BorrowMut<[Block]>>>(
	columns:  I,
	level:    &Level,
	map_size: MapSize,
) {
	let mut chunks = SegmentTracker::new(
		&*level.chunks,
		map_size.width(),
	);

	let columns = columns
		.into_iter()
		.enumerate()
		.map(|(x, column)| (x as u32, column));

	for (x, mut column) in columns {
		let Some(chunk) = chunks.next(x) else {
			break;
		};

		let mut layers = SegmentTracker::new(
			&*chunk.layers,
			map_size.height(),
		);

		let cells = column
			.borrow_mut()
			.iter_mut()
			.enumerate()
			.map(|(y, block)| (y as u32, block));

		for (y, cell) in cells {
			let Some(layer) = layers.next(y) else {
				break;
			};

			cell.set_material(layer.material);
		}
	}
}

fn fill_bedrock(map: &mut Map) {
	for column in map.columns_mut() {
		for cell in column.iter_mut().take(0x1) {
			cell.set_material(Material::Bedrock);
		}
	}
}
