use crate::filter::PaginationFilter;
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Duration, Utc};
use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::routes::Sale;

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

#[tracing::instrument(name = "Get statistics and sales history for last days", skip(pool))]
pub async fn paid(filter: web::Query<PaginationFilter>, pool: web::Data<PgPool>) -> HttpResponse {
    let now = Utc::now();
    let days = filter.days.unwrap_or(1);
    let start_from = now - Duration::days(days);
    let rows = query_sales(start_from, &pool).await;

    match rows {
        Ok(rows) => {
            let (top_sale, total_sale_volume) = calculate_report(&rows);
            let total_number_of_sales = rows.len();
            let paid_json = Paid::new(rows, total_sale_volume, total_number_of_sales, top_sale);
            HttpResponse::Ok().json(paid_json)
        }
        Err(e) => {
            println!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Query sales for last days from database", skip(pool))]
async fn query_sales(start_from: DateTime<Utc>, pool: &PgPool) -> Result<Vec<Sale>, sqlx::Error> {
    let sales = sqlx::query_as!(
        Sale,
        r#"
        SELECT * FROM sales WHERE date >= $1;
        "#,
        start_from,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(sales)
}

fn calculate_report(rows: &[Sale]) -> (Decimal, Decimal) {
    let mut top_sale = Decimal::zero();
    let mut total_sale_volume = Decimal::zero();
    for row in rows {
        if row.price > top_sale {
            top_sale = row.price
        }

        total_sale_volume += row.price
    }

    (top_sale, total_sale_volume)
}

#[cfg(test)]
mod test {
    use super::*;
    use rust_decimal_macros::dec;

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
