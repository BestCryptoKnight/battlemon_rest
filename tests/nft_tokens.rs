use fake::Fake;
use sqlx::types::{chrono::Utc, Json, Uuid};
use test_utils::{get_foo_lemon, tokens};

use crate::dummies::{AliceNftToken, BobNftToken, DannyNftToken};
use crate::helpers::assert_json_error;
use battlemon_rest::routes::NftToken;
use helpers::spawn_app;
use nft_models::ModelKind;

mod dummies;
mod helpers;

#[tokio::test]
async fn nft_tokens_success_with_valid_queries() {
    let app = spawn_app().await;
    let alice_tokens = (0..50).map(|_| AliceNftToken.fake());
    let bob_tokens = (0..30).map(|_| BobNftToken.fake());
    let danny_tokens = (0..20).map(|_| DannyNftToken.fake());
    let tokens: Vec<dummies::NftToken> =
        alice_tokens.chain(bob_tokens).chain(danny_tokens).collect();
    for token in tokens {
        sqlx::query!(
            r#"
            INSERT INTO nft_tokens (id, owner_id, token_id, media, model, db_created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            token.id,
            token.owner_id,
            token.token_id,
            token.media,
            Json(token.model) as _,
            token.db_created_at
        )
        .execute(&app.db_pool)
        .await
        .unwrap();
    }

    let valid_queries = [
        ("", 100),
        ("owner_id=alice.near", 50),
        ("owner_id=bob.near", 30),
        ("owner_id=danny.near", 20),
        ("owner_id=danny.near&limit=10", 10),
        ("owner_id=danny.near&limit=20", 20),
        ("owner_id=danny.near&limit=30", 20),
        ("owner_id=danny.near&offset=10", 10),
        ("owner_id=danny.near&offset=10&limit=5", 5),
    ];

    for query in valid_queries {
        let response = app.get_nft_tokens(query.0).await;
        assert!(response.status().is_success());
        let tokens: Vec<NftToken> = response
            .json()
            .await
            .expect("Couldn't parse response into `NftToken`");

        assert_eq!(tokens.len(), query.1);
    }
}

#[tokio::test]
async fn nft_tokens_fails_and_return_500_if_there_is_a_fatal_database_error() {
    let app = spawn_app().await;
    sqlx::query!("ALTER TABLE nft_tokens DROP COLUMN owner_id;",)
        .execute(&app.db_pool)
        .await
        .unwrap();
    let response = app.get_nft_tokens("owner_id=alice.near").await;
    assert_eq!(
        response.status().as_u16(),
        500,
        "The API didn't return 500 Internal Server Error"
    );

    assert_json_error(response).await
}

#[tokio::test]
async fn nft_tokens_for_valid_query_by_token_id_returns_200() {
    let app = spawn_app().await;
    let token_ids = tokens::<5>();
    let model = ModelKind::Lemon(get_foo_lemon());
    let owner_id = "alice.near";
    let ipfs_hash = "somehash";

    for id in token_ids.iter() {
        sqlx::query!(
            r#"
            INSERT INTO nft_tokens (id, owner_id, token_id, media, model, db_created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            Uuid::new_v4(),
            owner_id,
            id,
            ipfs_hash,
            Json(model.clone()) as _,
            Utc::now()
        )
        .execute(&app.db_pool)
        .await
        .expect("Failed to execute query");
    }

    for expected_token_id in token_ids {
        let query = format!("token_id={expected_token_id}");
        let response = app.get_nft_tokens(&query).await;
        assert!(response.status().is_success());

        let nft_tokens_json = response
            .json::<Vec<NftToken>>()
            .await
            .expect("Couldn't deserialize response into `Vec<NftToken>`");
        assert_eq!(
            nft_tokens_json.len(),
            1,
            "Expected length `1` for query `{}` and actual doesn't equal.",
            query
        );

        let actual_token_id = nft_tokens_json[0].token_id.as_str();
        assert_eq!(
            actual_token_id, expected_token_id,
            "Token id from response `{}` and expected id `{}` doesn't equal.",
            actual_token_id, expected_token_id
        );
    }
}

#[tokio::test]
async fn nft_tokens_for_valid_query_by_owner_id_returns_200() {
    let app = spawn_app().await;
    let token_ids = tokens::<5>();
    let model = ModelKind::Lemon(get_foo_lemon());

    let expected_owners_and_tokens_length = [
        ("owner1.testnet", 3),
        ("owner2.testnet", 2),
        ("others.testnet", 0),
    ];
    let ipfs_hash = "somehash";

    for (idx, id) in token_ids.iter().enumerate() {
        let owner_id = if idx < 3 {
            expected_owners_and_tokens_length[0].0
        } else {
            expected_owners_and_tokens_length[1].0
        };

        sqlx::query!(
            r#"
            INSERT INTO nft_tokens (id, owner_id, token_id, media, model, db_created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            Uuid::new_v4(),
            owner_id,
            id,
            ipfs_hash,
            Json(model.clone()) as _,
            Utc::now()
        )
        .execute(&app.db_pool)
        .await
        .expect("Failed to execute query");
    }

    for (id, length) in expected_owners_and_tokens_length {
        let query = format!("owner_id={id}");
        let response = app.get_nft_tokens(&query).await;
        assert!(response.status().is_success());
        let nft_tokens_json = response
            .json::<Vec<NftToken>>()
            .await
            .expect("Couldn't deserialize response into `Vec<NftToken>`");
        assert_eq!(
            nft_tokens_json.len(),
            length,
            "Expected length `{}` of tokens for owner `{}` and actual doesn't equal.",
            length,
            id
        );
        assert!(
            nft_tokens_json.iter().all(|v| v.owner_id == id),
            "Not all tokens belong to user with id `{}`",
            id
        );
    }
}
