use std::env;
use tokio;
use dotenv::dotenv;

mod auth;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID not set");
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET not set");

    match auth::get_access_token(&client_id, &client_secret).await {
        Ok(token) => {
            println!("Access token: {}", token);

            match auth::get_user_profile(&token).await {
                Ok(profile) => println!("User profile is {:?}", profile),
                Err(e) => eprintln!("Error getting user profile {}", e)
            }
        },
        Err(e) => eprintln!("Error getting access token: {}", e)
    }
}
