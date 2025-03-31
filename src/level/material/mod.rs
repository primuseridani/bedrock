// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::MaterialFromStrError;

use std::mem::transmute;
use std::str::FromStr;

#[allow(unused)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Material {
	#[default]
	Air = 0x0,

	Bedrock,
	Rock,
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
}

impl Material {
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
			"rock"     => Ok(Self::Rock),
			"water"     => Ok(Self::Water),

			_ => Err(MaterialFromStrError { name: s.into() })
		}
	}
}
