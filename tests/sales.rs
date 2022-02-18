use fake::{Fake, Faker};

use utils::spawn_app;

mod dummies;
mod utils;

#[tokio::test]
async fn sale() {
    let app = spawn_app().await;
    let expected_sale: dummies::Sale = Faker.fake();
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

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/sales", app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    let actual_sales = response.json::<Vec<dummies::Sale>>().await.unwrap();
    assert_eq!(actual_sales.len(), 1);
    assert_eq!(actual_sales[0].id, expected_sale.id);
}

#[tokio::test]
async fn sales() {
    let app = spawn_app().await;
    let sales = fake::vec![dummies::Sale; 200];
    for sale in sales {
        sqlx::query!(
            r#"
        INSERT INTO sales (id, prev_owner, curr_owner, token_id, price, date)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
            sale.id,
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

    let client = reqwest::Client::new();
    let queries_and_expectations = [
        ("/sales", 100),
        ("/sales?offset=2", 100),
        ("/sales?limit=50", 50),
        ("/sales?limit=198", 198),
        ("/sales?offset=10&limit=25", 25),
    ];
    for (query, expectation) in queries_and_expectations {
        let response = client
            .get(&format!("{}{}", app.address, query))
            .send()
            .await
            .expect("Failed to execute request");

        assert!(response.status().is_success());
        let actual_sales = response.json::<Vec<dummies::Sale>>().await.unwrap();
        assert_eq!(
            actual_sales.len(),
            expectation,
            "length of sales not the same of query {}",
            query
        );
    }
}
