use crate::filter::PaginationFilter;
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Sale {
    pub id: Uuid,
    pub prev_owner: String,
    pub curr_owner: String,
    pub token_id: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
    pub date: DateTime<Utc>,
}

#[tracing::instrument(name = "Handle sales request", skip(pool))]
pub async fn sale(filter: web::Query<PaginationFilter>, pool: web::Data<PgPool>) -> HttpResponse {
    let limit = filter.limit.unwrap_or(100);
    let offset = filter.offset.unwrap_or(0);

    let query_result = query_sales(limit, offset, &pool).await;

    match query_result {
        Ok(sales) => HttpResponse::Ok().json(sales),
        Err(e) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Query sales from database", skip(pool))]
pub async fn query_sales(limit: i64, offset: i64, pool: &PgPool) -> Result<Vec<Sale>, sqlx::Error> {
    let rows = sqlx::query_as!(
        Sale,
        r#"
        SELECT * FROM sales ORDER BY id LIMIT $1 OFFSET $2;
        "#,
        limit,
        offset
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(rows)
}
