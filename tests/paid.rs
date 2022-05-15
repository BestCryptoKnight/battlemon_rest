use chrono::Utc;
use fake::{Fake, Faker};

use battlemon_rest::routes::Paid;
use helpers::spawn_app;

mod dummies;
mod helpers;

#[tokio::test]
async fn paid_return_200_and_1_sale_in_history_stored_in_database() {
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

    let response = app.get_paid("days=1").await;
    assert!(response.status().is_success());
    let actual_sales = response
        .json::<Paid>()
        .await
        .expect("Couldn't deserialize response. Response must contain serialized `Paid` struct");
    assert_eq!(actual_sales.history.len(), 1);
}
