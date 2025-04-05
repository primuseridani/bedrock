// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::message::Message;

use oct::slot::Slot;
use std::net::TcpStream;

#[derive(Debug, Default)]
pub enum Connexion {
	#[default]
	Local,

	Remote {
		stream: TcpStream,
		buf:    Slot<Message>,
	},
}
