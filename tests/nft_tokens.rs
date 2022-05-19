use sqlx::types::{chrono::Utc, Json, Uuid};

use battlemon_rest::routes::NftToken;
use helpers::spawn_app;
use nft_models::lemon::{Cap, Cloth, Exo, Eyes, Head, Teeth};
use nft_models::{Lemon, ModelKind};

mod dummies;
mod helpers;

#[tokio::test]
async fn nft_tokens_query_by_owner_id_returns_200() {
    let app = spawn_app().await;
    let token_id = ["1", "2", "3", "4", "5"];
    let model = ModelKind::Lemon(Lemon {
        cap: Cap::MA01,
        cloth: Cloth::MA01,
        exo: Exo::BA01,
        eyes: Eyes::A01,
        head: Head::A01,
        teeth: Teeth::A01,
    });

    let expected_owners_and_tokens_length = [
        ("owner1.testnet", 3),
        ("owner2.testnet", 2),
        ("others.testnet", 0),
    ];
    let ipfs_hash = "somehash";

    for (idx, id) in token_id.iter().enumerate() {
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
