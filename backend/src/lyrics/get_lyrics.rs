use crate::models::SongInfo;
use lrclib_api_rs::LRCLibAPI;
use reqwest::{Client, Url};
use std::str::FromStr;
use http::Method;
use log::debug;
use crate::lyrics::models::{LrclibResponse, SyncedSongLyrics};


async fn get_lrclib_lyrics(song_info: &SongInfo) -> Option<LrclibResponse> {
    let api = LRCLibAPI::new();
    let title = song_info.title.as_deref()?;
    
    for artist in song_info.artists.as_deref()? {
        let request = api.get_lyrics(
            title,
            artist,
            song_info.album_name.as_deref(),
            song_info.duration_ms.map(|d| d.as_secs())
        ).ok()?;

        let client = Client::new();
        let request_uri = &request.uri().to_string();

        let reqwest_request = reqwest::Request::new(Method::GET, Url::from_str(request_uri).ok()?);

        if let Ok(response) = client.execute(reqwest_request).await {
            if let Ok(lyrics) = response.text().await {
                return LrclibResponse::new(&lyrics);
            }
        }
    }    
    None
}

// TODO: come up with method to represent languages and make this take in the users language
pub async fn get_synced_lyrics(song_info: &SongInfo) -> Result<SyncedSongLyrics, &str> {

    // If lrclib has synced lyrics, convert and return
    if let Some(lrclib_response) = get_lrclib_lyrics(song_info).await {
        println!("converted lcrlib_response:\n {lrclib_response:?}");
        if let Ok(synced_lyrics) = SyncedSongLyrics::from_lcrlib_response(&lrclib_response) {
            return Ok(synced_lyrics)
        }
    }
    // if we find other synced lyric providers other than lrclib, add them here
    Err("Could not convert song")
}

// use ai methods to convert songs to syncedSongLyrics, try get_synced_lyrics first
pub async fn get_synced_lyrics_ai(song_info: &SongInfo, openai_apikey: String) {
    unimplemented!()
}