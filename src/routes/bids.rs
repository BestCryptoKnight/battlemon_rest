use crate::domain::{BidFilter, Limit, Offset, Parse, ParseToPositiveInt, TokenId};
use crate::errors::BidError;
use crate::routes::{PaginationQuery, RowsJsonReport};
use actix_web::{web, HttpResponse};
use anyhow::Context;
use battlemon_models::market::bid::{BidForDb, BidForRest};
use sqlx::{PgPool, Postgres, Transaction};

impl TryFrom<PaginationQuery> for BidFilter {
    type Error = String;

    fn try_from(query: PaginationQuery) -> Result<Self, Self::Error> {
        let token_id = TokenId::parse(query.token_id)?;
        let limit = Limit::parse(query.limit)?;
        let offset = Offset::parse(query.offset)?;

        Ok(Self {
            limit,
            offset,
            token_id,
        })
    }
}

#[tracing::instrument(name = "Handle bids request", skip(filter, pool))]
pub async fn get_bids(
    web::Query(filter): web::Query<PaginationQuery>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, BidError> {
    let filter = filter.try_into().map_err(BidError::ValidationError)?;
    let bids = get_bids_db(&filter, &pool)
        .await
        .context("Failed to get the bid's data from the database.")?;

    Ok(HttpResponse::Ok().json(RowsJsonReport::from_rows(bids, filter.limit())))
}

#[tracing::instrument(name = "Query bids from database", skip(filter, pool))]
pub async fn get_bids_db(
    filter: &BidFilter,
    pool: &PgPool,
) -> Result<Vec<BidForDb>, anyhow::Error> {
    let rows = sqlx::query_as!(
        BidForDb,
        r#"
        SELECT id, token_id,  account_id, expire_at, create_at, price
        FROM bids
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

#[tracing::instrument(name = "Insert bid", skip(pool))]
pub async fn insert_bid(
    web::Json(bid): web::Json<BidForRest>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, BidError> {
    let mut tx = pool.begin().await.context("Failed to start transaction.")?;
    insert_bid_db(bid, &mut tx)
        .await
        .context("Failed to insert the bid data into the database.")?;
    tx.commit()
        .await
        .context("Failed to commit SQL transaction to store a new subscriber.")?;
    Ok(HttpResponse::Created().finish())
}

#[tracing::instrument(name = "Store bid to database", skip(tx))]
pub async fn insert_bid_db(
    bid: BidForRest,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<(), anyhow::Error> {
    sqlx::query!(
        r#"
        INSERT INTO bids (id, token_id, account_id, expire_at, create_at, price)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (token_id) DO NOTHING
        "#,
        bid.id,
        bid.token_id,
        bid.account_id,
        bid.expire_at,
        bid.create_at,
        bid.price,
    )
    .execute(tx)
    .await?;

    Ok(())
}

#[tracing::instrument(name = "Delete bid", skip(bid, pool))]
pub async fn delete_bid(
    web::Json(bid): web::Json<BidForRest>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, BidError> {
    let mut tx = pool.begin().await.context("Failed to start transaction.")?;
    delete_bid_db(bid, &mut tx)
        .await
        .context("Failed to remove the bid data from the database.")?;
    tx.commit()
        .await
        .context("Failed to commit SQL transaction to complete removing bid.")?;
    Ok(HttpResponse::Created().finish())
}

#[tracing::instrument(name = "Remove bid from database", skip(tx))]
pub async fn delete_bid_db(
    bid: BidForRest,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<(), anyhow::Error> {
    sqlx::query!(
        r#"
        DELETE FROM bids
        WHERE id = $1 
        "#,
        bid.id,
    )
    .execute(tx)
    .await?;

    Ok(())
}
