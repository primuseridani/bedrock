// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::{BlockTags, Material};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Block(u8);

impl Block {
	#[inline]
	#[must_use]
	pub const fn new(material: Material, seed: u8) -> Self {
		let     material = material as u8 & 0b00111111;
		let mut seed     = seed           & 0b00000011;

		seed <<= 0x6;

		let value = material | seed;
		Self(value)
	}

	#[inline]
	pub const fn set_material(&mut self, material: Material) {
		let material = material as u8 & 0b00111111;

		let mut value = self.0;

		value &= 0b11000000;
		value |= material;

		self.0 = value;
	}

	#[inline]
	#[must_use]
	pub const fn material(self) -> Material {
		let material = self.0 & 0b00111111;

		// SAFETY: We have applied an appropriate mask.
		// These bits also only come from a previously-ex-
		// isting `Material` object.
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
	pub fn tags(self) -> BlockTags {
		match self.material() {
			Material::Air       => BlockTags::EMPTY | BlockTags::STATIC,
			Material::Basalt    => BlockTags::STATIC,
			Material::Bedrock   => BlockTags::GOD | BlockTags::STATIC,
			Material::Clay      => BlockTags::NONE,
			Material::Dirt      => BlockTags::NONE,
			Material::Granite   => BlockTags::STATIC,
			Material::Grass     => BlockTags::NONE,
			Material::Gravel    => BlockTags::NONE,
			Material::Limestone => BlockTags::STATIC,
			Material::Magma     => BlockTags::HOT | BlockTags::LIQUID,
			Material::Marble    => BlockTags::STATIC,
			Material::Sand      => BlockTags::NONE,
			Material::Stone     => BlockTags::STATIC,
			Material::Water     => BlockTags::LIQUID,
			Material::Ice       => BlockTags::COLD | BlockTags::STICKY,
			Material::Wood      => BlockTags::STICKY,
			Material::Glass     => BlockTags::STICKY,
			Material::Fire      => BlockTags::HOT,
		}
	}
}
