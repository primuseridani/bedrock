// Copyright 2025 Gabriel Bjørnager Jensen.

use std::cmp::Ordering;
use std::fmt::{self, Debug, Display, Formatter};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

type Buffer = [u8; 0x4];

const _: () = assert!(size_of::<Buffer>() == size_of::<u8>() * 0x4);

#[repr(align(0x4), C)]
#[derive(Clone, Copy, FromBytes, Immutable, IntoBytes, KnownLayout)]
pub(super) struct Rgba(Buffer);

impl Rgba {
	#[inline(always)]
	#[must_use]
	pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
		let buf = [r, g, b, a];
		Self(buf)
	}

	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (u8, u8, u8, u8) {
		let [r, g, b, a] = self.0;
		(r, g, b, a)
	}
}

impl Debug for Rgba {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.get(), f)
	}
}

impl Default for Rgba {
	#[inline(always)]
	fn default() -> Self {
		Self::new(0x00, 0x00, 0x00, 0xFF)
	}
}

impl Display for Rgba {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.get(), f)
	}
}

impl PartialEq for Rgba {
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool {
		self.get() == other.get()
	}
}

impl PartialOrd for Rgba {
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.get().partial_cmp(&other.get())
	}
}
