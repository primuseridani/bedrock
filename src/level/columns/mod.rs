// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::{Block, Map};

use std::iter::FusedIterator;
use std::slice;

#[must_use]
#[derive(Debug)]
pub struct Columns<'a> {
	iter: slice::Iter<'a, Box<[Block]>>,
}

impl<'a> Columns<'a> {
	#[inline(always)]
	pub(super) fn new(map: &'a Map) -> Self {
		let iter = map.data.iter();
		Self { iter }
	}
}

impl ExactSizeIterator for Columns<'_> { }

impl FusedIterator for Columns<'_> { }

impl<'a> Iterator for Columns<'a> {
	type Item = &'a [Block];

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|column| &**column)
	}
}
