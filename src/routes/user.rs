use std::sync::Arc;

use actix_web::{get, web, HttpResponse, Responder};
use crate::auth::{self, AuthManager};

#[get("/get-profile")]
pub async fn get_user_profile(auth_manager: web::Data<Arc<AuthManager>>) -> impl Responder {
    match auth_manager.get_access_token().await {
        Ok(token) => match auth::get_user_profile(&token).await {
            Ok(profile) => HttpResponse::Ok().json(profile),
            Err(e) => HttpResponse::InternalServerError().body(format!("Error {}", e))
        },
        Err(err) => HttpResponse::InternalServerError().body(format!("Error {}", err))
    }
}
