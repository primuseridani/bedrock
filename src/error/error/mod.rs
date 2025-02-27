// Copyright 2025 Gabriel Bjørnager Jensen.

use std::fmt::{self, Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	MissingDataDir,

	UnknownLevel {
		name: Box<str>,
	},
}

impl Display for Error {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::MissingDataDir
			=> write!(f, "could not find data directory"),

			Self::UnknownLevel { ref name }
			=> write!(f, "unknown level \"{name}\""),
		}
	}
}

impl From<Error> for i32 {
	#[inline]
	fn from(value: Error) -> Self {
		match value {
			| Error::MissingDataDir
			| Error::UnknownLevel { .. }
			=> 0x2,
		}
	}
}
