use crate::helpers::{assert_json_error, spawn_app};
use anyhow::Context;

use battlemon_models::market::{Sale, SaleForInserting};
use battlemon_rest::routes::RowsJsonReport;
use fake::{Fake, Faker};
use serde_json::json;
use uuid::Uuid;

mod dummies;
mod helpers;

#[ignore]
#[tokio::test]
async fn invalid_password_is_rejected() {
    let app = spawn_app().await;
    let username = &app.test_user.username;
    let password = Uuid::new_v4().to_string();
    assert_ne!(
        app.test_user.password, password,
        "Passwords should be different"
    );

    let response = reqwest::Client::new()
        .post(&format!("{}/sales", &app.address))
        .basic_auth(username, Some(password))
        .json(&Faker.fake::<SaleForInserting>())
        .send()
        .await
        .expect("Failed to send request");

    let actual_status = response.status();
    assert_eq!(
        actual_status,
        reqwest::StatusCode::UNAUTHORIZED,
        "The expected status code must be 401, actual is `{}`",
        actual_status
    );
    assert_eq!(
        r#"Basic realm="sales""#,
        response.headers()["WWW-Authenticate"],
        r#"The WWW-Authenticate header must be set to `Basic realm="sales"`"#,
    );

    assert_json_error(response).await;
}

#[ignore]
#[tokio::test]
async fn non_existing_user_is_rejected() {
    let app = spawn_app().await;
    let username = Uuid::new_v4().to_string();
    let password = Uuid::new_v4().to_string();

    let response = reqwest::Client::new()
        .post(&format!("{}/sales", &app.address))
        .basic_auth(username, Some(password))
        .json(&Faker.fake::<SaleForInserting>())
        .send()
        .await
        .expect("Failed to send request");

    let actual_status = response.status();
    assert_eq!(
        actual_status,
        reqwest::StatusCode::UNAUTHORIZED,
        "The expected status code must be 401, actual is `{}`",
        actual_status
    );
    assert_eq!(
        r#"Basic realm="sales""#,
        response.headers()["WWW-Authenticate"],
        r#"The WWW-Authenticate header must be set to `Basic realm="sales"`"#,
    );

    assert_json_error(response).await;
}

#[ignore]
#[tokio::test]
async fn requests_missing_authorization_are_rejected() {
    let app = spawn_app().await;

    let response = reqwest::Client::new()
        .post(&format!("{}/sales", app.address))
        .json(&Faker.fake::<SaleForInserting>())
        .send()
        .await
        .expect("Failed to execute request");

    let actual_status = response.status();
    assert_eq!(
        actual_status,
        reqwest::StatusCode::UNAUTHORIZED,
        "The expected status code must be 401, actual is `{}`",
        actual_status
    );
    assert_eq!(
        r#"Basic realm="sales""#,
        response.headers()["WWW-Authenticate"],
        r#"The WWW-Authenticate header must be set to `Basic realm="sales"`"#,
    );

    assert_json_error(response).await;
}

#[tokio::test]
async fn insert_valid_sale_success() -> anyhow::Result<()> {
    let app = spawn_app().await;
    let sale: SaleForInserting = Faker.fake();
    let response = app.post_sale(&sale).await;

    let status = response.status();
    let body = response
        .text()
        .await
        .context("Couldn't get the response text")?;

    assert_eq!(
        status, 201,
        "The expected response doesn't have `201` status code.\n
        The actual response has status code `{}` and body: `{}`",
        status, body,
    );

    let response: RowsJsonReport<Sale> = app
        .get_sales(&format!("token_id={}", sale.token_id))
        .await
        .json()
        .await?;
    assert_eq!(
        response.rows[0].token_id, sale.token_id,
        "The inserted sale doesn't match the returned sale"
    );

    Ok(())
}

#[tokio::test]
async fn insert_invalid_sale_rejects_and_returns_400_status() {
    let app = spawn_app().await;
    let sale = json!({
        "wrong": "token json"
    });
    let response = app.post_sale(&sale).await;
    assert_eq!(response.status(), 400, "Response status is not `400`");
    assert_json_error(response).await;
}

#[tokio::test]
async fn insert_valid_two_sales_success() -> anyhow::Result<()> {
    let app = spawn_app().await;
    let sale: SaleForInserting = Faker.fake();
    for _ in 0..2 {
        let response = app.post_sale(&sale).await;

        let status = response.status();
        let body = response
            .text()
            .await
            .context("Couldn't get the response text")?;

        assert_eq!(
            status, 201,
            "The expected response doesn't have `201` status code.\n 
            The actual response has status code `{}` and body: `{}`",
            status, body,
        );
    }

    let response: RowsJsonReport<Sale> = app
        .get_sales(&format!("token_id={}", sale.token_id))
        .await
        .json()
        .await?;

    assert_eq!(
        response.rows.len(),
        2,
        "The expected number of rows is 2, actual is `{}`",
        response.rows.len()
    );

    assert_eq!(
        response.rows[0].token_id, sale.token_id,
        "The inserted token id doesn't match the returned token id"
    );
    Ok(())
}

#[tokio::test]
async fn insert_sale_fails_and_return_500_if_there_is_a_fatal_database_error() {
    let app = spawn_app().await;
    let sale: SaleForInserting = Faker.fake();
    sqlx::query!("ALTER TABLE sales DROP COLUMN prev_owner;",)
        .execute(&app.db_pool)
        .await
        .unwrap();
    let response = app.post_sale(&sale).await;
    let status = response.status();
    let body = response
        .text()
        .await
        .expect("Couldn't get the response text");

    assert_eq!(
        status, 500,
        r#"The actual API didn't return 500 Internal Server Error.
        The actual response has status code `{}`.
        The actual response body: `{}`"#,
        status, body,
    );

    let response = app.post_sale(&sale).await;
    assert_json_error(response).await
}
