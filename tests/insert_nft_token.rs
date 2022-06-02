use crate::helpers::{assert_json_error, spawn_app};

use fake::Fake;
use serde_json::json;
use uuid::Uuid;

mod dummies;
mod helpers;

#[tokio::test]
async fn non_existing_user_is_rejected() {
    let app = spawn_app().await;
    let username = Uuid::new_v4().to_string();
    let password = Uuid::new_v4().to_string();

    let response = reqwest::Client::new()
        .post(&format!("{}/nft_tokens", &app.address))
        .basic_auth(username, Some(password))
        .json(&dummies::AliceNftToken.fake::<dummies::NftToken>())
        .send()
        .await
        .expect("Failed to send request");
    let status = response.status();
    assert_eq!(
        status,
        reqwest::StatusCode::UNAUTHORIZED,
        "The actual status is {}, expected status is `401`",
        status.as_u16()
    );

    assert_json_error(response);
}

#[tokio::test]
async fn requests_missing_authorization_are_rejected() {
    let app = spawn_app().await;

    let response = reqwest::Client::new()
        .post(&format!("{}/nft_tokens", app.address))
        .json(&dummies::AliceNftToken.fake::<dummies::NftToken>())
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
        r#"Basic realm="nft_token""#,
        response.headers()["WWW-Authenticate"],
        r#"The WWW-Authenticate header must be set to `Basic realm="publish"`"#,
    );

    assert_json_error(response).await;
}

#[tokio::test]
async fn insert_valid_nft_token_success() {
    let app = spawn_app().await;
    let token: dummies::NftToken = dummies::AliceNftToken.fake();
    let response = app.post_nft_token(&token).await;

    let status = response.status();
    let body = response
        .text()
        .await
        .expect("Couldn't get the response text");

    assert_eq!(
        status, 201,
        "The expected response doesn't have `201` status code.\
         The actual response has status code `{}` and body: `{}`",
        status, body,
    );
}

#[tokio::test]
async fn insert_invalid_nft_token_rejects_and_returns_400_status() {
    let app = spawn_app().await;
    let token = json!({
        "wrong": "token json"
    });
    let response = app.post_nft_token(&token).await;
    assert_eq!(response.status(), 400, "Response status is not `400`");
    assert_json_error(response).await;
}

#[tokio::test]
async fn insert_nft_token_fails_and_return_500_if_there_is_a_fatal_database_error() {
    let app = spawn_app().await;
    let token: dummies::NftToken = dummies::AliceNftToken.fake();
    sqlx::query!("ALTER TABLE nft_tokens DROP COLUMN owner_id;",)
        .execute(&app.db_pool)
        .await
        .unwrap();
    let response = app.post_nft_token(&token).await;
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

    let response = app.post_nft_token(&token).await;
    assert_json_error(response).await
}
