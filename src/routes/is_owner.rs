use crate::errors::IsOwnerError;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[tracing::instrument(name = "Checking if user is owner of provided tokens", skip(pool))]
pub async fn is_owner(
    web::Json(tokens): web::Json<Vec<String>>,
    path: web::Path<String>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, IsOwnerError> {
    // todo ddd here (validation token_id, owner)
    let owner_candidate_id = path.into_inner();
    let ret = is_owner_db(&owner_candidate_id, &tokens, &pool).await?;
    let json = serde_json::json!({ "result": ret });
    Ok(HttpResponse::Ok().json(json))
}

#[tracing::instrument(
    name = "Checking that tokens belongs to `owner_candidate_id`",
    skip(pool)
)]
async fn is_owner_db(
    owner_candidate_id: &str,
    tokens: &[String],
    pool: &PgPool,
) -> Result<bool, anyhow::Error> {
    let tokens_ids = sqlx::query!(
        r#"
        SELECT token_id
        FROM nft_tokens
        WHERE owner_id = $1 AND token_id = ANY($2)
        "#,
        owner_candidate_id,
        tokens
    )
    .fetch_all(pool)
    .await?;

    Ok(tokens_ids.len() == tokens.len())
}
