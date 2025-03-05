// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::{Block, Columns, ColumnsMut, MapSize};

use std::hint::assert_unchecked;
use std::mem::swap;
use std::num::NonZero;
use std::ops::{Index, IndexMut};
use std::slice;

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

	#[allow(unused)]
	#[inline(always)]
	#[must_use]
	pub fn get(&self, x: u32, y: u32) -> Option<&Block> {
		let index = x as usize * self.height as usize + y as usize;
		self.data.get(index)
	}

	#[allow(unused)]
	#[inline(always)]
	#[must_use]
	pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut Block> {
		let index = x as usize * self.height as usize + y as usize;
		self.data.get_mut(index)
	}

	#[allow(unused)]
	#[inline(always)]
	pub fn iter(&self) -> slice::Iter<Block> {
		self.data.iter()
	}

	#[allow(unused)]
	#[inline(always)]
	pub fn iter_mut(&mut self) -> slice::IterMut<Block> {
		self.data.iter_mut()
	}

	#[allow(unused)]
	#[inline(always)]
	pub fn columns(&self) -> Columns {
		Columns::new(self)
	}

	#[allow(unused)]
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
		// SAFETY: This is guaranteed for compatibility
		// with `MapSize`.
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

impl Index<(u32, u32)> for Map {
	type Output = Block;

	#[inline(always)]
	#[track_caller]
	fn index(&self, index: (u32, u32)) -> &Self::Output {
		self.get(index.0, index.1).unwrap()
	}
}

impl IndexMut<(u32, u32)> for Map {
	#[inline(always)]
	#[track_caller]
	fn index_mut(&mut self, index: (u32, u32)) -> &mut Self::Output {
		self.get_mut(index.0, index.1).unwrap()
	}
}

impl<'a> IntoIterator for &'a Map {
	type Item = &'a Block;

	type IntoIter = slice::Iter<'a, Block>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a> IntoIterator for &'a mut Map {
	type Item = &'a mut Block;

	type IntoIter = slice::IterMut<'a, Block>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter_mut()
	}
}
