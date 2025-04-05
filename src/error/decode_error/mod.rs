// Copyright 2025 Gabriel Bjørnager Jensen.

use oct::error::GenericDecodeError;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecodeError(Box<str>);

impl DecodeError {
	#[inline]
	#[must_use]
	pub fn new<S: Display>(message: S) -> Self {
		Self(message.to_string().into())
	}
}

impl Display for DecodeError {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "decode error: {}", self.0)
	}
}

impl std::error::Error for DecodeError { }

impl<T: Into<GenericDecodeError>> From<T> for DecodeError{
	#[inline(always)]
	fn from(value: T) -> Self {
		Self::new(value.into())
	}
}
