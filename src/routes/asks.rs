use crate::domain::{AskFilter, Limit, Offset, Parse, ParseToPositiveInt, TokenId};
use crate::errors::AskError;
use crate::routes::{PaginationQuery, RowsJsonReport};
use actix_web::{web, HttpResponse};
use anyhow::Context;
use battlemon_models::market::ask::{AskForDb, AskForRest};
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
pub async fn get_asks(
    web::Query(filter): web::Query<PaginationQuery>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AskError> {
    let filter = filter.try_into().map_err(AskError::ValidationError)?;
    let asks = get_asks_db(&filter, &pool)
        .await
        .context("Failed to get the ask's data from the database.")?;

    Ok(HttpResponse::Ok().json(RowsJsonReport::from_rows(asks, filter.limit())))
}

#[tracing::instrument(name = "Query asks from database", skip(filter, pool))]
pub async fn get_asks_db(
    filter: &AskFilter,
    pool: &PgPool,
) -> Result<Vec<AskForDb>, anyhow::Error> {
    let rows = sqlx::query_as!(
        AskForDb,
        r#"
        SELECT id, token_id,  account_id, approval_id, price
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
    web::Json(ask): web::Json<AskForRest>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AskError> {
    let mut tx = pool.begin().await.context("Failed to start transaction.")?;
    insert_ask_db(ask, &mut tx)
        .await
        .context("Failed to insert the ask data into the database.")?;
    tx.commit()
        .await
        .context("Failed to commit SQL transaction to store a new subscriber.")?;
    Ok(HttpResponse::Created().finish())
}

#[tracing::instrument(name = "Store ask to database", skip(tx))]
pub async fn insert_ask_db(
    ask: AskForRest,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<(), anyhow::Error> {
    sqlx::query!(
        r#"
        INSERT INTO asks (id, token_id, account_id, approval_id, price)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (token_id) DO NOTHING
        "#,
        ask.id,
        ask.token_id,
        ask.account_id,
        ask.approval_id,
        ask.price,
    )
    .execute(tx)
    .await?;

    Ok(())
}

#[tracing::instrument(name = "Delete ask", skip(ask, pool))]
pub async fn delete_ask(
    web::Json(ask): web::Json<AskForRest>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AskError> {
    let mut tx = pool.begin().await.context("Failed to start transaction.")?;
    delete_ask_db(ask, &mut tx)
        .await
        .context("Failed to remove the ask data from the database.")?;
    tx.commit()
        .await
        .context("Failed to commit SQL transaction to complete removing ask.")?;
    Ok(HttpResponse::Created().finish())
}

#[tracing::instrument(name = "Remove ask from database", skip(tx))]
pub async fn delete_ask_db(
    ask: AskForRest,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<(), anyhow::Error> {
    sqlx::query!(
        r#"
        DELETE FROM asks
        WHERE id = $1 
        "#,
        ask.id,
    )
    .execute(tx)
    .await?;

    Ok(())
}
