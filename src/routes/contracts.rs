use actix_web::web;
use crate::config::get_config;

#[tracing::instrument(name = "List of contracts ids from config", fields(request_id = %uuid::Uuid::new_v4()))]
pub async fn contracts(contracts: web::Data<>) -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().finish()
}
