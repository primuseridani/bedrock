// Copyright 2025 Gabriel Bjørnager Jensen.

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Block {
	#[default]
	Air,

	Bedrock,
	Stone,
	Dirt,
	Sand,
}
