use once_cell::sync::Lazy;
use reqwest::{Client, Response};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use battlemon_rest::config;
use battlemon_rest::config::DatabaseSettings;
use battlemon_rest::errors::JsonError;
use battlemon_rest::startup::{get_connection_pool, Application};
use battlemon_rest::telemetry::{get_subscriber, init_subscriber};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_name: String,
    pub db_pool: PgPool,
}

#[allow(dead_code)]
impl TestApp {
    async fn get(&self, path: &str, query: &str) -> Response {
        Client::new()
            .get(&format!("{}/{path}?{query}", self.address))
            .send()
            .await
            .unwrap_or_else(|e| panic!("Failed to execute request {:#?}", e))
    }

    pub async fn get_paid(&self, query: &str) -> Response {
        self.get("paid", query).await
    }

    pub async fn get_sales(&self, query: &str) -> Response {
        self.get("sales", query).await
    }

    pub async fn get_nft_tokens(&self, query: &str) -> Response {
        self.get("nft_tokens", query).await
    }
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let config = {
        let mut cfg = config::get_config().expect("Failed to read configuration");
        cfg.database.database_name = Uuid::new_v4().to_string();
        cfg.application.port = 0;
        cfg
    };
    configure_database(&config.database).await;
    let application = Application::build(config.clone())
        .await
        .expect("Failed to build application");
    let address = format!("http://127.0.0.1:{}", application.port());
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        db_name: config.database.database_name.clone(),
        db_pool: get_connection_pool(&config.database),
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut conn = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    conn.execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let conn_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("../../migrations/migrations")
        .run(&conn_pool)
        .await
        .expect("Failed to migrate the database");

    conn_pool
}

#[allow(dead_code)]
pub async fn assert_json_error(response: Response) {
    let result = response.json::<JsonError>().await;
    assert!(
        result.is_ok(),
        "The response doesn't contain json error scheme: actual response is {:?}",
        result
    )
}
