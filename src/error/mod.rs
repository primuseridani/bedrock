// Copyright 2025 Gabriel Bjørnager Jensen.

mod decode_error;
mod error;

pub use decode_error::DecodeError;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
