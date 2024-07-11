use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64
}

#[derive(Deserialize, Debug)]
pub struct UserProfile {
    display_name: Option<String>,
    id: Option<String>,
    email: Option<String>
}

pub async fn get_access_token(
    client_id: &str,
    client_secret: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let params = [
        ("grant_type", "client_credentials")
    ];

    let res = client.post("https://accounts.spotify.com/api/token")
        .basic_auth(client_id, Some(client_secret))
        .form(&params)
        .send()
        .await?
        .json::<TokenResponse>().await?;

    Ok(res.access_token)
}

pub async fn get_user_profile(token: &str) -> Result<UserProfile, Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.get("https://api.spotify.com/v1/me")
        .bearer_auth(token)
        .send()
        .await?
        .json::<UserProfile>()
        .await?;

    Ok(res)
}