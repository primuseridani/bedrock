// Copyright 2025 Gabriel Bjørnager Jensen.

use std::fmt::{self, Debug, Formatter};
use zerocopy::{FromZeros, Immutable, IntoBytes, KnownLayout};

#[repr(align(0x2), C)]
#[derive(Clone, Copy, Default, FromZeros, Immutable, IntoBytes, KnownLayout, PartialEq, PartialOrd)]
pub struct Vec2([f16; 0x2]);

impl Vec2 {
	#[inline(always)]
	#[must_use]
	pub const fn new(x: f16, y: f16) -> Self {
		let buf = [x, y];
		Self(buf)
	}

	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (f16, f16) {
		let [x, y] = self.0;
		(x, y)
	}
}

impl Debug for Vec2 {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.get(), f)
	}
}

impl From<(f16, f16)> for Vec2 {
	#[inline(always)]
	fn from((x, y): (f16, f16)) -> Self {
		Self::new(x, y)
	}
}
