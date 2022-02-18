use chrono::{DateTime, Utc};
use fake::{Dummy, Fake};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Dummy, Deserialize, PartialEq)]
pub struct Sale {
    pub id: Uuid,
    pub prev_owner: String,
    pub curr_owner: String,
    pub token_id: String,
    #[dummy(faker = "1..1000000000")]
    pub price: i64,
    pub date: DateTime<Utc>,
}
