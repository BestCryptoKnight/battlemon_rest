use sqlx::{Connection, PgConnection};

use battlemon_rest::config;
use utils::spawn_app;

mod utils;

#[tokio::test]
async fn sales_works() {
    let app_address = spawn_app();
    let config = config::get_config().expect("Failed to read configuration");
    let conn_string = config.database.conn_string();
    let conn = PgConnection::connect(&conn_string)
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/sales?offset=10&limit=4", app_address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    let content = response.text().await.unwrap();
    assert_eq!(
        content, "pagination: Query(Pagination { limit: 4, offset: 10 })",
        "body of response with error",
    );
}
