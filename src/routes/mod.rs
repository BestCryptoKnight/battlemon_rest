use serde::Deserialize;

pub use health_check::*;
pub use nft_tokens::*;
pub use paid::*;
pub use sale::*;

mod health_check;
mod nft_tokens;
mod paid;
mod sale;

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct PaginationQuery {
    pub days: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

fn build_report_for_rows<T>(rows: &[T], limit: i64) -> (&[T], bool) {
    let limit = limit as usize;
    if rows.is_empty() || rows.len() <= limit {
        (rows, true)
    } else {
        (&rows[..limit], false)
    }
}
