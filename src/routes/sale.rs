use actix_web::{web, HttpRequest, HttpResponse};
use anyhow::Context;
use chrono::Utc;
use fake::faker::{chrono::en::DateTime, lorem::en::Word, number::raw::NumberWithFormat};
use fake::locales::EN;
use fake::{Dummy, Fake, Faker};
use rand::Rng;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};

use crate::domain::{Limit, Offset, Parse, ParseToPositiveInt, SaleDays, SaleFilter, TokenId};
use crate::errors::SaleError;
use crate::routes::RowsJsonReport;

use super::PaginationQuery;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sale {
    pub id: i64,
    pub prev_owner: String,
    pub curr_owner: String,
    pub token_id: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
    pub date: chrono::DateTime<Utc>,
}

impl Dummy<Faker> for Sale {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let scale = rng.gen_range(0..=24);
        let lo = rng.gen();
        let mid = rng.gen();
        let price = Decimal::from_parts(lo, mid, 0, false, scale);
        Self {
            id: Faker.fake(),
            prev_owner: format!("{}.near", Word().fake::<String>()),
            curr_owner: format!("{}.near", Word().fake::<String>()),
            token_id: NumberWithFormat(EN, "^########").fake::<String>(),
            price,
            date: DateTime().fake(),
        }
    }
}

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
pub async fn sale(
    web::Query(filter): web::Query<PaginationQuery>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, SaleError> {
    let filter = filter.try_into().map_err(SaleError::ValidationError)?;
    let sales = query_sales(filter, &pool)
        .await
        .context("Failed to get the sale's data from the database.")?;

    Ok(HttpResponse::Ok().json(RowsJsonReport::from_rows(sales, filter.limit())))
}

#[tracing::instrument(name = "Query sales from database", skip(filter, pool))]
pub async fn query_sales(filter: SaleFilter, pool: &PgPool) -> Result<Vec<Sale>, anyhow::Error> {
    let rows = sqlx::query_as!(
        Sale,
        r#"
        SELECT id, prev_owner, curr_owner, token_id, price, date
        FROM sales ORDER BY id LIMIT $1 OFFSET $2;
        "#,
        filter.limit() + 1,
        filter.offset()
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

#[tracing::instrument(name = "Insert sale", skip(sale, _request, pool))]
pub async fn insert_sale(
    web::Json(sale): web::Json<Sale>,
    _request: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, SaleError> {
    let mut tx = pool.begin().await.context("Failed to start transaction.")?;
    store_sale(sale, &mut tx)
        .await
        .context("Failed to insert the nft token data into the database.")?;
    tx.commit()
        .await
        .context("Failed to commit SQL transaction to store a new subscriber.")?;
    Ok(HttpResponse::Created().finish())
}

#[tracing::instrument(name = "Store sale to database", skip(tx))]
pub async fn store_sale(
    sale: Sale,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<(), anyhow::Error> {
    sqlx::query_as!(
        Sale,
        r#"
        INSERT INTO sales (prev_owner, curr_owner, token_id, price, date)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (token_id) DO NOTHING
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
