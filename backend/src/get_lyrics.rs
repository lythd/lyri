use crate::models::SongInfo;
use lrclib_api_rs::LRCLibAPI;
use reqwest::{Client, Url};
use std::str::FromStr;
use http::Method;

pub async fn get_lyrics(song_info: &SongInfo) -> Option<String> {
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
                return Some(lyrics);
            }
        }
    }    
    None
    
    
}