use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    limit: i64,
    offset: i64,
}

#[derive(Serialize)]
struct Sale {
    id: Uuid,
    prev_owner: String,
    curr_owner: String,
    token_id: String,
    price: String,
    date: DateTime<Utc>,
}

pub async fn sales(
    pagination: web::Query<Pagination>,
    pool: web::Data<PgPool>,
) -> actix_web::HttpResponse {
    let rows = sqlx::query_as!(
        Sale,
        r#"
        SELECT * FROM sales ORDER BY id LIMIT $1 OFFSET $2;
        "#,
        pagination.limit,
        pagination.offset
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
