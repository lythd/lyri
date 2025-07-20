use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct SyncedSongLyrics {
    pub lines: Vec<SongLine>
}
impl SyncedSongLyrics {
    pub fn from_lcrlib_response(response: &LrclibResponse) -> Result<Self, ()> {
        if response.synced_lyrics.is_none() {
            Err(())
        } else {
            unimplemented!(/* TODO: parse synced lyrics */)
        }
    }
}
#[derive(Debug)]
pub struct SongLine {
    pub duration: Duration,
    pub user_line: String,
    pub translation_line: TranslationLine,
}
#[derive(Debug)]
pub struct TranslationLine {
    // units of meaning, in both the songs language and the users language
    pub morphemes: Vec<(String,String)>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LrclibResponse {
    id: u64,
    name: String,
    track_name: String,
    artist_name: String,
    album_name: String,
    duration: f64,
    instrumental: bool,
    pub(crate) plain_lyrics: Option<String>,
    synced_lyrics: Option<String>,
}
impl LrclibResponse {
    pub fn new(response: &String) -> Option<Self> {
        serde_json::from_str(response).ok()
    }
}