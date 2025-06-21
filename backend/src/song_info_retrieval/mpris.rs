use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use mpris::{Metadata, PlaybackStatus, Player, PlayerFinder};
use tokio::sync::{mpsc, watch};
use tokio::sync::mpsc::Sender;
use tokio::time::sleep;
use crate::models::{PlaybackState, SongInfo};
use crate::song_info_retrieval::SongInfoRetriever;

pub struct MprisRetriever {
    active_player_bus_name: Arc<Mutex<String>>,
}
impl MprisRetriever {
    pub fn new() -> Result<MprisRetriever, Box<dyn Error>> {
        Ok(MprisRetriever { active_player_bus_name: Arc::new(Mutex::new(String::default())) })
    }
}

impl SongInfoRetriever for MprisRetriever {
    fn watch_active_song(&self, sender: mpsc::Sender<SongInfo>) -> Result<(), String> {
        let active_player_bus_name = Arc::clone(&self.active_player_bus_name);

        tokio::spawn(async move {
            let mut last_active_song = SongInfo::default();
            let active_bus_name = Arc::clone(&active_player_bus_name);

            loop {

                sleep(Duration::from_millis(500)).await;

                let result: Option<SongInfo> = async {
                    let finder = PlayerFinder::new().unwrap();

                    // TODO: dont panic here, just try again next time
                    let player = finder.find_active().unwrap();
                    let active_song = SongInfo::from_mpris_metadata(player.get_metadata().unwrap());

                    let mut bus_name = active_bus_name.lock().unwrap();
                    if *bus_name != player.bus_name() {
                        *bus_name = player.bus_name().to_string();
                    }

                    if active_song != last_active_song {
                        last_active_song = active_song.clone();
                        return Some(active_song);
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
        let active_player_bus_name = Arc::clone(&self.active_player_bus_name);

        tokio::spawn(async move {
            let active_bus_name = Arc::clone(&active_player_bus_name);
            
            loop {
                sleep(Duration::from_millis(500)).await;
                let result: Option<PlaybackState> = async {
                    let bus_name = {
                        let bus_name_guard = active_bus_name.lock().unwrap();
                        (*bus_name_guard).clone()
                    };
                    
                    let finder = PlayerFinder::new().unwrap();
                    if let Ok(player) = finder.find_by_name(bus_name.as_str()) {
                        let positon = player.get_position().unwrap();
                        match player.get_playback_status().unwrap() {
                            PlaybackStatus::Playing => return Some(PlaybackState::Playing(positon)),
                            PlaybackStatus::Paused => return Some(PlaybackState::Paused(positon)),
                            _ => {}
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