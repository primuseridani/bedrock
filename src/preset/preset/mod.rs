// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::error::DecodeError;
use crate::map::MapSize;

use oct::decode::Decode;
use oct::encode::{Encode, SizedEncode};

#[derive(Clone, Debug, Decode, Encode, SizedEncode)]
#[oct(decode_error = DecodeError)]
pub struct Preset {
	pub map_size: MapSize,
	pub tps:      u16,

	pub friendly_fire: bool,
}

// FIXME: `syn` does not parse default field val-
// ues.
impl Default for Preset {
	#[inline(always)]
	fn default() -> Self {
		Self {
			map_size: Default::default(),
			tps:      0x8,

			friendly_fire: true,
		}
	}
}
