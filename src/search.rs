use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    tracks: Tracks,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tracks {
    items: Vec<Track>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    name: String,
    artists: Vec<Artist>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Artist {
    name: String,
}

pub async fn search_track(token: &str, query: &str) -> Result<SearchResult, Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.get("https://api.spotify.com/v1/search")
        .query(&[("q", query), ("type", "track")])
        .bearer_auth(token)
        .send()
        .await?
        .json::<SearchResult>()
        .await?;

    Ok(res)
}