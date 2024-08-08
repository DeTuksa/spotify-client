use actix_web::{dev::Server, App, HttpServer};
use std::net::TcpListener;

use super::routes;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
        .service(routes::get_user_profile)
    }).listen(listener)?.run();
    Ok(server)
}