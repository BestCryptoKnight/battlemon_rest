use battlemon_rest::routes::{RowsJsonReport, Sale};
use fake::{Fake, Faker};

use helpers::{assert_json_error, spawn_app};

mod dummies;
mod helpers;

#[tokio::test]
async fn sales_return_200_and_zero_stored_in_database_token() {
    let app = spawn_app().await;

    let response = app.get_sales("").await;
    assert!(response.status().is_success());
    let actual_sales: RowsJsonReport<Sale> = response.json().await.unwrap();
    assert_eq!(actual_sales.rows.len(), 0);
    assert!(actual_sales.end);
}

#[tokio::test]
async fn sales_return_200_and_one_stored_in_database_token() {
    let app = spawn_app().await;
    let expected_sale: Sale = Faker.fake();
    sqlx::query!(
        r#"
        INSERT INTO sales (id, prev_owner, curr_owner, token_id, price, date)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        expected_sale.id,
        expected_sale.prev_owner,
        expected_sale.curr_owner,
        expected_sale.token_id,
        expected_sale.price,
        expected_sale.date
    )
    .execute(&app.db_pool)
    .await
    .expect("Failed to execute query");

    let response = app.get_sales("").await;
    assert!(response.status().is_success());
    let actual_sales: RowsJsonReport<Sale> = response.json().await.unwrap();
    assert_eq!(actual_sales.rows.len(), 1);
    assert_eq!(actual_sales.rows[0].id, expected_sale.id);
    assert!(actual_sales.end);
}

#[tokio::test]
async fn sales_success_and_returns_200_for_different_valid_queries() {
    let app = spawn_app().await;
    let sales = fake::vec![Sale; 200];
    for sale in sales {
        sqlx::query!(
            r#"
            INSERT INTO sales (prev_owner, curr_owner, token_id, price, date)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            sale.prev_owner,
            sale.curr_owner,
            sale.token_id,
            sale.price,
            sale.date
        )
        .execute(&app.db_pool)
        .await
        .expect("Failed to execute query");
    }

    let queries_and_expectations = [
        ("", 100, false),
        ("offset=2", 100, false),
        ("offset=199", 1, true),
        ("offset=200", 0, true),
        ("offset=150", 50, true),
        ("limit=50", 50, false),
        ("limit=250", 200, true),
        ("limit=0", 0, false),
        ("offset=0&limit=0", 0, false),
        ("offset=10&limit=25", 25, false),
        ("offset=200&limit=10", 0, true),
        ("offset=199&limit=11", 1, true),
    ];

    for (idx, (query, expectation, end)) in queries_and_expectations.iter().enumerate() {
        let response = app.get_sales(query).await;
        assert_eq!(response.status().as_u16(), 200);

        let actual_sales = response.json::<RowsJsonReport<Sale>>().await.unwrap();
        assert_eq!(
            actual_sales.rows.len(),
            *expectation,
            "length of sales not the same. query is: {} with index: {}",
            query,
            idx
        );

        assert_eq!(actual_sales.end, *end);
    }
}

#[tokio::test]
async fn sales_success_with_valid_queries_for_token_id() {
    let app = spawn_app().await;
    let sales_with_id_1 = (0..100).map(|_| {
        let mut sale: Sale = Faker.fake();
        sale.token_id = "1".to_string();
        sale
    });
    let sales_with_id_2 = (0..50).map(|_| {
        let mut sale: Sale = Faker.fake();
        sale.token_id = "2".to_string();
        sale
    });
    let sales_with_id_3 = (0..25).map(|_| {
        let mut sale: Sale = Faker.fake();
        sale.token_id = "3".to_string();
        sale
    });

    let sales: Vec<Sale> = sales_with_id_1
        .chain(sales_with_id_2)
        .chain(sales_with_id_3)
        .collect();

    for sale in sales {
        sqlx::query!(
            r#"
            INSERT INTO sales (prev_owner, curr_owner, token_id, price, date)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            sale.prev_owner,
            sale.curr_owner,
            sale.token_id,
            sale.price,
            sale.date
        )
        .execute(&app.db_pool)
        .await
        .expect("Failed to execute query");
    }

    let queries_and_expectations = [
        ("", 100, false),
        ("token_id=1", 100, true),
        ("token_id=2", 50, true),
        ("token_id=3", 25, true),
        ("token_id=1&limit=10", 10, false),
        ("token_id=2&limit=20", 20, false),
        ("token_id=3&limit=30", 25, true),
        ("token_id=1&offset=10", 90, true),
        ("token_id=3&offset=10&limit=5", 5, false),
    ];

    for (idx, (query, expectation, end)) in queries_and_expectations.iter().enumerate() {
        let response = app.get_sales(query).await;
        assert_eq!(response.status().as_u16(), 200);

        let actual_sales = response.json::<RowsJsonReport<Sale>>().await.unwrap();
        assert_eq!(
            actual_sales.rows.len(),
            *expectation,
            "length of sales not the same. query is: {} with index: {}",
            query,
            idx
        );

        assert_eq!(actual_sales.end, *end);
    }
}

#[tokio::test]
async fn sale_fails_and_return_500_if_there_is_a_fatal_database_error() {
    let app = spawn_app().await;
    sqlx::query!("ALTER TABLE sales DROP COLUMN price;",)
        .execute(&app.db_pool)
        .await
        .unwrap();
    let response = app.get_sales("offset=2&limit=10").await;
    assert_eq!(
        response.status().as_u16(),
        500,
        "The API didn't return 500 Internal Server Error"
    );

    assert_json_error(response).await
}

#[tokio::test]
async fn sale_fails_and_return_400_when_invalid_queries() {
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
        "token_id",
        "token_id=",
        r#"token_id="abc""#,
        r#"token_id="10""#,
        "token_id=-1",
        r#"offset="abc""#,
        r#"offset="10""#,
        "days",
        "days=",
        "days=-1",
        r#"days="abc""#,
        r#"days="10""#,
    ];

    for invalid_query in invalid_queries {
        let response = app.get_sales(invalid_query).await;
        let actual_status = response.status().as_u16();
        assert_eq!(
            actual_status, 400,
            "Actual: {}. Expected: 400. Wrong query is: {}",
            actual_status, invalid_query
        );
        assert_json_error(response).await;
    }
}
