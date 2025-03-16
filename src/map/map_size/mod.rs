// Copyright 2025 Gabriel Bjørnager Jensen.

use std::hint::assert_unchecked;
use std::num::NonZero;

#[derive(Clone, Copy, Debug)]
pub struct MapSize {
	width:  NonZero<u32>,
	height: NonZero<u32>,
}

impl MapSize {
	#[inline(always)]
	#[must_use]
	pub const fn new(width: u32, height: u32) -> Option<Self> {
		if width  % 0x2 != 0x0 { return None };
		if height % 0x2 != 0x0 { return None };

		if width  == 0x0 { return None };
		if height == 0x0 { return None };

		// FIXME(const-hacks): We cannot try in constant
		// expressions.
		if width.checked_mul(height).is_none() {
			return None;
		}

		// SAFETY: We have tested that the two axes:
		//
		// * Are even,
		// * Are non-zero, and
		// * Produce a non-overflowing product.
		let this = unsafe { Self::new_unchecked(width, height) };
		Some(this)
	}

	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub const unsafe fn new_unchecked(width: u32, height: u32) -> Self {
		debug_assert!(width  % 0x2 == 0x0);
		debug_assert!(height % 0x2 == 0x0);

		debug_assert!(width  != 0x0);
		debug_assert!(height != 0x0);

		// SAFETY: The caller guarantees that the provided
		// width is **NOT** null.
		let width = unsafe { NonZero::new_unchecked(width) };

		// SAFETY: The caller guarantees that the provided
		// height is **NOT** null.
		let height = unsafe { NonZero::new_unchecked(height) };

		// SAFETY: The caller guarantees that the provided
		// parts are both even and that their product does
		// not overflow.
		Self { width, height }
	}

	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (u32, u32) {
		let width  = self.width();
		let height = self.height();

		(width, height)
	}

	#[inline(always)]
	#[must_use]
	pub const fn width(self) -> u32 {
		let width = self.width.get();

		// SAFETY: This is always guaranteed.
		unsafe { assert_unchecked(width % 0x2 == 0x0) };

		width
	}

	#[inline(always)]
	#[must_use]
	pub const fn height(self) -> u32 {
		let height = self.height.get();

		// SAFETY: This is always guaranteed.
		unsafe { assert_unchecked(height % 0x2 == 0x0) };

		height
	}

	#[inline(always)]
	#[must_use]
	pub const fn product(self) -> u32 {
		let width  = self.width();
		let height = self.height();

		// SAFETY: We always guarantee that the product
		// does not overflow `u32`.
		unsafe { width.unchecked_mul(height) }
	}
}

impl Default for MapSize {
	#[inline(always)]
	fn default() -> Self {
		Self::new(0x180, 0x100).unwrap()
	}
}
