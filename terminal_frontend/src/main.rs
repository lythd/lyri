use std::sync::Arc;
// frontend/src/main.rs (simplified)
use backend::models::{PlaybackState, SongInfo};
use backend::song_info_retrieval;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use backend::song_info_retrieval::PlatformSongInterface;

enum Message {
    UpdateSong(SongInfo),
    UpdatePlayback(PlaybackState)
}
#[tokio::main]
async fn main() {
    let (tx_message, mut rx_message) = mpsc::channel::<Message>(10);

    let app = App::new();
    if let Err(e) = app.watch_active_song(tx_message.clone()) {
        eprintln!("Error starting song info watch: {}", e);
        return;
    }
    if let Err(e) = app.watch_duration(tx_message.clone()) {
        eprintln!("Error starting song duration watch: {}", e);
        return;
    }

    while let Some(message) = rx_message.recv().await {
        if let Message::UpdateSong(info) = message {
            if let Some(title) = info.title {
                println!("UI: New Song! {}", title);
            }
        } else if let Message::UpdatePlayback(playback) = message {
            println!("Playback updated {playback:?}");
        }
    }

    
    println!("Shutting down.");
}
struct App {
    song_retriever: Arc<Box<dyn PlatformSongInterface>>
}
impl App {
    fn new() -> App{
        App {song_retriever: Arc::new(song_info_retrieval::get_platform_retriever())    }
    }
    fn watch_active_song(&self, sender: mpsc::Sender<Message>) -> Result<(), String> {
        let song_retriever = Arc::clone(&self.song_retriever);
        tokio::spawn(async move {
            let mut current_song_info = SongInfo::default();
            loop {
                sleep(Duration::from_millis(500)).await;
                
                if let Some(info) = song_retriever.get_active_song() {
                    if current_song_info != info {
                        current_song_info = info.clone();
                        sender.send(Message::UpdateSong(info)).await.unwrap();
                    }
                }
            }
        });
        Ok(())
    }
    fn watch_duration(&self, sender: mpsc::Sender<Message>) -> Result<(), String> {
        let song_retriever = Arc::clone(&self.song_retriever);
        tokio::spawn(async move {
            let mut was_last_paused = false;
            loop {
                sleep(Duration::from_millis(500)).await;
                
                if let Some(state) = song_retriever.get_playback_duration() {
                    if state.same_kind(&PlaybackState::Paused(Duration::default())) && !was_last_paused {
                        was_last_paused = true;
                        sender.send(Message::UpdatePlayback(state)).await.unwrap();
                    } else if state.same_kind(&PlaybackState::Playing(Duration::default())) {
                        sender.send(Message::UpdatePlayback(state)).await.unwrap();
                    }
                }
            }            
        });
        Ok(())
    }
}

