// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::Block;
use crate::map::{ColumnsMut, MapSize};

use std::hint::assert_unchecked;
use std::mem::swap;
use std::num::NonZero;

#[derive(Clone, Debug)]
pub struct Map {
	height: u32,
	data:   Box<[Block]>,
}

impl Map {
	#[inline]
	#[must_use]
	#[track_caller]
	pub fn new(size: MapSize) -> Self {
		let data_len = size.product() as usize;

		let data = vec![Default::default(); data_len].into();

		Self { height: size.height(), data }
	}

	#[inline]
	#[track_caller]
	pub fn resize(&mut self, size: MapSize) {
		let data_len = size.product() as usize;

		let mut data = Default::default();
		swap(&mut data, &mut self.data);

		let mut data: Vec<Block> = data.into();

 		data.clear();
		data.resize(data_len, Default::default());

		let mut data = data.into();

		swap(&mut data, &mut self.data);
	}

	#[inline(always)]
	#[must_use]
	pub fn sample(&self, x: f64, y: f64) -> Option<Block> {
		if x <= 0.0 || x >= f64::from(self.width()) {
			return None;
		}

		if y <= 0.0 || y >= f64::from(self.height()) {
			return None;
		}

		let x = x as u32;
		let y = y as u32;

		let index = x as usize * self.height as usize + y as usize;
		self.data.get(index).copied()
	}

	#[inline(always)]
	pub fn columns_mut(&mut self) -> ColumnsMut {
		ColumnsMut::new(self)
	}

	#[inline(always)]
	#[must_use]
	pub fn height(&self) -> u32 {
		let height = self.height;

		unsafe { assert_unchecked(height != 0x0) };
		unsafe { assert_unchecked(height % 0x2 == 0x0) };

		height
	}

	#[inline(always)]
	#[must_use]
	pub fn width(&self) -> u32 {
		// SAFETY: This is already guaranteed for compati-
		// bility with `MapSize`.
		let height = unsafe { NonZero::new_unchecked(self.height() as usize) };

		// NOTE: We always guarantee that the buffer's
		// total length is a multiple of `height`.
		let width = (self.data.len() / height) as u32;

		unsafe { assert_unchecked(width % 0x2 == 0x0) };

		width
	}

	#[inline(always)]
	#[must_use]
	pub fn size(&self) -> MapSize {
		let height = self.height();
		let width  = self.width();

		unsafe { MapSize::new_unchecked(width, height) }
	}

	#[expect(unused)]
	#[inline(always)]
	#[must_use]
	pub fn as_ptr(&self) -> *const Block {
		self.data.as_ptr()
	}

	#[inline(always)]
	#[must_use]
	pub fn as_mut_ptr(&mut self) -> *mut Block {
		self.data.as_mut_ptr()
	}
}

impl Default for Map {
	#[inline(always)]
	fn default() -> Self {
		Self::new(Default::default())
	}
}
