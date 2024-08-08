use log::debug;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfile {
    display_name: Option<String>,
    id: Option<String>,
    email: Option<String>
}

struct CachedToken {
    token: String,
    expires_at: Instant
}

#[derive(Clone)]
pub struct AuthManager {
    client_id: String,
    client_secret: String,
    cached_token: Arc<Mutex<Option<CachedToken>>>
}

impl AuthManager {
    pub fn new(client_id: String, client_secret: String) -> Self {
        AuthManager {
            client_id,
            client_secret,
            cached_token: Arc::new(Mutex::new(None))
        }
    }

    async fn fetch_access_token(&self) -> Result<String, Box<dyn std::error::Error>> {
        debug!("Fetching new access token...");
        let client = Client::new();
        let params = [("grant_type", "client_credentials")];

        let res = client.post("https://accounts.spotify.com/api/token")
        .basic_auth(&self.client_id, Some(&self.client_secret))
        .form(&params)
        .send()
        .await?
        .json::<TokenResponse>().await?;

    let expires_at = Instant::now() + Duration::from_secs(res.expires_in);

    let mut token_lock = self.cached_token.lock().unwrap();
    *token_lock = Some(CachedToken { token: res.access_token.clone(), expires_at });

    debug!("New token fetched and cached");
    Ok(res.access_token)
    }

    pub async fn get_access_token(&self) -> Result<String, Box<dyn std::error::Error>> {
        debug!("Retrieving access token...");
        {
            let token_lock = self.cached_token.lock().unwrap();

        if let Some(ref cached_token) = *token_lock {
            if cached_token.expires_at > Instant::now() {
                return Ok(cached_token.token.clone());
            } else {
                debug!("Cached token expired, fetchin new token...");
            }
        }
        }
        self.fetch_access_token().await
    }
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