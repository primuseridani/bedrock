// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::{Block, Map};

use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::slice;

#[must_use]
#[derive(Debug)]
pub struct ColumnsMut<'a> {
	height: u32,

	len: u32,
	ptr: *mut Block,

	_map: PhantomData<&'a mut Map>,
}

impl<'a> ColumnsMut<'a> {
	#[inline(always)]
	pub(super) fn new(map: &'a mut Map) -> Self {
		let height = map.height();

		let len = map.width();
		let ptr = map.as_mut_ptr();

		Self {
			height,

			len,
			ptr,

			_map: PhantomData,
		}
	}
}

impl ExactSizeIterator for ColumnsMut<'_> { }

impl FusedIterator for ColumnsMut<'_> { }

impl<'a> Iterator for ColumnsMut<'a> {
	type Item = &'a mut [Block];

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		if self.len == 0x0 { return None };

		let height = self.height as usize;

		// SAFETY: We guarantee that the lengths of map
		// buffers are always a multiple of the map's
		// height. We have also in this case tested that
		// there are remaining columnsMut. We are also
		// guaranteed to exclusively access the buffer
		// thanks to `self._map`.
		let data = unsafe { slice::from_raw_parts_mut(self.ptr, height) };

		self.ptr = unsafe { self.ptr.add(height) };

		self.len -= 0x1;

		Some(data)
	}
}
