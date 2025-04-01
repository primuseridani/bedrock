// Copyright 2025 Gabriel Bjørnager Jensen.

use std::fmt::{self, Debug, Display, Formatter};
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};

/// A block seed.
///
/// Each [block](crate::level::Block) has contains a seed that defines the block's variant.
/// Along with the block's [material](crate::level::Material), this seed defines the properties of the block.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[rustc_layout_scalar_valid_range_end(0x3)]
pub struct Seed(u8);

impl Seed {
	/// Creates a new block seed.
	///
	/// See [`new_unchecked`](Self::new_unchecked) for the unsafe version of this function.
	///
	/// The provided value must be in the range `0..=3`.
	#[inline(always)]
	#[must_use]
	pub const fn new(value: u8) -> Option<Self> {
		if value > 0x3 {
			return None;
		}

		let this = unsafe { Self::new_unchecked(value) };
		Some(this)
	}

	/// Unsafely creates a new block seed.
	///
	/// See [`new`](Self::new) for the safe version of this function.
	///
	/// # Safety
	///
	/// The provided value must be in the range `0..=3`.
	#[inline(always)]
	#[must_use]
	pub const unsafe fn new_unchecked(value: u8) -> Self {
		debug_assert!(value <= 0x3);

		unsafe { Self(value) }
	}

	/// Converts the seed into `u8`.
	///
	/// The returned value is always in the range `0..=3`.
	#[inline(always)]
	#[must_use]
	pub const fn to_u8(self) -> u8 {
		self.0
	}
}

impl Debug for Seed {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.0, f)
	}
}

impl Default for Seed {
	#[inline(always)]
	fn default() -> Self {
		const { Self::new(0x0).unwrap() }
	}
}

impl Display for Seed {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl Distribution<Seed> for StandardUniform {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Seed {
		let mut value: u8 = self.sample(rng);

		value &= 0b00000011;

		unsafe { Seed::new_unchecked(value) }
	}
}
