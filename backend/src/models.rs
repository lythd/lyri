use std::time::Duration;
use mpris::Metadata;
#[derive(Debug)]
pub enum PlaybackState {
    Playing(Duration),
    Paused(Duration),
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SongInfo {
    pub title: Option<String>,
    pub artists: Option<Vec<String>>,
    pub album_name: Option<String>,
    pub duration_ms: Option<Duration>,
    pub url: Option<String>,
}

impl SongInfo {
    pub fn from_mpris_metadata(metadata: Metadata) -> SongInfo {
        SongInfo {
            title: metadata.title().map(|s| s.to_string()),
            artists: metadata.artists().map(|s| s.iter().map(|s| s.to_string()).collect()),
            album_name: metadata.album_name().map(|s| s.to_string()),
            duration_ms: metadata.length(),
            url: metadata.url().map(|s| s.to_string()),
        }
    }
}