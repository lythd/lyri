/*
This file is from https://git.nia.land/nia/lyri
Copyright (C) 2025 Nia

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

//! This module handles connecting with media players on dbus by using mpris with the MediaInterface struct
//! The most important functions here are ::new(), and ::find_media()

use mpris::Player;
use mpris::PlayerFinder;

/// Manages interfacing with playing media via mpris
pub struct MediaInterface {
	/// The mpris player finder
	finder: PlayerFinder,
}

impl MediaInterface {
	/// Constructor method, fails if PlayerFinder::new() returns err
	pub fn new() -> Option<Self> {
		let Ok(finder) = PlayerFinder::new() else { return None };
		Some(Self { finder })
	}

	/// Finds the active player
	pub fn player(&mut self) -> Option<Player> {
		let Ok(player) = self.finder.find_active() else {
			return None;
		};
		Some(player)
	}

	/// Return song url + name if found
	pub fn find_media(&mut self) -> (Option<String>, Option<String>) {
		let player = self.player();

		// no player = no song
		if player.is_none() {
			println!("No player :c");
			return None;
		}

		let player = player.unwrap();
		let Ok(meta) = player.get_metadata() else {
			println!("No metadata :c");
			return None;
		};
		(meta.url(), meta.name())
	}
}
