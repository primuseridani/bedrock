// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::player::Controls;

use std::borrow::Cow;

#[derive(Clone, Debug, Default)]
pub struct Player {
	pub name: Cow<'static, str> = Cow::Borrowed("epsiloneridani"),

	pub controls: Controls,
}
