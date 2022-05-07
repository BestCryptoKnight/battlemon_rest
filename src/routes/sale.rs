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

#[tracing::instrument(name = "Handle sales request", skip(pool), fields(request_id = %uuid::Uuid::new_v4()))]
pub async fn sale(filter: web::Query<PaginationFilter>, pool: web::Data<PgPool>) -> HttpResponse {
    let limit = filter.limit.unwrap_or(100);
    let offset = filter.offset.unwrap_or(0);

    let rows = query_sales(&pool, limit, offset).await;

    match rows {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Query sales from database", skip(pool))]
pub async fn query_sales(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Sale>, sqlx::Error> {
    let sales = sqlx::query_as!(
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

    Ok(sales)
}
