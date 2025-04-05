// Copyright 2025 Gabriel Bjørnager Jensen.

mod message;

pub use message::Message;

use conststr::String;

pub type LobbyName     = String<0x10>;
pub type LobbyPassword = String<0x40>;
pub type Username      = String<0x10>;
pub type ChatMessage   = String<0x40>;
