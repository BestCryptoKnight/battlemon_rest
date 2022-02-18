use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use sqlx::PgPool;

use crate::routes::{Pagination, Sale};

pub async fn paid(
    pagination: web::Query<Pagination>,
    pool: web::Data<PgPool>,
) -> actix_web::HttpResponse {
    let now = Utc::now();
    let days = pagination.days.unwrap_or_default();
    let start_from = now - Duration::days(days);
    let rows = sqlx::query_as!(
        Sale,
        r#"
        SELECT * FROM sales WHERE date >= $1;
        "#,
        start_from,
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
