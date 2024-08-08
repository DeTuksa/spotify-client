use std::{collections::HashMap, sync::Arc};

use actix_web::{get, web, HttpResponse, Responder};
use crate::{auth::{self, AuthManager}, search};

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

#[get("/search")]
pub async fn search_song(
    auth_manager: web::Data<Arc<AuthManager>>,
    query: web::Query<HashMap<String, String>>) -> impl Responder {

        let default_query = "".to_string();
        let query_string = query.get("query").unwrap_or(&default_query);

        match auth_manager.get_access_token().await {
            Ok(token) => match  search::search_track(&token, query_string).await {
                Ok(result) => HttpResponse::Ok().json(result),
                Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
            },
            Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err))
        }
}