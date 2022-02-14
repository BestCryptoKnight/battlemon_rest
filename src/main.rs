use std::net::TcpListener;

use sqlx::PgPool;

use battlemon_rest::{config, startup, telemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("indexer".into(), "info".into());
    telemetry::init_subscriber(subscriber);
    tracing::info!("Loading app config");
    let config = config::get_config().expect("Failed to read configuration");
    tracing::info!("Connection to Postgres");
    let pool = PgPool::connect(&config.database.conn_string())
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("0.0.0.0:{}", config.app_port);
    tracing::info!("Binding address for app: {}", address);
    let listener = TcpListener::bind(address)?;
    startup::run(listener, pool)?.await
}
