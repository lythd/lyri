use std::sync::Arc;

use backend::{models::PlaybackState, platform_song_interface::PlatformSongInterface};

pub async fn prev_button_action(song_interface: Arc<Box<dyn PlatformSongInterface>>) {
	if !song_interface.prev() {
		eprintln!("Failed to go to previous track");
	}
}

pub async fn play_button_action(song_interface: Arc<Box<dyn PlatformSongInterface>>) -> bool {
	if let Some(PlaybackState::Playing(_)) = song_interface.get_playback_duration() {
		song_interface.pause();
	} else {
		song_interface.play();
	}
	// have to have a seperate one in case pause/play fails
	if let Some(PlaybackState::Playing(_)) = song_interface.get_playback_duration() {
		true
	} else {
		false
	}
}

pub async fn next_button_action(song_interface: Arc<Box<dyn PlatformSongInterface>>) {
	if !song_interface.next() {
		eprintln!("Failed to go to next track");
	}
}
