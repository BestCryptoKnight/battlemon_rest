use chrono::{Duration, Utc};
use fake::{Fake, Faker};

use battlemon_models::market::{Paid, SaleForInserting};
use helpers::spawn_app;

use crate::helpers::assert_json_error;

mod dummies;
mod helpers;

#[tokio::test]
async fn paid_return_200_and_1_sale_in_history_stored_in_database() {
    let app = spawn_app().await;
    let expected_sale: SaleForInserting = Faker.fake();
    sqlx::query!(
        r#"
        INSERT INTO sales (prev_owner, curr_owner, token_id, price, date)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        expected_sale.prev_owner,
        expected_sale.curr_owner,
        expected_sale.token_id,
        expected_sale.price,
        Utc::now()
    )
    .execute(&app.db_pool)
    .await
    .expect("Failed to execute query");

    let response = app.get_paid("days=1").await;
    assert!(response.status().is_success());
    let actual_sales = response
        .json::<Paid>()
        .await
        .expect("Couldn't deserialize response. Response must contain serialized `Paid` struct");
    let actual_trades_number = actual_sales.history.len();
    assert_eq!(
        actual_trades_number, 1,
        "The length of history for trades doesn't equal 1, actual length is {}",
        actual_trades_number
    );
}

#[tokio::test]
async fn paid_success_and_returns_200_for_different_valid_queries() {
    let app = spawn_app().await;
    let sales = fake::vec![SaleForInserting; 200];
    for (idx, sale) in sales.iter().enumerate() {
        let date = if idx < 100 {
            Utc::now()
        } else {
            Utc::now() - Duration::days(1)
        };

        sqlx::query!(
            r#"
            INSERT INTO sales (prev_owner, curr_owner, token_id, price, date)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            sale.prev_owner,
            sale.curr_owner,
            sale.token_id,
            sale.price,
            date
        )
        .execute(&app.db_pool)
        .await
        .expect("Failed to execute query");
    }

    let queries_and_expectations = [
        ("", 100),
        ("offset=2", 98),
        ("offset=99", 1),
        ("offset=100", 0),
        ("offset=110", 0),
        ("limit=50", 50),
        ("limit=250", 100),
        ("limit=0", 0),
        ("days=0", 0),
        ("days=1", 100),
        ("days=2", 100),
        ("days=1&offset=2", 98),
        ("days=1&offset=5", 95),
        ("days=1&limit=200", 100),
        ("days=2&limit=200", 200),
        ("days=2&limit=150", 150),
        ("offset=0&limit=0", 0),
        ("offset=10&limit=25", 25),
        ("offset=100&limit=10", 0),
        ("offset=99&limit=11", 1),
        ("days=1&offset=2&limit=0", 0),
        ("days=1&offset=5&limit=90", 90),
        ("days=1&limit=250&offset=10", 90),
        ("days=2&limit=200&offset=100", 100),
        ("days=2&limit=50&offset=100", 50),
    ];
    for (idx, (query, expectation)) in queries_and_expectations.iter().enumerate() {
        let response = app.get_paid(query).await;
        assert_eq!(response.status().as_u16(), 200);
        let actual_paid = response.json::<Paid>().await.unwrap();
        let actual_trades_number = actual_paid.history.len();
        assert_eq!(
            actual_trades_number,
            *expectation,
            "Error was occurred for query `{}` with idx `{}`.\nThe length of history for trades doesn't equal `{}`, actual length is `{}`",
            query,
            idx,
            expectation,
            actual_trades_number
        );
    }
}

#[tokio::test]
async fn paid_fails_and_return_500_if_there_is_a_fatal_database_error() {
    let app = spawn_app().await;
    sqlx::query!("ALTER TABLE sales DROP COLUMN price;",)
        .execute(&app.db_pool)
        .await
        .unwrap();
    let response = app.get_paid("days=1&offset=2&limit=10").await;
    assert_eq!(
        response.status().as_u16(),
        500,
        "The API didn't return 500 Internal Server Error"
    );

    assert_json_error(response).await
}

#[tokio::test]
async fn paid_fails_and_return_400_when_invalid_queries() {
    let app = spawn_app().await;

    let invalid_queries = [
        "limit",
        "limit=",
        "limit=-1",
        r#"limit="abc""#,
        r#"limit="10""#,
        "offset",
        "offset=",
        "offset=-1",
        r#"offset="abc""#,
        r#"offset="10""#,
        "days",
        "days=",
        "days=-1",
        r#"days="abc""#,
        r#"days="10""#,
    ];

    for invalid_query in invalid_queries {
        let response = app.get_paid(invalid_query).await;
        let actual_status = response.status().as_u16();
        assert_eq!(
            actual_status, 400,
            "Actual: {}. Expected: 400. Wrong query is: {}",
            actual_status, invalid_query
        );
        assert_json_error(response).await;
    }
}
