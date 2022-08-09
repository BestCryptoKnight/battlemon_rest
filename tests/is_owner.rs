use crate::dummies::{AliceNftToken, NftToken};
use fake::Fake;
use helpers::{assert_json_error, spawn_app};
use serde_json::Value;
use sqlx::types::{chrono::Utc, Json};

mod dummies;
mod helpers;

#[tokio::test]
async fn is_owner_route_for_nft_tokens_success() {
    let app = spawn_app().await;
    let tokens = (0..10).map(|i| {
        let mut token: NftToken = AliceNftToken.fake();
        token.token_id = i.to_string();
        token
    });

    for token in tokens {
        sqlx::query!(
            r#"
            INSERT INTO nft_tokens (owner_id, token_id, media, model, db_created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            token.owner_id,
            token.token_id,
            token.media,
            Json(token.model) as _,
            Utc::now()
        )
        .execute(&app.db_pool)
        .await
        .unwrap();
    }

    let valid_payloads_and_expected_results = [
        (vec!["1", "2"], true),
        (vec!["2", "1"], true),
        (vec!["11", "2"], false),
        (vec!["1", "2", "3", "4", "5"], true),
        (vec!["5", "4", "3", "2", "1"], true),
        (vec!["-1", "2"], false),
        (vec!["one", "two"], false),
        (vec!["0", "9"], true),
        (vec!["0", "10"], false),
    ];

    for (payload, expected) in valid_payloads_and_expected_results {
        let response = app
            .post(
                "users/alice.near/is_owner",
                &serde_json::to_value(&payload)
                    .expect("Failed to deserialize array with token ids"),
            )
            .await;
        assert!(
            response.status().is_success(),
            "Response status for payload `{:?}` doesn't equal `200`",
            payload
        );
        let ret: Value = response
            .json()
            .await
            .expect("Couldn't parse response into `Value`");

        assert_eq!(ret["result"], expected);
    }
}
