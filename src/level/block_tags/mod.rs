// Copyright 2025 Gabriel Bjørnager Jensen.

use std::fmt::{self, Debug, Formatter};
use std::ops::{
	BitAnd,
	BitAndAssign,
	BitOr,
	BitOrAssign,
	BitXor,
	BitXorAssign,
	Not,
	Sub,
	SubAssign,
};

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct BlockTags(u32);

impl BlockTags {
	/// The block has no tags.
	pub const NONE:     Self = Self(0b00000000_00000000_00000000_00000000);

	/// The block is not affected by gravity.
	pub const STATIC:   Self = Self(0b00000000_00000000_00000000_00000001);

	/// The block is a liquid.
	pub const LIQUID:   Self = Self(0b00000000_00000000_00000000_00000010);

	/// The block is hot.
	///
	/// Some blocks may be affected by being in the vicinity of "hot" blocks.
	pub const HOT:      Self = Self(0b00000000_00000000_00000000_00000100);

	/// The block is cold.
	///
	/// Some blocks may be affected by being in the vicinity of "cold" blocks.
	pub const COLD:     Self = Self(0b00000000_00000000_00000000_00001000);

	/// The block does not affect collisions.
	pub const EMPTY:    Self = Self(0b00000000_00000000_00000000_00010000);

	/// The block cannot be destroyed.
	pub const DIVINE:   Self = Self(0b00000000_00000000_00000000_00100000);

	/// The block is partially affected by gravity.
	///
	/// The behaviour of sticky blocks overlap with that of statics and ordinary blocks; if a sticky block is physically connected to another block of the same material, then the two are not affected by gravity.
	pub const STICKY:   Self = Self(0b00000000_00000000_00000000_01000000);

	/// The block is burnt by hot blocks.
	pub const BERNIE:   Self = Self(0b00000000_00000000_00000000_10000000);

	/// The block is evaporated by hot blocks.
	pub const VOLATILE: Self = Self(0b00000000_00000000_00000001_00000000);

	/// The block contains all tags.
	pub const ALL:      Self = Self(0b11111111_11111111_11111111_11111111);
}

impl BlockTags {
	#[inline(always)]
	#[must_use]
	pub const fn union(self, other: Self) -> Self {
		let value = self.0 | other.0;
		Self(value)
	}

	#[inline(always)]
	#[must_use]
	pub const fn intersection(self, other: Self) -> Self {
		let value = self.0 & other.0;
		Self(value)
	}

	#[inline(always)]
	#[must_use]
	pub const fn difference(self, other: Self) -> Self {
		let value = self.0.wrapping_sub(other.0);
		Self(value)
	}

	#[inline(always)]
	#[must_use]
	pub const fn symmetric_difference(self, other: Self) -> Self {
		let value = self.0 ^ other.0;
		Self(value)
	}

	#[inline(always)]
	#[must_use]
	pub const fn invert(self) -> Self {
		let value = !self.0;
		Self(value)
	}

	#[inline(always)]
	#[must_use]
	pub const fn contains(self, other: Self) -> bool {
		self.0 | other.0 == self.0
	}
}

impl BitAnd for BlockTags {
	type Output = Self;

	#[inline(always)]
	fn bitand(self, rhs: Self) -> Self::Output {
		self.intersection(rhs)
	}
}

impl BitAndAssign for BlockTags {
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: Self) {
		*self = *self & rhs
	}
}

impl BitOr for BlockTags {
	type Output = Self;

	#[inline(always)]
	fn bitor(self, rhs: Self) -> Self::Output {
		self.union(rhs)
	}
}

impl BitOrAssign for BlockTags {
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs
	}
}

impl BitXor for BlockTags {
	type Output = Self;

	#[inline(always)]
	fn bitxor(self, rhs: Self) -> Self::Output {
		self.symmetric_difference(rhs)
	}
}

impl BitXorAssign for BlockTags {
	#[inline(always)]
	fn bitxor_assign(&mut self, rhs: Self) {
		*self = *self ^ rhs;
	}
}

impl Debug for BlockTags {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Block({:#034b})", self.0)
	}
}

impl Not for BlockTags {
	type Output = Self;

	#[inline(always)]
	fn not(self) -> Self::Output {
		self.invert()
	}
}

impl Sub for BlockTags {
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output {
		self.difference(rhs)
	}
}

impl SubAssign for BlockTags {
	#[inline(always)]
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs;
	}
}
