use serde::Deserialize;

pub use health_check::*;
pub use nft_tokens::*;
pub use paid::*;
pub use sale::*;

mod health_check;
mod nft_tokens;
mod paid;
mod sale;

#[derive(Deserialize, Debug)]
pub struct PaginationQuery {
    pub days: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
