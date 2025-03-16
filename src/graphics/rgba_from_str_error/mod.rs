// Copyright 2025 Gabriel Bjørnager Jensen.

use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum RgbaFromStrError {
	InvalidLength(usize),

	MissingHash,

	UnknownFormat,
}

impl Error for RgbaFromStrError { }

impl Display for RgbaFromStrError {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::InvalidLength(len)
			=> write!(f, "rgba code has length `{len}` but should have been neither `4` or `7` or `9` octets long"),

			Self::MissingHash
			=> write!(f, "rgba code is missing prefixed hash `#`"),

			Self::UnknownFormat
			=> write!(f, "rgba code is of an otherwise unknown format"),
		}
	}
}
