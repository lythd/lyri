use std::time::Duration;
use mpris::Metadata;
#[derive(Debug, Clone, Default, PartialEq)]
pub enum PlaybackState {
    #[default]
    None,
    Playing(Duration),
    Paused(Duration),
}

impl PlaybackState {
    pub fn same_kind(&self, other: &Self) -> bool {
        match (self, other) {
            (PlaybackState::None, PlaybackState::None) => true,
            (PlaybackState::Playing(_), PlaybackState::Playing(_)) => true,
            (PlaybackState::Paused(_), PlaybackState::Paused(_)) => true,
            _ => false,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SongInfo {
    pub title: Option<String>,
    pub artists: Option<Vec<String>>,
    pub album_name: Option<String>,
    pub duration_ms: Option<Duration>,
    pub url: Option<String>,
    pub art_url: Option<String>
}

impl SongInfo {
    pub fn from_mpris_metadata(metadata: Metadata) -> SongInfo {
        SongInfo {
            title: metadata.title().map(|s| s.to_string()),
            artists: metadata.artists().map(|s| s.iter().map(|s| s.to_string()).collect()),
            album_name: metadata.album_name().map(|s| s.to_string()),
            duration_ms: metadata.length(),
            url: metadata.url().map(|s| s.to_string()),
            art_url: metadata.art_url().map(|s| s.to_string()),
        }
    }
}

pub struct SyncedSongLyrics {
    pub lines: Vec<SongLine>
}
pub struct SongLine {
    pub duration: Duration,
    pub user_line: String,
    pub translation_line: TranslationLine,
}
pub struct TranslationLine {
    // units of meaning, in both the songs language and the users language
    pub morphemes: Vec<(String,String)>
}