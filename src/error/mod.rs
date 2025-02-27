// Copyright 2025 Gabriel Bjørnager Jensen.

mod error;

pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
