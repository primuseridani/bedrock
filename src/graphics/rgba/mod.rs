// Copyright 2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::graphics::RgbaFromStrError;

use std::fmt::{self, Debug, Display, Formatter};
use std::ops::RangeInclusive;
use std::str::FromStr;
use zerocopy::{FromZeros, Immutable, IntoBytes};

#[repr(transparent)]
#[derive(Clone, Copy, Default, Eq, FromZeros, Immutable, IntoBytes, Ord, PartialEq, PartialOrd)]
pub struct Rgba([u8; 0x4]);

impl Rgba {
	#[inline(always)]
	#[must_use]
	pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
		let data = [r, g, b, a];
		Self(data)
	}

	#[inline(always)]
	#[must_use]
	pub const fn from_u32(value: u32) -> Self {
		let data = value.to_be_bytes();
		Self(data)
	}

	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> (u8, u8, u8, u8) {
		let [r, g, b, a] = self.0;
		(r, g, b, a)
	}

	#[inline(always)]
	#[must_use]
	pub const fn to_u32(self) -> u32 {
		let data = self.0;
		u32::from_be_bytes(data)
	}

	#[must_use]
	pub fn to_wgpu_color_lossy(self) -> wgpu::Color {
		// Convert perceptual RGB to linear RGB.

		let (r, g, b, a) = self.get();

		let mut r = f64::from(r) / f64::from(u8::MAX);
		let mut g = f64::from(g) / f64::from(u8::MAX);
		let mut b = f64::from(b) / f64::from(u8::MAX);
		let     a = f64::from(a) / f64::from(u8::MAX);

		for slot in [&mut r, &mut g, &mut b] {
			let mut value = *slot;

			value = if value > 0.040_450 {
				((value + 0.055) / 1.055).powf(2.4)
			} else {
				value / 12.920
			};

			*slot = value;
		}

		wgpu::Color { r, g, b, a }
	}
}

impl Debug for Rgba {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(self, f)
	}
}

impl Display for Rgba {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let value = self.to_u32();

		write!(f, "#{value:08X}")
	}
}

impl From<(u8, u8, u8, u8)> for Rgba {
	#[inline(always)]
	fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
		Self::new(r, g, b, a)
	}
}

impl FromStr for Rgba {
	type Err = RgbaFromStrError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if !s.starts_with('#') {
			return Err(RgbaFromStrError::MissingHash);
		}

		let get_int_in_range = |range: RangeInclusive<usize>| -> Result<u8, Self::Err> {
			let value = s.get(range).map(|s| u8::from_str_radix(s, 0x10));

			if let Some(Ok(value)) = value {
				Ok(value)
			} else {
				Err(RgbaFromStrError::UnknownFormat)
			}
		};

		let colour = match s.len() {
			0x4 => {
				let r = get_int_in_range(0x1..=0x1)? * 0x11;
				let g = get_int_in_range(0x2..=0x2)? * 0x11;
				let b = get_int_in_range(0x3..=0x3)? * 0x11;

				Self::new(r, g, b, 0xFF)
			}

			0x7 => {
				let r = get_int_in_range(0x1..=0x2)?;
				let g = get_int_in_range(0x3..=0x4)?;
				let b = get_int_in_range(0x5..=0x6)?;

				Self::new(r, g, b, 0xFF)
			}

			0x9 => {
				let r = get_int_in_range(0x1..=0x2)?;
				let g = get_int_in_range(0x3..=0x4)?;
				let b = get_int_in_range(0x5..=0x6)?;
				let a = get_int_in_range(0x7..=0x8)?;

				Self::new(r, g, b, a)
			}

			len => return Err(RgbaFromStrError::InvalidLength(len)),
		};

		Ok(colour)
	}
}
