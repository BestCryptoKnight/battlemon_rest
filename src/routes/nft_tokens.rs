use super::PaginationQuery;
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use nft_models::ModelKind;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct NftToken {
    pub id: Uuid,
    pub owner_id: String,
    pub token_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: String,
    pub media_hash: Option<String>,
    pub copies: Option<String>,
    pub issued_at: Option<String>,
    pub expires_at: Option<String>,
    pub model: Json<ModelKind>,
    pub db_created_at: DateTime<Utc>,
}

pub async fn nft_tokens(
    filter: web::Query<PaginationQuery>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let limit = filter.limit.unwrap_or(100);
    let offset = filter.offset.unwrap_or_default();
    let rows = sqlx::query_as!(
        NftToken,
        r#"
        SELECT id, owner_id, token_id, media, model as "model: Json<ModelKind>", db_created_at, copies, description, expires_at, issued_at, title, media_hash
        FROM nft_tokens ORDER BY id LIMIT $1 OFFSET $2;
        "#,
        limit,
        offset
    ).fetch_all(pool.get_ref())
    .await;

    match rows {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(e) => {
            println!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
