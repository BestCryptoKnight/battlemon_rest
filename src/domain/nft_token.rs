pub use crate::domain::NftTokenOwnerId;
use crate::domain::{AsInner, IntoInner, NftTokenLimit, NftTokenTokenId};

pub struct NftTokenFilter {
    // pub offset:
    pub limit: NftTokenLimit,
    // pub days:
    pub owner_id: NftTokenOwnerId,
    pub token_id: NftTokenTokenId,
    // pub by_token_trait: TokenTrait,
}

impl NftTokenFilter {
    pub fn limit(&self) -> i64 {
        self.limit.get()
    }

    pub fn owner_id(&self) -> Option<&str> {
        self.owner_id.as_inner()
    }

    pub fn token_id(&self) -> Option<&str> {
        self.token_id.as_inner()
    }
}
