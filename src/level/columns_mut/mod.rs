// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::{Block, Map};

use std::iter::FusedIterator;
use std::slice;

#[must_use]
#[derive(Debug)]
pub struct ColumnsMut<'a> {
	iter: slice::IterMut<'a, Box<[Block]>>,
}

impl<'a> ColumnsMut<'a> {
	#[inline(always)]
	pub(super) fn new(map: &'a mut Map) -> Self {
		let iter = map.data.iter_mut();
		Self { iter }
	}
}

impl ExactSizeIterator for ColumnsMut<'_> { }

impl FusedIterator for ColumnsMut<'_> { }

impl<'a> Iterator for ColumnsMut<'a> {
	type Item = &'a mut [Block];

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|column| &mut **column)
	}
}
