// Copyright 2025 Gabriel Bjørnager Jensen.

use std::fmt::{self, Debug, Display, Formatter};
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[rustc_layout_scalar_valid_range_end(0x3)]
pub struct Seed(u8);

impl Seed {
	#[inline(always)]
	#[must_use]
	pub const fn new(value: u8) -> Option<Self> {
		if value > 0x3 {
			return None;
		}

		let this = unsafe { Self::new_unchecked(value) };
		Some(this)
	}

	#[inline(always)]
	#[must_use]
	pub const unsafe fn new_unchecked(value: u8) -> Self {
		debug_assert!(value <= 0x3);

		unsafe { Self(value) }
	}

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
