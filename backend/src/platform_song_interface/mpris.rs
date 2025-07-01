use crate::models::{PlaybackState, SongInfo};
use crate::platform_song_interface::PlatformSongInterface;
use mpris::{PlaybackStatus, PlayerFinder};
use std::error::Error;
use std::sync::{Arc, Mutex};


pub struct MprisRetriever {
    active_player_identity: Arc<Mutex<String>>,
}
impl MprisRetriever {
    pub fn new() -> Result<MprisRetriever, Box<dyn Error>> {
        Ok(MprisRetriever { active_player_identity: Arc::new(Mutex::new(String::default())) })
    }
}

impl PlatformSongInterface for MprisRetriever {

    fn get_active_song(&self) -> Option<SongInfo> {
        let active_player_identity = Arc::clone(&self.active_player_identity);
        let finder = PlayerFinder::new().unwrap();

        if let Ok(player) = finder.find_active() {
            if let Ok(metadata) = player.get_metadata() {
                let active_song = SongInfo::from_mpris_metadata(metadata);

                let mut player_identity = active_player_identity.lock().unwrap();
                if *player_identity != player.bus_name() {
                    *player_identity = player.identity().to_string();
                }
                
                return Some(active_song);
                
            }
        }
        return None;
    }

    fn get_playback_duration(&self) -> Option<PlaybackState> {
        let active_player_identity = Arc::clone(&self.active_player_identity);
        
        let player_identity = {
            let player_identity_guard = active_player_identity.lock().unwrap();
            (*player_identity_guard).clone()
        };

        if let Ok(finder) = PlayerFinder::new() {
            if let Ok(player) = finder.find_by_name(player_identity.as_str()) {
                let positon = player.get_position().unwrap();
                return match player.get_playback_status().unwrap() {
                    PlaybackStatus::Playing => Some(PlaybackState::Playing(positon)),
                    PlaybackStatus::Paused => Some(PlaybackState::Paused(positon)),
                    _ => None,
                }
            }
        }
        None
    }

    fn play(&self) -> bool {
        let player_identity = {
            let clone = Arc::clone(&self.active_player_identity);
            let player_identity_guard = clone.lock().unwrap();
            (*player_identity_guard).clone()
        };
        if let Ok(finder) = PlayerFinder::new() {
            if let Ok(player) = finder.find_by_name((player_identity.as_str())) {
                if let Ok(()) = player.play() {
                     return true;
                }
            }
        } 
        false
    }

    fn pause(&self) -> bool {
        let player_identity = {
            let clone = Arc::clone(&self.active_player_identity);
            let player_identity_guard = clone.lock().unwrap();
            (*player_identity_guard).clone()
        };
        if let Ok(finder) = PlayerFinder::new() {
            if let Ok(player) = finder.find_by_name((player_identity.as_str())) {
                if let Ok(()) = player.pause() {
                    return true;
                }
            }
        }
        false
    }

    fn next(&self) -> bool {
        let player_identity = {
            let clone = Arc::clone(&self.active_player_identity);
            let player_identity_guard = clone.lock().unwrap();
            (*player_identity_guard).clone()
        };
        if let Ok(finder) = PlayerFinder::new() {
            if let Ok(player) = finder.find_by_name((player_identity.as_str())) {
                if let Ok(()) = player.next() {
                    return true;
                }
            }
        }
        false
    }

    fn prev(&self) -> bool {
        let player_identity = {
            let clone = Arc::clone(&self.active_player_identity);
            let player_identity_guard = clone.lock().unwrap();
            (*player_identity_guard).clone()
        };
        if let Ok(finder) = PlayerFinder::new() {
            if let Ok(player) = finder.find_by_name((player_identity.as_str())) {
                if let Ok(()) = player.previous() {
                    return true;
                }
            }
        }
        false
    }
}