use std::net::TcpListener;

use actix_web::web;
use sqlx::PgPool;
use tracing::instrument;

use crate::routes;

#[instrument("Running application", skip(listener, pool))]
pub fn run(listener: TcpListener, pool: PgPool) -> Result<actix_web::dev::Server, std::io::Error> {
    let pool = web::Data::new(pool);
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(tracing_actix_web::TracingLogger::default())
            .route("/health_check", web::get().to(routes::health_check))
            .route("/sales", web::get().to(routes::sale))
            .route("/paid", web::get().to(routes::paid))
            .route("/nft_tokens", web::get().to(routes::nft_tokens))
            .app_data(pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
