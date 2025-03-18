// Copyright 2025 Gabriel Bjørnager Jensen.

mod load_builtin;

use crate::graphics::Rgba;
use crate::level::Chunk;

use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct Level {
	pub name:        Cow<'static, str>,
	pub creatour:    Cow<'static, str>,
	pub description: Cow<'static, str>,

	pub background: Rgba,

	pub chunks: Cow<'static, [Chunk]>,
}

impl Default for Level {
	#[inline(always)]
	fn default() -> Self {
		Self::LAKE
	}
}
