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
	/// The song url
	url: Option<String>,
	/// The song name
	name: Option<String>,
	/// The song's image url
	image_url: Option<String>,
	/// Whether there is a song and it is playing
	playing: bool,
}

impl MediaInterface {
	/// Constructor method
	pub fn new() -> Self {
		let finder = PlayerFinder::new().expect("Could not create PlayerFinder");
		Self {
			finder,
			url: None,
			name: None,
			image_url: None,
			playing: false,
		}
	}

	/// Finds the active player if one is active
	pub fn player(&mut self) -> Option<Player> {
		let Ok(player) = self.finder.find_active() else {
			return None;
		};
		Some(player)
	}

	/// Updates media, should be called periodically to ensure fields are accurate
	pub fn update_media(&mut self) {
		let player = self.player();

		// no player = no song
		if player.is_none() {
			self.url = None;
			self.name = None;
			self.playing = false;
			return;
		}

		// no metadata
		let player = player.unwrap();
		self.playing = player.is_running();
		let Ok(meta) = player.get_metadata() else {
			self.url = None;
			self.name = None;
			return;
		};

		self.url = match meta.url() {
			Some(s) => Some(s.to_string()),
			None => None,
		};
		self.name = match meta.title() {
			Some(s) => Some(s.to_string()),
			None => None,
		};
		self.image_url = match meta.art_url() {
			Some(s) => Some(s.to_string()),
			None => None,
		};
	}

	pub fn current_song_info(&self) -> (Option<&str>, Option<&str>) {
		(self.name(), self.image_url())
	}

	pub fn name(&self) -> Option<&str> {
		match &self.name {
			Some(s) => Some(s),
			None => None,
		}
	}

	pub fn url(&self) -> Option<&str> {
		match &self.url {
			Some(s) => Some(s),
			None => None,
		}
	}

	pub fn image_url(&self) -> Option<&str> {
		match &self.image_url {
			Some(s) => Some(s),
			None => None,
		}
	}

	pub fn get_lyrics(&self) -> Option<(String, String)> {
		// TODO!!!!
		Some((String::from("Original..."), String::from("Translated...")))
	}

	pub fn is_playing(&self) -> bool {
		self.playing
	}

	pub fn play(&mut self) {
		// nothing to do if there is no player
		let Some(player) = self.player() else {
			return;
		};

		let _ = player.play();
	}

	pub fn pause(&mut self) {
		// nothing to do if there is no player
		let Some(player) = self.player() else {
			return;
		};

		let _ = player.pause();
	}

	pub fn next(&mut self) {
		// nothing to do if there is no player
		let Some(player) = self.player() else {
			return;
		};

		let _ = player.next();
	}

	pub fn prev(&mut self) {
		// nothing to do if there is no player
		let Some(player) = self.player() else {
			return;
		};

		let _ = player.previous();
	}
}
