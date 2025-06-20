use std::time::Duration;

#[derive(Debug, Clone)]
pub struct SongInfo {
    pub title: Option<String>,
    pub artists: Option<Vec<String>>,
    pub album_name: Option<String>,
    pub duration_ms: Option<Duration>,
    pub url: Option<String>,
}