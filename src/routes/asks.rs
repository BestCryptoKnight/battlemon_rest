use crate::domain::{AskFilter, Limit, Offset, Parse, ParseToPositiveInt, TokenId};
use crate::errors::AskError;
use crate::routes::{PaginationQuery, RowsJsonReport};
use actix_web::{web, HttpResponse};
use anyhow::Context;
use battlemon_models::market::ask_contract::Ask;
use sqlx::{PgPool, Postgres, Transaction};

impl TryFrom<PaginationQuery> for AskFilter {
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

#[tracing::instrument(name = "Handle asks request", skip(filter, pool))]
pub async fn ask(
    web::Query(filter): web::Query<PaginationQuery>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AskError> {
    let filter = filter.try_into().map_err(AskError::ValidationError)?;
    let asks = query_asks(&filter, &pool)
        .await
        .context("Failed to get the ask's data from the database.")?;

    Ok(HttpResponse::Ok().json(RowsJsonReport::from_rows(asks, filter.limit())))
}

#[tracing::instrument(name = "Query asks from database", skip(filter, pool))]
pub async fn query_asks(filter: &AskFilter, pool: &PgPool) -> Result<Vec<Ask>, anyhow::Error> {
    let rows = sqlx::query_as!(
        Ask,
        r#"
        SELECT id, token_id, expire_at, account_id, approve_id, price
        FROM asks
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

#[tracing::instrument(name = "Insert ask", skip(ask, pool))]
pub async fn insert_ask(
    web::Json(ask): web::Json<Ask>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AskError> {
    let mut tx = pool.begin().await.context("Failed to start transaction.")?;
    store_ask(ask, &mut tx)
        .await
        .context("Failed to insert the ask data into the database.")?;
    tx.commit()
        .await
        .context("Failed to commit SQL transaction to store a new subscriber.")?;
    Ok(HttpResponse::Created().finish())
}

#[tracing::instrument(name = "Store ask to database", skip(tx))]
pub async fn store_ask(ask: Ask, tx: &mut Transaction<'_, Postgres>) -> Result<(), anyhow::Error> {
    sqlx::query!(
        r#"
        INSERT INTO asks (, curr_owner, token_id, price, date)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        ask.prev_owner,
        ask.curr_owner,
        ask.token_id,
        ask.price,
        Utc::now()
    )
    .execute(tx)
    .await?;

    Ok(())
}
