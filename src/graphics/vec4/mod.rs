// Copyright 2025 Gabriel Bjørnager Jensen.

use std::cmp::Ordering;
use zerocopy::{
	FromZeros,
	Immutable,
	IntoBytes,
	KnownLayout,
	transmute,
};

#[cfg(target_arch = "x86")]
type Buffer = std::arch::x86::__m128;

#[cfg(target_arch = "x86_64")]
type Buffer = std::arch::x86_64::__m128;

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
type Buffer = [f32; 0x4];

const _: () = assert!(size_of::<Buffer>() == size_of::<f32>() * 0x4);

#[repr(align(0x10), C)]
#[derive(Clone, Copy, Debug, FromZeros, Immutable, IntoBytes, KnownLayout)]
pub(super) struct Vec4(Buffer);

impl Vec4 {
	#[inline(always)]
	#[must_use]
	pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
		let buf = [x, y, z, w];

		#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
		{
			let buf = transmute!(buf);
			Self(buf)
		}

		#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
		{
			Self(buf)
		}
	}

	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (f32, f32, f32, f32) {
		#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
		{
			let [x, y, z, w] = transmute!(self.0);
			(x, y, z, w)
		}

		#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
		{
			self.0
		}
	}
}

impl PartialEq for Vec4 {
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool {
		self.get() == other.get()
	}
}

impl PartialOrd for Vec4 {
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.get().partial_cmp(&other.get())
	}
}
