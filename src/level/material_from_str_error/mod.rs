// Copyright 2025 Gabriel Bjørnager Jensen.

use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub struct MaterialFromStrError {
	pub name: Box<str>,
}

impl std::error::Error for MaterialFromStrError { }

impl Display for MaterialFromStrError {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "unknown block \"{}\"", self.name)
	}
}
