use crate::errors::AuthError;
use crate::telemetry::spawn_blocking_with_tracing;
use actix_web::http::header::HeaderMap;
use anyhow::Context;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

pub struct Credentials {
    pub username: String,
    pub password: Secret<String>,
}

#[tracing::instrument(name = "Get stored credentials", skip(username, pool))]
pub async fn get_stored_credentials(
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

#[tracing::instrument(
    name = "Verify password hash",
    skip(expected_password_hash, password_candidate)
)]
pub fn verify_password_hash(
    expected_password_hash: Secret<String>,
    password_candidate: Secret<String>,
) -> Result<(), AuthError> {
    let phc_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .context("Failed to parse hash in PHC string format.")
        .map_err(AuthError::UnexpectedError)?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &phc_password_hash,
        )
        .context("Invalid password")
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(name = "Validate credentials", skip(username, password, pool))]
pub async fn validate_credentials(
    Credentials { username, password }: Credentials,
    pool: &PgPool,
) -> Result<i64, AuthError> {
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
        .map_err(AuthError::UnexpectedError)?
    {
        user_id = Some(stored_user_id);
        password_hash = stored_password_hash;
    }

    spawn_blocking_with_tracing(move || verify_password_hash(password_hash, password))
        .await
        .context("Failed to spawn blocking task.")??;

    user_id
        .context("Unknown username.")
        .map_err(AuthError::InvalidCredentials)
}

pub fn basic_auth(headers: &HeaderMap) -> Result<Credentials, AuthError> {
    let header_value = headers
        .get("Authorization")
        .context("The `Authorization` header was missing.")
        .map_err(AuthError::BadRequest)?
        .to_str()
        .context("The `Authorization` header was not a valid UTF-8 string.")
        .map_err(AuthError::BadRequest)?;

    let base64encoded_segment = header_value
        .strip_prefix("Basic ")
        .context("The authorization scheme was not `Basic`.")
        .map_err(AuthError::BadRequest)?;

    let decoded_bytes = base64::decode_config(base64encoded_segment, base64::STANDARD)
        .context("Failed to base64-decode `Basic` credentials.")
        .map_err(AuthError::BadRequest)?;

    let decoded_credentials = String::from_utf8(decoded_bytes)
        .context("The decoded credential string is not valid UTF-8")
        .map_err(AuthError::BadRequest)?;

    let mut credentials = decoded_credentials.splitn(2, ':');
    let username = credentials
        .next()
        .context("A username must be provided in `Basic` auth")
        .map_err(AuthError::BadRequest)?;
    let password = credentials
        .next()
        .context("A password must be provided in `Basic` auth")
        .map_err(AuthError::BadRequest)?;

    Ok(Credentials {
        username: username.to_string(),
        password: Secret::new(password.to_string()),
    })
}
