// Copyright 2025 Gabriel Bjørnager Jensen.

use std::cmp::Ordering;
use std::fmt::{self, Debug, Display, Formatter};
use zerocopy::{FromZeros, Immutable, IntoBytes, KnownLayout};

type Buffer = [f32; 0x2];

const _: () = assert!(size_of::<Buffer>() == size_of::<f32>() * 0x2);

#[repr(align(0x8), C)]
#[derive(Clone, Copy, Default, FromZeros, Immutable, IntoBytes, KnownLayout)]
pub(super) struct Vec2(Buffer);

impl Vec2 {
	#[inline(always)]
	#[must_use]
	pub const fn new(x: f32, y: f32) -> Self {
		let buf = [x, y];
		Self(buf)
	}

	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (f32, f32) {
		let [x, y] = self.0;
		(x, y)
	}
}

impl Debug for Vec2 {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.get(), f)
	}
}

impl Display for Vec2 {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.get(), f)
	}
}

impl PartialEq for Vec2 {
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool {
		self.get() == other.get()
	}
}

impl PartialOrd for Vec2 {
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.get().partial_cmp(&other.get())
	}
}
