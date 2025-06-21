use std::error::Error;
use std::time::Duration;
use tokio::sync::mpsc;
use crate::models::{PlaybackState, SongInfo};

mod mpris;

pub trait SongInfoRetriever: Send + Sync {
    fn get_active_song(&self) -> Option<SongInfo>;
    //gets playback duration on currently playing song
    fn get_playback_duration(&self) -> Option<PlaybackState>;
}

// Function to get the platform-specific retriever implementation
pub fn get_platform_retriever() -> Box<dyn SongInfoRetriever> {
    #[cfg(target_os = "linux")]
    {
        Box::new(mpris::MprisRetriever::new().unwrap())
    }
    #[cfg(target_os = "windows")]
    {
        Box::new(gsmtc::GsmtcRetriever::new())
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        // Fallback for unsupported OS, or return an error
        Box::new(unsupported::UnsupportedRetriever)
    }
}