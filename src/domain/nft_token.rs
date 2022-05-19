pub use crate::domain::OwnerId;
use crate::domain::{AsInner, IntoInner, TokenId};

pub struct NftTokenFilter {
    // pub offset:
    // pub limit:
    // pub days:
    pub owner_id: OwnerId,
    pub token_id: TokenId,
    // pub by_token_trait: TokenTrait,
}

impl NftTokenFilter {
    pub fn owner_id(&self) -> Option<&str> {
        self.owner_id.as_inner()
    }
    pub fn token_id(&self) -> Option<&str> {
        self.token_id.as_inner()
    }
}
