// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::MaterialFromStrError;

use std::mem::transmute;
use std::str::FromStr;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Material {
	#[default]
	Air,

	Bedrock,
	Stone,
	Dirt,
	Sand,
	Water,
	Granite,
	Magma, // Or `Lava`?
	Basalt,
	Clay,
	Gravel,
	Marble,
	Limestone,
	Grass,
	Ice,
	Wood,
	Glass,
	Fire,

	// SAFETY: Remember to update `Self::new`.
}

impl Material {
	pub const fn new(value: u8) -> Option<Self> {
		if value > Self::Fire as u8 { return None };

		// SAFETY: We have tested bounds.
		let this = unsafe { Self::new_unchecked(value) };
		Some(this)
	}

	pub const unsafe fn new_unchecked(value: u8) -> Self {
		// SAFETY: Caller guarantees bounds.
		unsafe { transmute::<u8, Self>(value) }
	}
}

impl FromStr for Material {
	type Err = MaterialFromStrError;

	#[inline]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"air"       => Ok(Self::Air),
			"basalt"    => Ok(Self::Basalt),
			"bedrock"   => Ok(Self::Bedrock),
			"clay"      => Ok(Self::Clay),
			"dirt"      => Ok(Self::Dirt),
			"granite"   => Ok(Self::Granite),
			"grass"     => Ok(Self::Grass),
			"gravel"    => Ok(Self::Gravel),
			"limestone" => Ok(Self::Limestone),
			"magma"     => Ok(Self::Magma),
			"marble"    => Ok(Self::Marble),
			"sand"      => Ok(Self::Sand),
			"stone"     => Ok(Self::Stone),
			"water"     => Ok(Self::Water),

			_ => Err(MaterialFromStrError { name: s.into() })
		}
	}
}
