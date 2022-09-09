use fake::faker::address::en::Geohash;

use battlemon_models::nft::{FromTraitWeights, Lemon, ModelKind};
use fake::{Dummy, Fake};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct NftToken {
    pub owner_id: String,
    pub token_id: String,
    pub media: String,
    pub model: ModelKind,
}

pub struct AliceNftToken;

impl Dummy<AliceNftToken> for NftToken {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &AliceNftToken, rng: &mut R) -> Self {
        let traits_config = fake::vec![u8; 5]
            .try_into()
            .expect("Couldn't convert to the array with length 5");
        let model = ModelKind::Lemon(Lemon::from_trait_weights(&"".to_string(), &traits_config));
        let token_id = rng.gen::<u64>().to_string();

        Self {
            owner_id: "alice.near".to_string(),
            token_id,
            media: Geohash(24).fake(),
            model,
        }
    }
}

pub struct BobNftToken;

impl Dummy<BobNftToken> for NftToken {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &BobNftToken, _rng: &mut R) -> Self {
        let alice_token: NftToken = AliceNftToken.fake();
        Self {
            owner_id: "bob.near".to_string(),
            ..alice_token
        }
    }
}

pub struct DannyNftToken;

impl Dummy<DannyNftToken> for NftToken {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &DannyNftToken, _rng: &mut R) -> Self {
        let alice_token: NftToken = AliceNftToken.fake();
        Self {
            owner_id: "danny.near".to_string(),
            ..alice_token
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alice_nft_token_correct() {
        let token: NftToken = AliceNftToken.fake();
        assert_eq!(token.owner_id, "alice.near".to_string());
    }

    #[test]
    fn bob_nft_token_correct() {
        let token: NftToken = BobNftToken.fake();
        assert_eq!(token.owner_id, "bob.near".to_string());
    }

    #[test]
    fn danny_nft_token_correct() {
        let token: NftToken = DannyNftToken.fake();
        assert_eq!(token.owner_id, "danny.near".to_string());
    }
}
