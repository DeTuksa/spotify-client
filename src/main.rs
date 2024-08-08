use std::{net::TcpListener, sync::Arc};
use spotify_client::{configuration::get_configuration, startup::run};
use tokio;
use dotenv::dotenv;
use std::env;
use env_logger::Env;

use spotify_client::auth::AuthManager;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let configuration = get_configuration().expect("Failed to read configuration");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listen = TcpListener::bind(address)?;

    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID not set");
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET not set");

    let auth_manager = 
        Arc::new(
            AuthManager::new(client_id, client_secret)
        );
    
    run(listen, auth_manager)?.await
}
