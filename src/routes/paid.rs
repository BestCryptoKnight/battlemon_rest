use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;
use serde_json::json;
use sqlx::PgPool;

use crate::routes::{Pagination, Sale};

pub async fn paid(
    pagination: web::Query<Pagination>,
    pool: web::Data<PgPool>,
) -> actix_web::HttpResponse {
    let now = Utc::now();
    let days = pagination.days.unwrap_or(1);
    let start_from = now - Duration::days(days);
    let rows = sqlx::query_as!(
        Sale,
        r#"
        SELECT * FROM sales WHERE date >= $1;
        "#,
        start_from,
    )
    .fetch_all(pool.get_ref())
    .await;

    match rows {
        Ok(rows) => {
            let (top_sale, total_sale_volume) = calculate_report(&rows);
            let json = json!({
                 "history": rows,
                 "statistics": {
                     "total_sale_volume": total_sale_volume,
                     "total_number_of_sales": rows.len(),
                     "top_sale": top_sale,
                 }
            });
            HttpResponse::Ok().json(json)
        }
        Err(e) => {
            println!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
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
