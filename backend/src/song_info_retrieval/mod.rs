use std::error::Error;
use std::time::Duration;
use tokio::sync::mpsc;
use crate::models::{PlaybackState, SongInfo};

mod mpris;

pub trait SongInfoRetriever {
    // sends updates to the sender when the currently active song changes
    fn watch_active_song(&self, sender: mpsc::Sender<SongInfo>) -> Result<(), String>;
    //sends updates with playback state when the current song changes
    fn watch_duration(&self, sender: mpsc::Sender<PlaybackState>) -> Result<(), String>;
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