use actix_web::{web, HttpResponse};
use anyhow::Context;
use chrono::{Duration, Utc};
use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::domain::{PaidDays, PaidFilter, PaidLimit, PaidOffset, ParseToPositiveInt};
use crate::errors::PaidError;
use crate::routes::Sale;

use super::PaginationQuery;

#[derive(Serialize, Deserialize)]
pub struct Paid {
    pub history: Vec<Sale>,
    pub statistics: PaidStatistics,
}

impl Paid {
    fn new(
        history: Vec<Sale>,
        total_trades_volume: Decimal,
        total_number_of_trades: usize,
        top_trade: Decimal,
    ) -> Self {
        let statistics = PaidStatistics {
            total_trades_volume,
            total_number_of_trades,
            top_trade,
        };

        Self {
            history,
            statistics,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PaidStatistics {
    #[serde(with = "rust_decimal::serde::str")]
    pub total_trades_volume: Decimal,
    pub total_number_of_trades: usize,
    #[serde(with = "rust_decimal::serde::str")]
    pub top_trade: Decimal,
}

impl TryFrom<PaginationQuery> for PaidFilter {
    type Error = String;

    fn try_from(query: PaginationQuery) -> Result<Self, Self::Error> {
        let limit = PaidLimit::parse(query.limit)?;
        let offset = PaidOffset::parse(query.offset)?;
        let days = PaidDays::parse(query.days)?;

        Ok(Self {
            limit,
            offset,
            days,
        })
    }
}

#[tracing::instrument(
    name = "Get statistics and trades history for last days",
    skip(filter, pool)
)]
pub async fn paid(
    web::Query(filter): web::Query<PaginationQuery>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PaidError> {
    let filter = filter.try_into().map_err(PaidError::ValidationError)?;
    let trades = query_trades(filter, &pool)
        .await
        .context("Failed to get sale's data from the database.")?;

    let (top_trade, total_trade_volume) = calculate_report(&trades);
    let trades_number = trades.len();
    let paid_json = Paid::new(trades, total_trade_volume, trades_number, top_trade);
    Ok(HttpResponse::Ok().json(paid_json))
}

#[tracing::instrument(name = "Query trades for last days from database", skip(pool))]
async fn query_trades(filter: PaidFilter, pool: &PgPool) -> Result<Vec<Sale>, anyhow::Error> {
    let now = Utc::now();
    let start_from = now - Duration::days(filter.days());
    let trades = sqlx::query_as!(
        Sale,
        r#"
        SELECT id, prev_owner, curr_owner, token_id, price, date
        FROM sales WHERE date >= $1 ORDER BY date OFFSET $2 LIMIT $3;
        "#,
        start_from,
        filter.offset(),
        filter.limit(),
    )
    .fetch_all(pool)
    .await?;

    Ok(trades)
}

fn calculate_report(trades: &[Sale]) -> (Decimal, Decimal) {
    let mut top_trade = Decimal::zero();
    let mut total_trade_volume = Decimal::zero();
    for trade in trades {
        if trade.price > top_trade {
            top_trade = trade.price
        }

        total_trade_volume += trade.price
    }

    (top_trade, total_trade_volume)
}

#[cfg(test)]
mod test {
    use rust_decimal_macros::dec;

    use super::*;

    impl Default for Sale {
        fn default() -> Self {
            Sale {
                id: Default::default(),
                prev_owner: Default::default(),
                curr_owner: Default::default(),
                token_id: Default::default(),
                price: Default::default(),
                date: Utc::now(),
            }
        }
    }

    #[test]
    fn calculate_report_empty() {
        let rows = vec![];
        let (top_trade, total_trades_volume) = calculate_report(&rows);
        assert_eq!(top_trade, Decimal::zero());
        assert_eq!(total_trades_volume, Decimal::zero());
    }

    #[test]
    fn calculate_report_one_trade() {
        let trade = Sale {
            price: dec!(10),
            ..Sale::default()
        };

        let rows = vec![trade];
        let (top_trade, total_trades_volume) = calculate_report(&rows);
        assert_eq!(top_trade, dec!(10));
        assert_eq!(total_trades_volume, dec!(10));
    }

    #[test]
    fn calculate_report_two_trades() {
        let trade0 = Sale {
            price: dec!(1),
            ..Sale::default()
        };

        let trade1 = Sale {
            price: dec!(10),
            ..Sale::default()
        };

        let rows = vec![trade0, trade1];
        let (top_trade, total_trade_volume) = calculate_report(&rows);
        assert_eq!(top_trade, dec!(10));
        assert_eq!(total_trade_volume, dec!(11));
    }

    #[test]
    fn calculate_report_tree_trades() {
        let trade0 = Sale {
            price: dec!(5),
            ..Sale::default()
        };

        let trade1 = Sale {
            price: dec!(3),
            ..Sale::default()
        };

        let trade2 = Sale {
            price: dec!(1),
            ..Sale::default()
        };

        let rows = vec![trade0, trade1, trade2];
        let (top_trade, total_trades_volume) = calculate_report(&rows);
        assert_eq!(top_trade, dec!(5));
        assert_eq!(total_trades_volume, dec!(9));
    }
}
