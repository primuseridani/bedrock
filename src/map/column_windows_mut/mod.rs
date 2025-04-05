// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::Block;
use crate::map::ColumnsMut;

use std::marker::PhantomData;

#[must_use]
#[derive(Debug)]
pub struct ColumnWindowsMut<'a> {
	len: usize,
	ptr: *mut Block,

	_lifetime: PhantomData<&'a mut [Block]>,
}

impl<'a> ColumnWindowsMut<'a> {
	#[inline(always)]
	#[must_use]
	pub(super) fn new(columns: &mut ColumnsMut<'a>) -> Option<Self> {
		let column = columns.next()?;

		let len = column.len();
		let ptr = column.as_mut_ptr();

		debug_assert!(len % 0x2 == 0x0);

		let this = Self {
			len,
			ptr,
			_lifetime: PhantomData,
		};

		Some(this)
	}

	#[inline]
	#[must_use]
	pub fn next(&mut self) -> Option<[&'a mut Block; 0x2]> {
		if self.is_empty() {
			return None;
		}

		// SAFETY: `ptr` is always within bounds if `len`
		// is non-null.
		let block      = unsafe { &mut *self.ptr };

		// SAFETY: `ptr + 0x1` is always within bounds if
		// `len` is at least one.
		let next_block = unsafe { &mut *self.ptr.add(0x1) };

		self.ptr  = unsafe { self.ptr.add(0x1) };
		self.len -= 0x1;

		Some([block, next_block])
	}

	#[inline(always)]
	#[must_use]
	pub fn len(&self) -> usize {
		self.len
	}

	#[inline(always)]
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.len() == 0x1
	}
}
