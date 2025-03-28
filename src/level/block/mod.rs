// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::{BlockTags, Material};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Block(u8);

impl Block {
	#[inline]
	pub const fn set_material(&mut self, material: Material) {
		let material = material as u8 & 0b00111111;

		let mut value = self.0;

		value &= 0b11000000;
		value |= material;

		self.0 = value;
	}

	#[inline]
	pub const fn set_seed(&mut self, seed: u8) {
		let mut seed = seed & 0b00000011;

		seed <<= 0x6;

		let mut value = self.0;

		value &= 0b00111111;
		value |= seed;

		self.0 = value;
	}

	#[inline]
	#[must_use]
	pub const fn material(self) -> Material {
		let material = self.0 & 0b00111111;

		// SAFETY: We have applied an appropriate mask.
		// These bits also only come from a previously-ex-
		// isting `Material` object.
		//
		// Also note that `0x0` will is defined as `Air`,
		// which makes `<Self as Default>::default` sound.
		unsafe { Material::new_unchecked(material) }
	}

	#[inline]
	#[must_use]
	pub const fn seed(self) -> u8 {
		let mut seed = self.0 & 0b11000000;

		seed >>= 0x6;

		seed
	}

	#[inline]
	#[must_use]
	pub const fn tags(self) -> BlockTags {
		match self.material() {
			Material::Air       => BlockTags::EMPTY.union(BlockTags::STATIC),
			Material::Basalt    => BlockTags::STATIC,
			Material::Bedrock   => BlockTags::DIVINE.union(BlockTags::STATIC),
			Material::Clay      => BlockTags::NONE,
			Material::Dirt      => BlockTags::NONE,
			Material::Granite   => BlockTags::STATIC,
			Material::Grass     => BlockTags::NONE,
			Material::Gravel    => BlockTags::NONE,
			Material::Limestone => BlockTags::STATIC,
			Material::Magma     => BlockTags::HOT.union(BlockTags::LIQUID),
			Material::Marble    => BlockTags::STATIC,
			Material::Sand      => BlockTags::NONE,
			Material::Rock     => BlockTags::STATIC,
			Material::Water     => BlockTags::LIQUID,
			Material::Ice       => BlockTags::COLD.union(BlockTags::STICKY),
			Material::Wood      => BlockTags::STICKY,
			Material::Glass     => BlockTags::STICKY,
			Material::Fire      => BlockTags::HOT,
		}
	}
}

macro_rules! def_is {
	{ $($name:ident: $tag:ident),*$(,)? } => {
		impl ::bedrock::level::Block {$(
			#[allow(dead_code)]
			#[inline(always)]
			#[must_use]
			pub const fn $name(self) -> bool {
				self.tags().contains(::bedrock::level::BlockTags::$tag)
			}
		)*}
	};
}

def_is! {
	is_none:     NONE,
	is_static:   STATIC,
	is_liquid:   LIQUID,
	is_hot:      HOT,
	is_cold:     COLD,
	is_emtpy:    EMPTY,
	is_divine:   DIVINE,
	is_sticky:   STICKY,
	is_bernie:   BERNIE,
	is_volatile: VOLATILE,
	is_any:      ALL,
}
