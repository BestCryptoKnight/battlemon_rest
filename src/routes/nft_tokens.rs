use crate::routes::Pagination;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

pub async fn nft_tokens(
    pagination: web::Query<Pagination>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let limit = pagination.limit.unwrap_or(100);
    let offset = pagination.offset.unwrap_or_default();

    let rows = sqlx::query_as!(
        TokenExt,
        r#"
        SELECT * FROM nft_tokens ORDER BY id LIMIT $1 OFFSET $2;
        "#,
        limit,
        offset
    )
    .fetch_all(pool.get_ref())
    .await;

    match rows {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(e) => {
            println!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
