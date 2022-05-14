use actix_web::{web, HttpResponse};
use chrono::{DateTime, Duration, Utc};
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
        total_sale_volume: Decimal,
        total_number_of_sales: usize,
        top_sale: Decimal,
    ) -> Self {
        let statistics = PaidStatistics {
            total_sale_volume,
            total_number_of_sales,
            top_sale,
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
    pub total_sale_volume: Decimal,
    pub total_number_of_sales: usize,
    #[serde(with = "rust_decimal::serde::str")]
    pub top_sale: Decimal,
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
    name = "Get statistics and sales history for last days",
    skip(filter, pool)
)]
pub async fn paid(
    web::Query(filter): web::Query<PaginationQuery>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PaidError> {
    let filter = filter.try_into().map_err(PaidError::ValidationError)?;
    let sales = query_sales(filter, &pool).await?;

    let (top_sale, total_sale_volume) = calculate_report(&sales);
    let total_number_of_sales = sales.len();
    let paid_json = Paid::new(sales, total_sale_volume, total_number_of_sales, top_sale);
    Ok(HttpResponse::Ok().json(paid_json))
}

#[tracing::instrument(name = "Query sales for last days from database", skip(pool))]
async fn query_sales(filter: PaidFilter, pool: &PgPool) -> Result<Vec<Sale>, anyhow::Error> {
    let now = Utc::now();
    let start_from = now - Duration::days(filter.days());
    let sales = sqlx::query_as!(
        Sale,
        r#"
        SELECT id, prev_owner, curr_owner, token_id, price, date
        FROM sales WHERE date >= $1;
        "#,
        start_from,
    )
    .fetch_all(pool)
    .await?;

    Ok(sales)
}

fn calculate_report(sales: &[Sale]) -> (Decimal, Decimal) {
    let mut top_sale = Decimal::zero();
    let mut total_sale_volume = Decimal::zero();
    for row in sales {
        if row.price > top_sale {
            top_sale = row.price
        }

        total_sale_volume += row.price
    }

    (top_sale, total_sale_volume)
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
        let (top_sale, total_sale_volume) = calculate_report(&rows);
        assert_eq!(top_sale, Decimal::zero());
        assert_eq!(total_sale_volume, Decimal::zero());
    }

    #[test]
    fn calculate_report_one_sale() {
        let sale = Sale {
            price: dec!(10),
            ..Sale::default()
        };

        let rows = vec![sale];
        let (top_sale, total_sale_volume) = calculate_report(&rows);
        assert_eq!(top_sale, dec!(10));
        assert_eq!(total_sale_volume, dec!(10));
    }

    #[test]
    fn calculate_report_two_sale() {
        let sale0 = Sale {
            price: dec!(1),
            ..Sale::default()
        };

        let sale1 = Sale {
            price: dec!(10),
            ..Sale::default()
        };

        let rows = vec![sale0, sale1];
        let (top_sale, total_sale_volume) = calculate_report(&rows);
        assert_eq!(top_sale, dec!(10));
        assert_eq!(total_sale_volume, dec!(11));
    }

    #[test]
    fn calculate_report_tree_sale() {
        let sale0 = Sale {
            price: dec!(5),
            ..Sale::default()
        };

        let sale1 = Sale {
            price: dec!(3),
            ..Sale::default()
        };

        let sale2 = Sale {
            price: dec!(1),
            ..Sale::default()
        };

        let rows = vec![sale0, sale1, sale2];
        let (top_sale, total_sale_volume) = calculate_report(&rows);
        assert_eq!(top_sale, dec!(5));
        assert_eq!(total_sale_volume, dec!(9));
    }
}
