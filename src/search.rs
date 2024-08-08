use serde::Deserialize;
use reqwest::Client;

#[derive(Deserialize, Debug)]
struct SearchResult {
    tracks: Tracks,
}

#[derive(Deserialize, Debug)]
struct Tracks {
    items: Vec<Track>,
}

#[derive(Deserialize, Debug)]
struct Track {
    name: String,
    artists: Vec<Artist>,
}

#[derive(Deserialize, Debug)]
struct Artist {
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