use actix_web::{web, HttpResponse};
use anyhow::Context;
use battlemon_models::market::{sale::SaleForDb, sale::SaleForRest};
use chrono::Utc;
use sqlx::{PgPool, Postgres, Transaction};

use crate::domain::{Limit, Offset, Parse, ParseToPositiveInt, SaleDays, SaleFilter, TokenId};
use crate::errors::SaleError;
use crate::routes::RowsJsonReport;

use super::PaginationQuery;

impl TryFrom<PaginationQuery> for SaleFilter {
    type Error = String;

    fn try_from(query: PaginationQuery) -> Result<Self, Self::Error> {
        let token_id = TokenId::parse(query.token_id)?;
        let limit = Limit::parse(query.limit)?;
        let offset = Offset::parse(query.offset)?;
        SaleDays::parse(query.days)?;

        Ok(Self {
            limit,
            offset,
            token_id,
        })
    }
}

#[tracing::instrument(name = "Handle sales request", skip(filter, pool))]
pub async fn get_sales(
    web::Query(filter): web::Query<PaginationQuery>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, SaleError> {
    let filter = filter.try_into().map_err(SaleError::ValidationError)?;
    let sales = get_sales_db(&filter, &pool)
        .await
        .context("Failed to get the sale's data from the database.")?;

    Ok(HttpResponse::Ok().json(RowsJsonReport::from_rows(sales, filter.limit())))
}

#[tracing::instrument(name = "Query sales from database", skip(filter, pool))]
pub async fn get_sales_db(
    filter: &SaleFilter,
    pool: &PgPool,
) -> Result<Vec<SaleForDb>, anyhow::Error> {
    let rows = sqlx::query_as!(
        SaleForDb,
        r#"
        SELECT id, prev_owner, curr_owner, token_id, price, date
        FROM sales
        WHERE ($1::text IS null OR token_id = $1)
        ORDER BY id LIMIT $2 OFFSET $3;
        "#,
        filter.token_id(),
        filter.limit() + 1,
        filter.offset()
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

#[tracing::instrument(name = "Insert sale", skip(pool))]
pub async fn insert_sale(
    web::Json(sale): web::Json<SaleForRest>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, SaleError> {
    let mut tx = pool.begin().await.context("Failed to start transaction.")?;
    insert_sale_db(sale, &mut tx)
        .await
        .context("Failed to insert the nft token data into the database.")?;
    tx.commit()
        .await
        .context("Failed to commit SQL transaction to store a new subscriber.")?;
    Ok(HttpResponse::Created().finish())
}

#[tracing::instrument(name = "Store sale to database", skip(tx))]
pub async fn insert_sale_db(
    sale: SaleForRest,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<(), anyhow::Error> {
    sqlx::query!(
        r#"
        INSERT INTO sales (prev_owner, curr_owner, token_id, price, date)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        sale.prev_owner,
        sale.curr_owner,
        sale.token_id,
        sale.price,
        Utc::now()
    )
    .execute(tx)
    .await?;

    Ok(())
}
