// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::{Block, Map};

use std::{iter::FusedIterator, slice};

#[must_use]
#[derive(Debug)]
pub struct MapIterMut<'a> {
	x: u32,
	y: u32,

	rows:    slice::IterMut<'a, Box<[Block]>>,
	columns: Option<slice::IterMut<'a, Block>>,
}

impl<'a> MapIterMut<'a> {
	#[inline(always)]
	pub(super) fn new(map: &'a mut Map) -> Self {
		Self {
			x: 0x0,
			y: 0x0,

			rows:    map.data.iter_mut(),
			columns: None,
		}
	}
}

impl FusedIterator for MapIterMut<'_> { }

impl<'a> Iterator for MapIterMut<'a> {
	type Item = (u32, u32, &'a mut Block);

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		if self.columns.is_none() {
			self.x = self.x.saturating_add(0x1);
			self.y = Default::default();

			self.columns = self.rows.next().map(IntoIterator::into_iter);
		}

		match self.columns {
			Some(ref mut columns) => {
				let x = self.x;
				let y = self.y;

				self.y = self.y.saturating_add(0x1);

				columns.next().map(|cell| (x, y, cell))
			}

			_ => None,
		}
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let min = 0x0;
		let max = u32::MAX as usize * u32::MAX as usize;

		(min, Some(max))
	}
}
