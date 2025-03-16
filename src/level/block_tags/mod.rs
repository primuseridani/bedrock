// Copyright 2025 Gabriel Bjørnager Jensen.

use std::ops::{BitAnd, BitOr, BitOrAssign};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BlockTags(u32);

impl BlockTags {
	/// The block has no tags.
	pub const NONE:   Self = Self(0b00000000_00000000_00000000_00000000);

	/// The block is not affected by gravity.
	pub const STATIC: Self = Self(0b00000000_00000000_00000000_00000001);

	/// The block is a liquid.
	pub const LIQUID: Self = Self(0b00000000_00000000_00000000_00000010);

	/// The block is hot.
	///
	/// Some blocks may be affected by being in the vicinity of "hot" blocks.
	pub const HOT:    Self = Self(0b00000000_00000000_00000000_00000100);

	/// The block is cold.
	///
	/// Some blocks may be affected by being in the vicinity of "cold" blocks.
	pub const COLD:   Self = Self(0b00000000_00000000_00000000_00001000);

	/// The block does not affect collisions.
	pub const EMPTY:  Self = Self(0b00000000_00000000_00000000_00010000);

	/// The block cannot be destroyed.
	pub const GOD:    Self = Self(0b00000000_00000000_00000000_00100000);

	/// The block is partially affected by gravity.
	///
	/// The behaviour of sticky blocks overlap with that of statics and ordinary blocks; if a sticky block is physically connected to another block of the same material, then the two are not affected by gravity.
	pub const STICKY: Self = Self(0b00000000_00000000_00000000_01000000);

	/// The block is burnt by hot blocks.
	pub const BERNIE: Self = Self(0b00000000_00000000_00000000_10000000);

}

macro_rules! def_is {
	{ $($name:ident: $tag:ident),*$(,)? } => {
		impl $crate::level::BlockTags {$(
			#[inline(always)]
			#[must_use]
			pub const fn $name(self) -> bool {
				self.0 & Self::$tag.0 == 0x0
			}
		)*}
	};
}

def_is! {
	is_static: STATIC,
	is_liquid: LIQUID,
	is_hot:    HOT,
	is_cold:   COLD,
}

impl BitAnd for BlockTags {
	type Output = bool;

	#[inline(always)]
	fn bitand(self, rhs: Self) -> Self::Output {
		self.0 & rhs.0 == 0x0
	}
}

impl BitOr for BlockTags {
	type Output = Self;

	#[inline(always)]
	fn bitor(self, rhs: Self) -> Self::Output {
		let value = self.0 | rhs.0;
		Self(value)
	}
}

impl BitOrAssign for BlockTags {
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: Self) {
		self.0 |= rhs.0
	}
}
