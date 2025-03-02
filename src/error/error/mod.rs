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
				let field = if let Some(ref section) = *section {
					Cow::Owned(section.to_string() + field)
				} else {
					Cow::Borrowed(&**field)
				};

				write!(f, "invalid level field `{field}`: {source}")
			}

			Self::MissingDataDir
			=> write!(f, "could not find data directory"),

			Self::UnknownLevel { ref path, ref source }
			=> write!(f, "unable to load level at \"{}\": {source}", path.display()),
		}
	}
}

impl std::error::Error for Error {
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
			| Error::InvalidLevel { .. }
			| Error::MissingDataDir
			| Error::UnknownLevel { .. }
			=> 0x2,
		}
	}
}
