use std::error::Error;
use crate::models::SongInfo;

mod mpris;

pub trait SongInfoRetriever {
    fn get_current_song(&self) -> Result<SongInfo, Box<dyn Error>>;
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