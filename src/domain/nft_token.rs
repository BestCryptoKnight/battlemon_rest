use crate::domain::TokenId;
pub use crate::domain::UserId;

pub struct NftTokenFilter {
    // pub offset:
    // pub limit:
    // pub days:
    pub by_user_id: UserId,
    pub by_token_id: TokenId,
    // pub by_token_trait: TokenTrait,
}
