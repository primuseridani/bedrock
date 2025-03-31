// Copyright 2025 Gabriel Bjørnager Jensen.

use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};
use std::path::Path;

#[derive(Debug)]
pub enum Error {
	InvalidLevel {
		section: Option<Box<str>>,
		field:   Box<str>,
		source:  Box<dyn std::error::Error>,
	},

	MissingDataDir,

	MissingSpawnChunk,

	UnknownCliArg(Box<str>),

	UnknownLevel {
		path:   Box<Path>,
		source: Box<dyn std::error::Error>,
	},
}

impl Display for Error {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::InvalidLevel { ref section, ref field, ref source } => {
				let field = section.as_ref().map_or_else(
					||  Cow::Borrowed(&**field),
					|s| Cow::Owned(s.to_string() + field),
				);

				write!(f, "invalid level field `{field}`: {source}")
			}

			Self::MissingDataDir
			=> write!(f, "could not find data directory"),

			Self::MissingSpawnChunk
			=> write!(f, "there are no spawn chunks in the level"),

			Self::UnknownCliArg(ref arg)
			=> write!(f, "unknown command line interface \"{arg}\""),

			Self::UnknownLevel { ref path, ref source }
			=> write!(f, "unable to load level at \"{}\": {source}", path.display()),
		}
	}
}

impl std::error::Error for Error {
	#[allow(clippy::match_same_arms)]
	#[inline]
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match *self {
			Self::InvalidLevel { ref source, .. }
			=> Some(&**source),

			Self::UnknownLevel { ref source, .. }
			=> Some(&**source),

			_ => None,
		}
	}
}

impl From<Error> for i32 {
	#[inline]
	fn from(value: Error) -> Self {
		match value {
			| Error::MissingDataDir
			| Error::UnknownCliArg(_)
			| Error::UnknownLevel { .. }
			=> 0x2,

			| Error::InvalidLevel { .. }
			| Error::MissingSpawnChunk
			=> 0x3,
		}
	}
}
