use crate::auth::password::{basic_auth, validate_credentials};
use crate::errors::JsonError;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::HttpResponse;
use actix_web_lab::middleware::Next;
use anyhow::Context;
use sqlx::PgPool;

pub async fn auth(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let creds = basic_auth(req.headers())?;
    let db_pool = req
        .app_data::<actix_web::web::Data<PgPool>>()
        .context("Failed to get database pool from application data.")
        .map_err(|e| {
            actix_web::error::InternalError::from_response(
                e.to_string(),
                HttpResponse::InternalServerError().json(JsonError::new(e)),
            )
        })?;

    tracing::Span::current().record("username", &tracing::field::display(&creds.username));
    let user_id = validate_credentials(creds, db_pool).await?;
    tracing::Span::current().record("user_id", &tracing::field::display(&user_id));

    next.call(req).await
}
