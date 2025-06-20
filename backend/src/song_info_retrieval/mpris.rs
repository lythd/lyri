use std::collections::HashMap;
use std::error::Error;
use mpris::{Player, PlayerFinder};
use crate::models::SongInfo;
use crate::song_info_retrieval::SongInfoRetriever;

pub struct MprisRetriever {
    finder: PlayerFinder,
    song_players: HashMap<SongInfo, Player>,
}
impl MprisRetriever {
    pub fn new() -> Result<MprisRetriever, Box<dyn Error>> {
        let finder = PlayerFinder::new()?;
        Ok(MprisRetriever { finder, song_players: HashMap::new() })
    }
}

impl SongInfoRetriever for MprisRetriever {
    fn get_current_song(&self) -> Result<SongInfo, Box<dyn Error>> {
        let player = self.finder.find_active()?;
        let metadata = player.get_metadata()?;
        
        Ok(SongInfo {
            title: metadata.title().map(|s| s.to_string()),
            artists: metadata.artists().map(|s| s.iter().map(|s| s.to_string()).collect()),
            album_name: metadata.album_name().map(|s| s.to_string()),
            duration_ms: metadata.length(),
            url: metadata.url().map(|s| s.to_string()),
        })
    }
}