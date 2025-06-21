use crate::models::{PlaybackState, SongInfo};
use crate::song_info_retrieval::SongInfoRetriever;
use mpris::{PlaybackStatus, PlayerFinder};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

pub struct MprisRetriever {
    active_player_identity: Arc<Mutex<String>>,
}
impl MprisRetriever {
    pub fn new() -> Result<MprisRetriever, Box<dyn Error>> {
        Ok(MprisRetriever { active_player_identity: Arc::new(Mutex::new(String::default())) })
    }
}

impl SongInfoRetriever for MprisRetriever {
    fn watch_active_song(&self, sender: mpsc::Sender<SongInfo>) -> Result<(), String> {
        let active_player_identity = Arc::clone(&self.active_player_identity);

        tokio::spawn(async move {
            let mut last_active_song = SongInfo::default();
            let active_player_identity = Arc::clone(&active_player_identity);

            loop {

                sleep(Duration::from_millis(500)).await;

                let result: Option<SongInfo> = async {
                    let finder = PlayerFinder::new().unwrap();

                    if let Ok(player) = finder.find_active() {
                        if let Ok(metadata) = player.get_metadata() {
                            let active_song = SongInfo::from_mpris_metadata(metadata);
                            
                            let mut player_identity = active_player_identity.lock().unwrap();
                            if *player_identity != player.bus_name() {
                                *player_identity = player.identity().to_string();
                            }

                            if active_song != last_active_song {
                                last_active_song = active_song.clone();
                                return Some(active_song);
                            }
                        }                        
                    }
                    return None;
                }.await;

                match result {
                    Some(t) => sender.send(t).await.unwrap(),
                    None => {}
                }
            }
        });
        Ok(())
    }
    fn watch_duration(&self, sender: mpsc::Sender<PlaybackState>) -> Result<(), String> {
        let active_player_identity = Arc::clone(&self.active_player_identity);

        tokio::spawn(async move {
            let mut was_last_paused = true;
            let active_player_identity = Arc::clone(&active_player_identity);
            
            loop {
                sleep(Duration::from_millis(500)).await;
                let result: Option<PlaybackState> = async {
                    let bus_name = {
                        let bus_name_guard = active_player_identity.lock().unwrap();
                        (*bus_name_guard).clone()
                    };
                    
                    if let Ok(finder) = PlayerFinder::new() {
                        if let Ok(player) = finder.find_by_name(bus_name.as_str()) {
                            let positon = player.get_position().unwrap();
                            return match player.get_playback_status().unwrap() {
                                PlaybackStatus::Playing => {
                                    was_last_paused = false;
                                    Some(PlaybackState::Playing(positon))
                                },
                                PlaybackStatus::Paused if !was_last_paused => {
                                    was_last_paused = true;
                                    Some(PlaybackState::Paused(positon))
                                },
                                _ => None,
                            }
                        }
                    }                               
                    None
                }.await;
                
                match result {
                    Some(t) => sender.send(t).await.unwrap(),
                    None => sleep(Duration::from_millis(500)).await,

                }
            }
        });
        Ok(())
    }
}