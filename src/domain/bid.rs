use crate::domain::AsInner;

#[derive(Debug, Clone)]
pub struct BidFilter {
    pub limit: crate::domain::Limit,
    pub offset: crate::domain::Offset,
    pub token_id: crate::domain::TokenId,
}

impl BidFilter {
    pub fn limit(&self) -> i64 {
        self.limit.get()
    }

    pub fn offset(&self) -> i64 {
        self.offset.get()
    }

    pub fn token_id(&self) -> Option<&str> {
        self.token_id.as_inner()
    }
}
