use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64
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