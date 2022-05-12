pub use health_check::*;
pub use nft_tokens::*;
pub use paid::*;
pub use sale::*;
// todo: add tracing to all routes
mod health_check;
mod nft_tokens;
mod paid;
mod sale;

use serde::Deserialize;
use std::fmt::Debug;

#[derive(Deserialize, Debug)]
pub struct PaginationQuery {
    pub days: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
