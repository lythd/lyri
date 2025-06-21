// frontend/src/main.rs (simplified)
use backend::models::{PlaybackState, SongInfo};
use backend::song_info_retrieval;
use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx_song_info, mut rx_song_info) = mpsc::channel::<SongInfo>(10);
    let (tx_playback_pos, mut rx_playback_pos) = mpsc::channel::<PlaybackState>(100);

    let song_retriever = song_info_retrieval::get_platform_retriever();
    if let Err(e) = song_retriever.watch_active_song(tx_song_info) {
        eprintln!("Error starting song info watch: {}", e);
        return;
    }
    
    if let Err(e) = song_retriever.watch_duration(tx_playback_pos) {
        eprintln!("Error starting song duration watch: {}", e);
        return;
    }

    // Spawn a task to handle song info updates
    tokio::spawn(async move {
        while let Some(song_info) = rx_song_info.recv().await {
            if let Some(title) = song_info.title {
                println!("UI: New Song! {}", title);
            }
        }
    });

    // Spawn a task to handle playback position updates
    tokio::spawn(async move {
        while let Some(position) = rx_playback_pos.recv().await {
            println!("UI: Current position: {:?}", position);
        }
    });
    
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    println!("Shutting down.");
}