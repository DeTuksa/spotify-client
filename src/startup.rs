use actix_web::web;
use actix_web::{dev::Server, App, HttpServer};
use std::net::TcpListener;
use std::sync::Arc;

use crate::auth::AuthManager;

use super::routes;

pub fn run(listener: TcpListener, auth_manager: Arc<AuthManager>) -> Result<Server, std::io::Error> {

    let server = HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(auth_manager.clone()))
        .service(routes::get_user_profile)
    }).listen(listener)?.run();
    Ok(server)
}