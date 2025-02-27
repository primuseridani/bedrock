// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::{Block, Map};

use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::slice;

#[must_use]
#[derive(Clone, Debug)]
pub struct Columns<'a> {
	height: u32,

	len: u32,
	ptr: *const Block,

	_map: PhantomData<&'a Map>,
}

impl<'a> Columns<'a> {
	#[inline(always)]
	pub(super) fn new(map: &'a Map) -> Self {
		let height = map.height();

		let len = map.width();
		let ptr = map.as_ptr();

		Self {
			height,

			len,
			ptr,

			_map: PhantomData,
		}
	}
}

impl ExactSizeIterator for Columns<'_> { }

impl FusedIterator for Columns<'_> { }

impl<'a> Iterator for Columns<'a> {
	type Item = &'a [Block];

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		if self.len == 0x0 { return None };

		let height = self.height as usize;

		// SAFETY: We guarantee that the lengths of map
		// buffers are always a multiple of the map's
		// height. We have also in this case tested that
		// there are remaining columns.
		let data = unsafe { slice::from_raw_parts(self.ptr, height) };

		self.ptr = unsafe { self.ptr.add(height) };

		self.len -= 0x1;

		Some(data)
	}
}
