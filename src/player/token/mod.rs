// Copyright 2025 Gabriel Bjørnager Jensen.

use oct::decode::Decode;
use oct::encode::{Encode, SizedEncode};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Decode, Default, Encode, Eq, PartialEq, SizedEncode)]
pub enum Token {
	#[default]
	Pawn,

	Bicycle,
	Train,
	Boot,
	Dog,
	Tank,
	Cannon,
	Duck,
	Wastebasket,
}
