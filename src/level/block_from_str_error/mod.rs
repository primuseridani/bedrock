// Copyright 2025 Gabriel Bjørnager Jensen.

use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub struct BlockFromStrError {
	pub name: Box<str>,
}

impl std::error::Error for BlockFromStrError { }

impl Display for BlockFromStrError {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "unknown block \"{}\"", self.name)
	}
}
