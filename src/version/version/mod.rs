// Copyright 2025 Gabriel Bjørnager Jensen.

use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Version {
	pub major: u32,
	pub minor: u32,
	pub patch: u32,
	pub pre:   Option<u32>,
}

impl Version {
	pub const CURRENT: Self = Self {
		major: 0x0,
		minor: 0x5,
		patch: 0x0,
		pre:   Some(0x5),
	};
}

impl Display for Version {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;

		if let Some(pre) = self.pre {
			write!(f, "-{pre}")?;
		}

		Ok(())
	}
}
