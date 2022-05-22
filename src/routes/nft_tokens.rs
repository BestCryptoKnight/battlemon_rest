use actix_web::{web, HttpResponse};
use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::PgPool;
use uuid::Uuid;

use nft_models::ModelKind;

use crate::domain::{
    Limit, NftTokenDays, NftTokenFilter, NftTokenOwnerId, NftTokenTokenId, Offset, Parse,
    ParseToPositiveInt,
};
use crate::errors::NftTokensError;

#[derive(Debug, Deserialize)]
pub struct NftTokenQuery {
    pub days: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub owner_id: Option<String>,
    pub token_id: Option<String>,
    pub nft_trait: Option<String>,
}

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

impl TryFrom<NftTokenQuery> for NftTokenFilter {
    type Error = String;
    fn try_from(query: NftTokenQuery) -> Result<Self, Self::Error> {
        let token_id = NftTokenTokenId::parse(query.token_id)?;
        let owner_id = NftTokenOwnerId::parse(query.owner_id)?;
        let limit = Limit::parse(query.limit)?;
        let offset = Offset::parse(query.offset)?;
        NftTokenDays::parse(query.days)?;

        Ok(Self {
            token_id,
            owner_id,
            limit,
            offset,
        })
    }
}

#[tracing::instrument(name = "Handle nft tokens request", skip(filter, pool))]
pub async fn nft_tokens(
    web::Query(filter): web::Query<NftTokenQuery>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, NftTokensError> {
    let filter: NftTokenFilter = filter.try_into().map_err(NftTokensError::ValidationError)?;
    let nft_tokens = query_nft_tokens(pool, filter)
        .await
        .context("Failed to get the nft tokens data from database.")?;

    Ok(HttpResponse::Ok().json(nft_tokens))
}

#[tracing::instrument(name = "Query nft tokens from database", skip(filter, pool))]
pub async fn query_nft_tokens(
    pool: web::Data<PgPool>,
    filter: NftTokenFilter,
) -> Result<Vec<NftToken>, anyhow::Error> {
    let rows= sqlx::query_as!(
        NftToken,
        r#"
        SELECT id, owner_id, token_id, media, model as "model: Json<ModelKind>", db_created_at, copies, description, expires_at, issued_at, title, media_hash
        FROM nft_tokens
        WHERE ($1::text IS null OR token_id = $1)
            AND ($2::text IS null OR owner_id = $2)
        ORDER BY db_created_at LIMIT $3 OFFSET $4
        "#,
        filter.token_id(),
        filter.owner_id(),
        filter.limit(),
        filter.offset(),
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(rows)
}
