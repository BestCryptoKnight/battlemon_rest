use battlemon_rest::routes::NftToken;
use nft_models::lemon::{Cap, Cloth, Exo, Eyes, Head, Teeth};
use nft_models::{Lemon, ModelKind};
use sqlx::types::{chrono::Utc, Json, Uuid};
use utils::spawn_app;
mod dummies;
mod utils;

#[tokio::test]
async fn nft_tokens() {
    let app = spawn_app().await;
    let owner_id = "owner.testnet";
    let token_id = "123";
    let model = ModelKind::Lemon(Lemon {
        cap: Cap::MA01,
        cloth: Cloth::MA01,
        exo: Exo::BA01,
        eyes: Eyes::A01,
        head: Head::A01,
        teeth: Teeth::A01,
    });

    let ipfs_hash = "somehash";
    sqlx::query!(
        r#"
        INSERT INTO nft_tokens (id, owner_id, token_id, media, model, db_created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        Uuid::new_v4(),
        owner_id,
        token_id,
        ipfs_hash,
        Json(model) as _,
        Utc::now()
    )
    .execute(&app.db_pool)
    .await
    .expect("Failed to execute query");

    let token = sqlx::query_as!(
        NftToken,
        r#"
        SELECT id, owner_id, token_id, media, model as "model: Json<ModelKind>", db_created_at, copies, description, expires_at, issued_at, title, media_hash
        FROM nft_tokens;
        "#,
    )
    .fetch_one(&app.db_pool)
    .await
    .unwrap();

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/nft_tokens", app.address))
        .send()
        .await
        .expect("Failed to execute request");
}
