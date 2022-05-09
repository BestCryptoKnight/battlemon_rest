use actix_web::body::MessageBody;
use actix_web::error::QueryPayloadError;
use actix_web::{error, web, HttpResponse, ResponseError};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::instrument;

use crate::routes;

#[instrument("Running application", skip(listener, pool))]
pub fn run(listener: TcpListener, pool: PgPool) -> Result<actix_web::dev::Server, std::io::Error> {
    let pool = web::Data::new(pool);
    let server = actix_web::HttpServer::new(move || {
        let query_config = web::QueryConfig::default().error_handler(|err, _req| {
            let json_body = serde_json::json!({
                "error": err.to_string(),
            });
            error::InternalError::from_response(err, HttpResponse::BadRequest().json(json_body))
                .into()
        });

        actix_web::App::new()
            .wrap(tracing_actix_web::TracingLogger::default())
            .route("/health_check", web::get().to(routes::health_check))
            .route("/sales", web::get().to(routes::sale))
            .route("/paid", web::get().to(routes::paid))
            .route("/nft_tokens", web::get().to(routes::nft_tokens))
            .app_data(pool.clone())
            .app_data(query_config)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
