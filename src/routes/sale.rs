use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub days: Option<i64>,
}

#[derive(Serialize)]
pub struct Sale {
    pub id: Uuid,
    pub prev_owner: String,
    pub curr_owner: String,
    pub token_id: String,
    pub price: i64,
    pub date: DateTime<Utc>,
}

pub async fn sale(
    pagination: web::Query<Pagination>,
    pool: web::Data<PgPool>,
) -> actix_web::HttpResponse {
    let limit = pagination.limit.unwrap_or(100);
    let offset = pagination.offset.unwrap_or_default();

    let rows = sqlx::query_as!(
        Sale,
        r#"
        SELECT * FROM sales ORDER BY id LIMIT $1 OFFSET $2;
        "#,
        limit,
        offset
    )
    .fetch_all(pool.get_ref())
    .await;

    match rows {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
