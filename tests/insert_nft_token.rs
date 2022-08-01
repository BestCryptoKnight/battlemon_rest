use crate::helpers::{assert_json_error, spawn_app};
use anyhow::Context;

use battlemon_rest::routes::{NftToken, RowsJsonReport};
use fake::Fake;
use serde_json::json;
use uuid::Uuid;

mod dummies;
mod helpers;

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
        .post(&format!("{}/nft_tokens", &app.address))
        .basic_auth(username, Some(password))
        .json(&dummies::AliceNftToken.fake::<dummies::NftToken>())
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
        r#"Basic realm="nft_token""#,
        response.headers()["WWW-Authenticate"],
        r#"The WWW-Authenticate header must be set to `Basic realm="nft_token"`"#,
    );

    assert_json_error(response).await;
}

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
        r#"The WWW-Authenticate header must be set to `Basic realm="nft_token"`"#,
    );

    assert_json_error(response).await;
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
        r#"The WWW-Authenticate header must be set to `Basic realm="nft_token"`"#,
    );

    assert_json_error(response).await;
}

#[tokio::test]
async fn insert_valid_nft_token_success() -> anyhow::Result<()> {
    let app = spawn_app().await;
    let token: dummies::NftToken = dummies::AliceNftToken.fake();
    let response = app.post_nft_token(&token).await;

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

    let response: RowsJsonReport<NftToken> = app
        .get_nft_tokens(&format!("token_id={}", token.token_id))
        .await
        .json()
        .await?;
    assert_eq!(
        response.rows[0].token_id, token.token_id,
        "The inserted token id doesn't match the returned token id"
    );
    Ok(())
}

#[tokio::test]
async fn insert_valid_two_equals_nft_token_reject_without_error_response() -> anyhow::Result<()> {
    let app = spawn_app().await;
    let token: dummies::NftToken = dummies::AliceNftToken.fake();
    for _ in 0..2 {
        app.post_nft_token(&token).await;

        let response = app.post_nft_token(&token).await;

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

    let response: RowsJsonReport<NftToken> = app
        .get_nft_tokens(&format!("token_id={}", token.token_id))
        .await
        .json()
        .await?;

    assert_eq!(
        response.rows.len(),
        1,
        "The expected number of rows is 1, actual is `{}`",
        response.rows.len()
    );

    assert_eq!(
        response.rows[0].token_id, token.token_id,
        "The inserted token id doesn't match the returned token id"
    );
    Ok(())
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
