use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use serde_json::json;
use sqlx::PgPool;

use crate::routes::{Pagination, Sale};

pub async fn paid(
    pagination: web::Query<Pagination>,
    pool: web::Data<PgPool>,
) -> actix_web::HttpResponse {
    let now = Utc::now();
    let days = pagination.days.unwrap_or_default();
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

fn calculate_report(rows: &[Sale]) -> (u64, u64) {
    let mut top_sale = 0;
    let mut total_sale_volume = 0;
    for row in rows {
        if row.price > top_sale {
            top_sale = row.price
        }

        total_sale_volume += row.price
    }

    (top_sale as u64, total_sale_volume as u64)
}

#[cfg(test)]
mod test {
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
        assert_eq!(top_sale, 0);
        assert_eq!(total_sale_volume, 0);
    }

    #[test]
    fn calculate_report_one_sale() {
        let sale = Sale {
            price: 10,
            ..Sale::default()
        };

        let rows = vec![sale];
        let (top_sale, total_sale_volume) = calculate_report(&rows);
        assert_eq!(top_sale, 10);
        assert_eq!(total_sale_volume, 10);
    }

    #[test]
    fn calculate_report_two_sale() {
        let sale0 = Sale {
            price: 1,
            ..Sale::default()
        };

        let sale1 = Sale {
            price: 10,
            ..Sale::default()
        };

        let rows = vec![sale0, sale1];
        let (top_sale, total_sale_volume) = calculate_report(&rows);
        assert_eq!(top_sale, 10);
        assert_eq!(total_sale_volume, 11);
    }

    #[test]
    fn calculate_report_tree_sale() {
        let sale0 = Sale {
            price: 5,
            ..Sale::default()
        };

        let sale1 = Sale {
            price: 3,
            ..Sale::default()
        };

        let sale2 = Sale {
            price: 1,
            ..Sale::default()
        };

        let rows = vec![sale0, sale1, sale2];
        let (top_sale, total_sale_volume) = calculate_report(&rows);
        assert_eq!(top_sale, 5);
        assert_eq!(total_sale_volume, 9);
    }
}
