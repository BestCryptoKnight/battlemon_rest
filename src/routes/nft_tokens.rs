use actix_web::{web, HttpResponse};
use anyhow::Context;
use battlemon_models::nft::{ModelKind, NftKind, NftTokenForRest};
use chrono::Utc;
use serde::Deserialize;
use sqlx::types::Json;
use sqlx::{PgPool, Postgres, Transaction};

use crate::domain::{
    Limit, NftTokenDays, NftTokenFilter, NftTokenOwnerId, Offset, Parse, ParseToPositiveInt,
    TokenId,
};
use crate::errors::NftTokensError;
use crate::routes::RowsJsonReport;

#[derive(Debug, Deserialize, Clone)]
pub struct NftTokenQuery {
    pub days: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub owner_id: Option<String>,
    pub token_id: Option<String>,
    pub nft_trait: Option<String>,
    pub nft_kind: Option<NftKind>,
}

impl TryFrom<NftTokenQuery> for NftTokenFilter {
    type Error = String;
    fn try_from(query: NftTokenQuery) -> Result<Self, Self::Error> {
        let token_id = TokenId::parse(query.token_id)?;
        let owner_id = NftTokenOwnerId::parse(query.owner_id)?;
        let limit = Limit::parse(query.limit)?;
        let offset = Offset::parse(query.offset)?;
        let nft_kind = query.nft_kind.map(|k| {
            serde_json::to_value(k)
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
        });
        NftTokenDays::parse(query.days)?;

        Ok(Self {
            token_id,
            owner_id,
            limit,
            offset,
            nft_kind,
        })
    }
}

#[tracing::instrument(name = "Handle nft tokens request", skip(filter, pool))]
pub async fn get_nft_tokens(
    web::Query(filter): web::Query<NftTokenQuery>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, NftTokensError> {
    let filter: NftTokenFilter = filter.try_into().map_err(NftTokensError::ValidationError)?;
    let nft_tokens = get_nft_tokens_db(pool, &filter)
        .await
        .context("Failed to get the nft tokens data from database.")?;

    Ok(HttpResponse::Ok().json(RowsJsonReport::from_rows(nft_tokens, filter.limit())))
}

#[tracing::instrument(name = "Query nft tokens from database", skip(filter, pool))]
pub async fn get_nft_tokens_db(
    pool: web::Data<PgPool>,
    filter: &NftTokenFilter,
) -> Result<Vec<NftTokenForRest>, anyhow::Error> {
    dbg!(filter.nft_kind());
    let rows= sqlx::query_as!(
        NftTokenForRest,
        r#"
        SELECT token_id, owner_id, media, model as "model: Json<ModelKind>", copies, description, expires_at, issued_at, title, media_hash
        FROM nft_tokens
        WHERE ($1::text IS null OR token_id = $1)
            AND ($2::text IS null OR owner_id = $2)
            AND ($3::text IS null OR model->>'kind' = $3)
        ORDER BY id LIMIT $4 OFFSET $5
        "#,
        filter.token_id(),
        filter.owner_id(),
        filter.nft_kind(),
        filter.limit() + 1,
        filter.offset(),
    )
        .fetch_all(pool.get_ref())
        .await?;

    Ok(rows)
}

#[tracing::instrument(name = "Insert nft tokens", skip(nft_token, pool))]
pub async fn insert_nft_token(
    web::Json(nft_token): web::Json<NftTokenForRest>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, NftTokensError> {
    let mut tx = pool.begin().await.context("Failed to start transaction.")?;
    insert_nft_token_db(nft_token, &mut tx)
        .await
        .context("Failed to insert the nft token data into the database.")?;
    tx.commit()
        .await
        .context("Failed to commit SQL transaction to store a new subscriber.")?;
    Ok(HttpResponse::Created().finish())
}

#[tracing::instrument(name = "Store nft tokens to database", skip(tx))]
pub async fn insert_nft_token_db(
    nft_token: NftTokenForRest,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<(), anyhow::Error> {
    sqlx::query_as!(
        NftToken,
        r#"
        INSERT INTO nft_tokens (owner_id, token_id, title, description, media, media_hash, copies, issued_at, expires_at, model, db_created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        ON CONFLICT (token_id) DO NOTHING
        "#,
        nft_token.owner_id,
        nft_token.token_id,
        nft_token.title,
        nft_token.description,
        nft_token.media,
        nft_token.media_hash,
        nft_token.copies,
        nft_token.issued_at,
        nft_token.expires_at,
        Json(nft_token.model) as _,
        Utc::now()
    )
    .execute(tx)
    .await?;

    Ok(())
}

#[tracing::instrument(name = "Update nft tokens", skip(nft_token, pool))]
pub async fn update_nft_token(
    web::Json(nft_token): web::Json<NftTokenForRest>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, NftTokensError> {
    let mut tx = pool.begin().await.context("Failed to start transaction.")?;
    update_nft_token_db(nft_token, &mut tx)
        .await
        .context("Failed to insert the nft token data into the database.")?;
    tx.commit()
        .await
        .context("Failed to commit SQL transaction to store a new subscriber.")?;
    Ok(HttpResponse::Created().finish())
}

#[tracing::instrument(name = "Store nft tokens to database", skip(tx))]
pub async fn update_nft_token_db(
    nft_token: NftTokenForRest,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<(), anyhow::Error> {
    sqlx::query_as!(
        NftToken,
        r#"
        UPDATE nft_tokens
        SET model = $1
        "#,
        Json(nft_token.model) as _,
    )
    .execute(tx)
    .await?;

    Ok(())
}
