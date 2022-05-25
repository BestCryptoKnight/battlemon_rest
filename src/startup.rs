use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{error, web, HttpResponse};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::config::{DatabaseSettings, Settings};
use crate::routes;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(config: Settings) -> Result<Application, std::io::Error> {
        tracing::info!("Connect to Postgres");
        let connection_pool = get_connection_pool(&config.database);
        let address = format!("{}:{}", config.application.host, config.application.port);
        tracing::info!("Binding address - {address} for app");
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, connection_pool)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(config.with_db())
}

#[tracing::instrument(name = "Running application", skip(listener, pool))]
pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
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
            .service(
                web::resource("nft_tokens")
                    .route(web::get().to(routes::nft_tokens))
                    .route(web::post().to(routes::insert_nft_token)),
            )
            .app_data(pool.clone())
            .app_data(query_config)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
