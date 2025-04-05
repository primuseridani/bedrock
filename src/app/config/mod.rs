// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::message::{LobbyName, LobbyPassword};

use std::net::SocketAddr;

#[derive(Clone, Debug, Default)]
pub struct Config {
	pub addr:     Option<SocketAddr>,
	pub name:     LobbyName,
	pub password: LobbyPassword,
}
