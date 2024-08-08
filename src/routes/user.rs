use actix_web::{get, HttpResponse, Responder};
use crate::auth;

#[get("/get-profile")]
pub async fn get_user_profile() -> impl Responder {
    match auth::get_access_token().await {
        Ok(token) => match auth::get_user_profile(&token).await {
            Ok(profile) => HttpResponse::Ok().json(profile),
            Err(e) => HttpResponse::InternalServerError().body(format!("Error {}", e))
        },
        Err(err) => HttpResponse::InternalServerError().body(format!("Error {}", err))
    }
}
