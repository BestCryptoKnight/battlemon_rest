use crate::config::get_config;
use actix_web::web;

#[tracing::instrument(name = "List of contracts ids from config", fields(request_id = %uuid::Uuid::new_v4()))]
pub async fn contracts() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().finish()
}
