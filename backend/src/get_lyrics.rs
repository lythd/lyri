use crate::models::SongInfo;
use lrclib_api_rs::LRCLibAPI;
use reqwest::{Client, Url};
use std::str::FromStr;
use http::Method;
// TODO: create song lyrics representation that will separate the song language into "words"
// and then translate each word into the users language, then highlight the "words"  so that the user can see definitions for each word 
pub async fn get_synced_lyrics(song_info: &SongInfo) -> Option<String> {
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