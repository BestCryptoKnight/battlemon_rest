use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RowsJsonReport<T> {
    pub rows: Vec<T>,
    pub end: bool,
}

impl<'de, T> RowsJsonReport<T>
where
    T: Serialize + Deserialize<'de>,
{
    fn new(rows: Vec<T>, end: bool) -> Self {
        Self { rows, end }
    }

    pub fn from_rows(mut rows: Vec<T>, limit: i64) -> Self {
        let limit = limit as usize;
        let (rows, end) = if rows.is_empty() || rows.len() <= limit {
            (rows, true)
        } else {
            rows.pop();
            (rows, false)
        };

        Self::new(rows, end)
    }
}
