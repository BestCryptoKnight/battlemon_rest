use actix_web::http::header::HeaderMap;
use actix_web::{web, HttpRequest, HttpResponse};
use anyhow::{anyhow, Context};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use battlemon_models::nft::ModelKind;
use chrono::Utc;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::{PgPool, Postgres, Transaction};

use crate::domain::{
    Limit, NftTokenDays, NftTokenFilter, NftTokenOwnerId, Offset, Parse, ParseToPositiveInt,
    TokenId,
};
use crate::errors::NftTokensError;
use crate::routes::RowsJsonReport;
use crate::telemetry::spawn_blocking_with_tracing;

#[derive(Debug, Deserialize, Clone)]
pub struct NftTokenQuery {
    pub days: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub owner_id: Option<String>,
    pub token_id: Option<String>,
    pub nft_trait: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftToken {
    pub token_id: String,
    pub owner_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: String,
    pub media_hash: Option<String>,
    pub copies: Option<String>,
    pub issued_at: Option<String>,
    pub expires_at: Option<String>,
    pub model: Json<ModelKind>,
}

impl TryFrom<NftTokenQuery> for NftTokenFilter {
    type Error = String;
    fn try_from(query: NftTokenQuery) -> Result<Self, Self::Error> {
        let token_id = TokenId::parse(query.token_id)?;
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
    let nft_tokens = query_nft_tokens(pool, &filter)
        .await
        .context("Failed to get the nft tokens data from database.")?;

    Ok(HttpResponse::Ok().json(RowsJsonReport::from_rows(nft_tokens, filter.limit())))
}

struct Credentials {
    username: String,
    password: Secret<String>,
}

fn basic_authentication(headers: &HeaderMap) -> Result<Credentials, anyhow::Error> {
    let header_value = headers
        .get("Authorization")
        .context("The `Authorization` header was missing.")?
        .to_str()
        .context("The `Authorization` header was not a valid UTF-8 string.")?;
    let base64encoded_segment = header_value
        .strip_prefix("Basic ")
        .context("The authorization scheme was not `Basic`.")?;
    let decoded_bytes = base64::decode_config(base64encoded_segment, base64::STANDARD)
        .context("Failed to base64-decode `Basic` credentials.")?;
    let decoded_credentials = String::from_utf8(decoded_bytes)
        .context("The decoded credential string is not valid UTF-8")?;
    let mut credentials = decoded_credentials.splitn(2, ':');
    let username = credentials
        .next()
        .context("A username must be provided in `Basic` auth")?;
    let password = credentials
        .next()
        .context("A password must be provided in `Basic` auth")?;

    Ok(Credentials {
        username: username.to_string(),
        password: Secret::new(password.to_string()),
    })
}

#[tracing::instrument(name = "Validate credentials", skip(username, password, pool))]
async fn validate_credentials(
    Credentials { username, password }: Credentials,
    pool: &PgPool,
) -> Result<i64, NftTokensError> {
    let mut user_id = None;
    // prevent time attack
    let mut password_hash = Secret::new(
        "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
            .to_string(),
    );

    if let Some((stored_user_id, stored_password_hash)) = get_stored_credentials(&username, pool)
        .await
        .map_err(NftTokensError::UnexpectedError)?
    {
        user_id = Some(stored_user_id);
        password_hash = stored_password_hash;
    }

    spawn_blocking_with_tracing(move || verify_password_hash(password_hash, password))
        .await
        .context("Failed to spawn blocking task.")??;

    user_id.ok_or_else(|| NftTokensError::AuthError(anyhow!("Unknown username.")))
}

#[tracing::instrument(
    name = "Verify password hash",
    skip(expected_password_hash, password_candidate)
)]
fn verify_password_hash(
    expected_password_hash: Secret<String>,
    password_candidate: Secret<String>,
) -> Result<(), NftTokensError> {
    let phc_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .context("Failed to parse hash in PHC string format.")
        .map_err(NftTokensError::UnexpectedError)?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &phc_password_hash,
        )
        .context("Invalid password")
        .map_err(NftTokensError::AuthError)
}

#[tracing::instrument(name = "Get stored credentials", skip(username, pool))]
async fn get_stored_credentials(
    username: &str,
    pool: &PgPool,
) -> Result<Option<(i64, Secret<String>)>, anyhow::Error> {
    let row = sqlx::query!(
        "SELECT user_id, password_hash FROM users WHERE username = $1",
        username,
    )
    .fetch_optional(pool)
    .await
    .context("Failed to perform a query to retrieve stored credentials.")?
    .map(|r| (r.user_id, Secret::new(r.password_hash)));

    Ok(row)
}

#[tracing::instrument(name = "Insert nft tokens", skip(nft_token, request, pool))]
pub async fn insert_nft_token(
    web::Json(nft_token): web::Json<NftToken>,
    request: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, NftTokensError> {
    let credentials = basic_authentication(request.headers()).map_err(NftTokensError::AuthError)?;
    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));
    let user_id = validate_credentials(credentials, &pool).await?;
    tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
    let mut tx = pool.begin().await.context("Failed to start transaction.")?;
    store_nft_token(nft_token, &mut tx)
        .await
        .context("Failed to insert the nft token data into the database.")?;
    tx.commit()
        .await
        .context("Failed to commit SQL transaction to store a new subscriber.")?;
    Ok(HttpResponse::Created().finish())
}

#[tracing::instrument(name = "Store nft tokens to database", skip(tx))]
pub async fn store_nft_token(
    nft_token: NftToken,
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

#[tracing::instrument(name = "Query nft tokens from database", skip(filter, pool))]
pub async fn query_nft_tokens(
    pool: web::Data<PgPool>,
    filter: &NftTokenFilter,
) -> Result<Vec<NftToken>, anyhow::Error> {
    let rows= sqlx::query_as!(
        NftToken,
        r#"
        SELECT token_id, owner_id, media, model as "model: Json<ModelKind>", copies, description, expires_at, issued_at, title, media_hash
        FROM nft_tokens
        WHERE ($1::text IS null OR token_id = $1)
            AND ($2::text IS null OR owner_id = $2)
        ORDER BY id LIMIT $3 OFFSET $4
        "#,
        filter.token_id(),
        filter.owner_id(),
        filter.limit() + 1,
        filter.offset(),
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(rows)
}
