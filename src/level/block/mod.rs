// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::BlockFromStrError;

use std::str::FromStr;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Block {
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
}

impl FromStr for Block {
	type Err = BlockFromStrError;

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

			_ => Err(BlockFromStrError { name: s.into() })
		}
	}
}
