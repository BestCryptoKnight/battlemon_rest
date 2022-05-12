use battlemon_rest::routes::Paid;
use chrono::Utc;
use fake::{Fake, Faker};

use utils::spawn_app;

mod dummies;
mod utils;

#[tokio::test]
async fn paid() {
    let app = spawn_app().await;
    let mut expected_sale: dummies::Sale = Faker.fake();
    expected_sale.date = Utc::now();
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
        .get(&format!("{}/paid?days=1", app.address))
        .send()
        .await
        .expect("Failed to execute request");
    //todo: this test sometimes is failed. may be because something wrong with calculation
    assert!(response.status().is_success());
    let actual_sales = response
        .json::<Paid>()
        .await
        .expect("Couldn't deserialize response");
    assert_eq!(actual_sales.history.len(), 1);
}
