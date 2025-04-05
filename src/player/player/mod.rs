// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::player::{Connexion, Team, Token};

use std::borrow::Cow;

#[derive(Debug, Default)]
pub struct Player {
	pub name:  Cow<'static, str> = Cow::Borrowed("epsiloneridani"),
	pub token: Token,
	pub team:  Team,

	pub is_admin: bool,

	pub connexion: Connexion,
}
