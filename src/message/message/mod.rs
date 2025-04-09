// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::error::DecodeError;
use crate::message::{ChatMessage, LobbyPassword, Username};
use crate::player::{Team, Token};
use crate::preset::Preset;

use oct::decode::Decode;
use oct::encode::{Encode, SizedEncode};

#[derive(Debug, Decode, Encode, SizedEncode)]
#[oct(decode_error = DecodeError)]
pub enum Message {
	Join {
		username: Username,
		password: LobbyPassword,
	},

	Chat(ChatMessage),

	UpdatePreset(Preset),

	Quit,

	Kick,

	Start,

	ChangeTeam(Team),

	ChangeToken(Token),
}
