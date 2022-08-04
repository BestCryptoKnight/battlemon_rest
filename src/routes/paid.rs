use actix_web::{web, HttpResponse};
use anyhow::Context;
use chrono::{Duration, Utc};
use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;

use battlemon_models::market::{paid::Paid, sale::SaleForDb};
use sqlx::PgPool;

use crate::domain::{Limit, Offset, PaidDays, PaidFilter, ParseToPositiveInt};
use crate::errors::PaidError;

use super::PaginationQuery;

impl TryFrom<PaginationQuery> for PaidFilter {
    type Error = String;

    fn try_from(query: PaginationQuery) -> Result<Self, Self::Error> {
        let limit = Limit::parse(query.limit)?;
        let offset = Offset::parse(query.offset)?;
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
async fn query_trades(filter: PaidFilter, pool: &PgPool) -> Result<Vec<SaleForDb>, anyhow::Error> {
    let now = Utc::now();
    let start_from = now - Duration::days(filter.days());
    let trades = sqlx::query_as!(
        SaleForDb,
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

fn calculate_report(trades: &[SaleForDb]) -> (Decimal, Decimal) {
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
    use super::*;
    use rust_decimal_macros::dec;

    fn get_sale() -> SaleForDb {
        SaleForDb {
            id: 1,
            prev_owner: "alice.near".to_string(),
            curr_owner: "bob.near".to_string(),
            token_id: "1".to_string(),
            price: Default::default(),
            date: Utc::now(),
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
        let trade = SaleForDb {
            price: dec!(10),
            ..get_sale()
        };

        let rows = vec![trade];
        let (top_trade, total_trades_volume) = calculate_report(&rows);
        assert_eq!(top_trade, dec!(10));
        assert_eq!(total_trades_volume, dec!(10));
    }

    #[test]
    fn calculate_report_two_trades() {
        let trade0 = SaleForDb {
            price: dec!(1),
            ..get_sale()
        };

        let trade1 = SaleForDb {
            price: dec!(10),
            ..get_sale()
        };

        let rows = vec![trade0, trade1];
        let (top_trade, total_trade_volume) = calculate_report(&rows);
        assert_eq!(top_trade, dec!(10));
        assert_eq!(total_trade_volume, dec!(11));
    }

    #[test]
    fn calculate_report_tree_trades() {
        let trade0 = SaleForDb {
            price: dec!(5),
            ..get_sale()
        };

        let trade1 = SaleForDb {
            price: dec!(3),
            ..get_sale()
        };

        let trade2 = SaleForDb {
            price: dec!(1),
            ..get_sale()
        };

        let rows = vec![trade0, trade1, trade2];
        let (top_trade, total_trades_volume) = calculate_report(&rows);
        assert_eq!(top_trade, dec!(5));
        assert_eq!(total_trades_volume, dec!(9));
    }
}
