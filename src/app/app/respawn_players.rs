// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::app::App;
use crate::error::{Error, Result};
use crate::log::log;

impl App {
	pub(super) fn respawn_players(&mut self) -> Result<()> {
		log!("respawning `{}` player(s)", self.players.len());

		let spawn_chunks: Vec<_> = self
			.level
			.chunks
			.iter()
			.enumerate()
			.filter(|&(_, c)| c.is_spawnable)
			.map(|(i, _)| i)
			.collect();

		log!(
			note,
			"there are `{}` spawn chunk(s) in level \"{}\"",
			spawn_chunks.len(),
			self.level.name,
		);

		if spawn_chunks.is_empty() {
			return Err(Error::MissingSpawnChunk);
		}

		// TODO

		for player in &self.players {
			log!("spawning player \"{}\"", player.name);
		}

		Ok(())
	}
}
