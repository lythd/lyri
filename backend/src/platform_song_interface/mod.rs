use crate::models::{PlaybackState, SongInfo};
use std::error::Error;

mod mpris;

pub trait PlatformSongInterface: Send + Sync {
    fn get_active_song(&self) -> Option<SongInfo>;
    //gets playback duration on currently playing song
    fn get_playback_duration(&self) -> Option<PlaybackState>;
    fn play(&self) -> bool;
    fn pause(&self) -> bool;
    fn next(&self) -> bool;
    fn prev(&self) -> bool;

}

// Function to get the platform-specific retriever implementation
pub fn get_platform_retriever() -> Box<dyn PlatformSongInterface> {
    #[cfg(target_os = "linux")]
    {
        Box::new(mpris::MprisRetriever::new().unwrap())
    }
    #[cfg(target_os = "windows")]
    {
        // TODO 
        Box::new(gsmtc::GsmtcRetriever::new())
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        panic!("unsuported os");
    }
}