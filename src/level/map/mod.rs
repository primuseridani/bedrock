// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::{
	Block,
	Columns,
	ColumnsMut,
	MapIter,
	MapIterMut,
};

use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Map {
	pub(super) data: Box<[Box<[Block]>]>,
}

impl Map {
	#[inline]
	#[must_use]
	#[track_caller]
	pub fn new(width: u32, height: u32) -> Self {
		let columns = vec![Default::default(); height as usize].into_boxed_slice();

		let map = vec![columns; width as usize].into();

		Self { data: map }
	}

	#[inline(always)]
	#[must_use]
	pub fn get(&self, x: u32, y: u32) -> Option<&Block> {
		let cell = self.data.get(x as usize)?.get(y as usize)?;
		Some(cell)
	}

	#[inline(always)]
	#[must_use]
	pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut Block> {
		let cell = self.data.get_mut(x as usize)?.get_mut(y as usize)?;
		Some(cell)
	}

	#[inline(always)]
	pub fn iter(&self) -> MapIter {
		MapIter::new(self)
	}

	#[inline(always)]
	pub fn iter_mut(&mut self) -> MapIterMut {
		MapIterMut::new(self)
	}

	#[inline(always)]
	pub fn columns(&self) -> Columns {
		Columns::new(self)
	}

	#[inline(always)]
	pub fn columns_mut(&mut self) -> ColumnsMut {
		ColumnsMut::new(self)
	}
}

impl Index<(u32, u32)> for Map {
	type Output = Block;

	#[inline(always)]
	#[track_caller]
	fn index(&self, index: (u32, u32)) -> &Self::Output {
		&self.data[index.0 as usize][index.1 as usize]
	}
}

impl IndexMut<(u32, u32)> for Map {
	#[inline(always)]
	#[track_caller]
	fn index_mut(&mut self, index: (u32, u32)) -> &mut Self::Output {
		&mut self.data[index.0 as usize][index.1 as usize]
	}
}

impl<'a> IntoIterator for &'a Map {
	type Item = (u32, u32, &'a Block);

	type IntoIter = MapIter<'a>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a> IntoIterator for &'a mut Map {
	type Item = (u32, u32, &'a mut Block);

	type IntoIter = MapIterMut<'a>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter_mut()
	}
}
