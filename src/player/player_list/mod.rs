// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::log::log;
use crate::message::Message;
use crate::player::{Connexion, Player};

use std::io::{self, Write};
use std::slice;
use std::vec;

#[derive(Debug, Default)]
pub struct PlayerList(Vec<Player>);

impl PlayerList {
	#[inline(always)]
	#[track_caller]
	pub fn clear(&mut self) {
		self.0.clear()
	}

	#[inline(always)]
	#[track_caller]
	pub fn insert(&mut self, player: Player) -> bool {
		if self.iter().any(|p| p.name == player.name) {
			return true;
		}

		self.0.push(player);

		false
	}

	pub fn remove(&mut self, name: &str) -> Option<Player> {
		let index = self.iter().position(|p| p.name == name)?;

		let player = self.0.remove(index);
		Some(player)
	}

	pub fn send_message_to_all(&mut self, message: &Message) -> io::Result<()> {
		log!(debug, "sending message `{message:?}` to players");

		for player in self {
			if let Connexion::Remote { ref mut stream, ref mut buf } = player.connexion {
				buf.write(message).unwrap();

				if let Err(e) = stream.write(buf) {
					let addr = stream.peer_addr().unwrap();
					log!("unable to send message to `{addr}`: {e}");
				}
			}
		}

		Ok(())
	}

	#[allow(unused)]
	#[inline(always)]
	#[must_use]
	pub const fn len(&self) -> usize {
		self.0.len()
	}

	#[allow(unused)]
	#[inline(always)]
	#[must_use]
	pub const fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	#[allow(unused)]
	#[inline(always)]
	pub fn iter(&self) -> slice::Iter<Player> {
		self.0.iter()
	}

	#[allow(unused)]
	#[inline(always)]
	pub fn iter_mut(&mut self) -> slice::IterMut<Player> {
		self.0.iter_mut()
	}
}

impl IntoIterator for PlayerList {
	type Item = Player;

	type IntoIter = vec::IntoIter<Player>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

impl<'a> IntoIterator for &'a PlayerList {
	type Item = &'a Player;

	type IntoIter = slice::Iter<'a, Player>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a> IntoIterator for &'a mut PlayerList {
	type Item = &'a mut Player;

	type IntoIter = slice::IterMut<'a, Player>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter_mut()
	}
}
