use crate::errors::ContractError;
use actix_web::{web, HttpResponse};
use anyhow::Context;
use battlemon_models::config::ContractConfig;
use sqlx::{types::Json, PgPool, Postgres, Transaction};

#[tracing::instrument(name = "List contracts ids", skip(pool))]
pub async fn contracts(pool: web::Data<PgPool>) -> Result<HttpResponse, ContractError> {
    let contracts = get_contracts(pool)
        .await
        .context("Failed to get the contracts id data from database.")?;

    Ok(HttpResponse::Ok().json(contracts))
}

struct Record {
    contracts_config: Json<ContractConfig>,
}

#[tracing::instrument(name = "Query contracts ids from database", skip(pool))]
pub async fn get_contracts(pool: web::Data<PgPool>) -> Result<ContractConfig, anyhow::Error> {
    let record = sqlx::query_as!(
        Record,
        r#"
        SELECT contracts_config as "contracts_config: Json<ContractConfig>" FROM contracts
        "#,
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(record.contracts_config.0)
}

#[tracing::instrument(name = "Upsert contract ids", skip(pool))]
pub async fn insert_contracts(
    web::Json(contract_config): web::Json<ContractConfig>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ContractError> {
    let mut tx = pool.begin().await.context("Failed to start transaction.")?;
    upsert_contracts_db(contract_config, &mut tx)
        .await
        .context("Failed to upsert the contracts ids data into the database.")?;
    tx.commit()
        .await
        .context("Failed to commit SQL transaction to store a new subscriber.")?;
    Ok(HttpResponse::Created().finish())
}

#[tracing::instrument(name = "Store or update contracts ids to database", skip(tx))]
pub async fn upsert_contracts_db(
    contracts_config: ContractConfig,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<(), anyhow::Error> {
    sqlx::query_as!(
        ContractConfig,
        r#"
        INSERT INTO contracts (contracts_config)
        VALUES ($1)
        ON CONFLICT (contracts_config) DO UPDATE
        SET contracts_config = $1
        "#,
        Json(contracts_config) as _,
    )
    .execute(tx)
    .await?;

    Ok(())
}
