use crate::helpers::{assert_json_error, spawn_app};

use crate::dummies::AliceNftToken;
use fake::Fake;
use serde_json::json;

mod dummies;
mod helpers;

#[tokio::test]
async fn insert_valid_nft_token_success() {
    let app = spawn_app().await;
    let token: dummies::NftToken = AliceNftToken.fake();
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
async fn insert_invalid_nft_token_fails() {
    let app = spawn_app().await;
    let token = json!({
        "wrong": "token json"
    });
    let response = app.post_nft_token(&token).await;
    assert_eq!(response.status(), 400, "Response status is not `400`");
    assert_json_error(response).await;
}
