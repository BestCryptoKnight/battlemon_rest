#[tracing::instrument(name = "Health check", fields(request_id = %uuid::Uuid::new_v4()))]
pub async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().finish()
}
