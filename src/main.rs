use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;

use battlemon_rest::{config, startup, telemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("indexer".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    tracing::info!("Loading app config");
    let config = config::get_config().expect("Failed to read configuration");
    tracing::info!("Connection to Postgres");
    let conn_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(config.database.with_db());

    let address = format!("{}:{}", config.application.host, config.application.port);
    tracing::info!("Binding address for app: {}", address);
    let listener = TcpListener::bind(address)?;
    startup::run(listener, conn_pool)?.await
}
