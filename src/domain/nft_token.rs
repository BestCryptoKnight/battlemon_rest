use crate::domain::{AsInner, Limit, NftTokenOwnerId, Offset, TokenId};
use battlemon_models::nft::NftKind;

#[derive(Debug, Clone)]
pub struct NftTokenFilter {
    pub offset: Offset,
    pub limit: Limit,
    pub owner_id: NftTokenOwnerId,
    pub token_id: TokenId,
    pub nft_kind: Option<String>,
    // pub by_token_trait: TokenTrait,
}

impl NftTokenFilter {
    pub fn limit(&self) -> i64 {
        self.limit.get()
    }

    pub fn offset(&self) -> i64 {
        self.offset.get()
    }

    pub fn owner_id(&self) -> Option<&str> {
        self.owner_id.as_inner()
    }

    pub fn token_id(&self) -> Option<&str> {
        self.token_id.as_inner()
    }

    pub fn nft_kind(&self) -> Option<&str> {
        self.nft_kind.as_deref()
    }
}
