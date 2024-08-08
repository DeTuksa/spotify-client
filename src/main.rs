use std::net::TcpListener;
use spotify_client::{configuration::get_configuration, startup::run};
use tokio;
use dotenv::dotenv;

mod auth;
mod routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let configuration = get_configuration().expect("Failed to read configuration");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listen = TcpListener::bind(address)?;
    
    run(listen)?.await
}
