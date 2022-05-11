use actix_web::{web, HttpResponse};
use anyhow::Context;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{SaleDays, SaleFilter, SaleLimit, SaleOffset};
use crate::errors::SaleError;

use super::PaginationQuery;

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

impl TryFrom<PaginationQuery> for SaleFilter {
    type Error = String;

    fn try_from(value: PaginationQuery) -> Result<Self, Self::Error> {
        let limit = SaleLimit::parse(value.limit)?;
        let offset = SaleOffset::parse(value.offset)?;
        SaleDays::parse(value.days)?;

        Ok(Self { limit, offset })
    }
}

#[tracing::instrument(name = "Handle sales request", skip(filter, pool))]
pub async fn sale(
    web::Query(filter): web::Query<PaginationQuery>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, SaleError> {
    let filter = filter.try_into().map_err(SaleError::ValidationError)?;
    let sales = query_sales(filter, &pool)
        .await
        .context("Failed to get the sale's data from the database.")?;

    Ok(HttpResponse::Ok().json(sales))
}

#[tracing::instrument(name = "Query sales from database", skip(filter, pool))]
pub async fn query_sales(filter: SaleFilter, pool: &PgPool) -> Result<Vec<Sale>, anyhow::Error> {
    let rows = sqlx::query_as!(
        Sale,
        r#"
        SELECT id, prev_owner, curr_owner, token_id, price, date
        FROM sales ORDER BY id LIMIT $1 OFFSET $2;
        "#,
        filter.limit(),
        filter.offset()
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_pagination_query_into_sale_filter() {
        todo!()
    }
}
