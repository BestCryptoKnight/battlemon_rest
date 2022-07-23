use fake::faker::address::en::Geohash;
use fake::faker::chrono::raw::DateTime;
use fake::faker::lorem::en::Word;
use fake::faker::number::raw::NumberWithFormat;
use fake::locales::EN;
use fake::uuid::UUIDv4;
use fake::{Dummy, Fake, Faker};
use nft_models::{Lemon, ModelKind};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct Sale {
    pub id: uuid::Uuid,
    pub prev_owner: String,
    pub curr_owner: String,
    pub token_id: String,
    pub price: rust_decimal::Decimal,
    pub date: chrono::DateTime<chrono::Utc>,
}

impl Dummy<Faker> for Sale {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let scale = rng.gen_range(0..=24);
        let lo = rng.gen();
        let mid = rng.gen();
        let price = rust_decimal::Decimal::from_parts(lo, mid, 0, false, scale);
        Self {
            id: UUIDv4.fake::<uuid::Uuid>(),
            prev_owner: format!("{}.near", Word().fake::<String>()),
            curr_owner: format!("{}.near", Word().fake::<String>()),
            token_id: NumberWithFormat(EN, "^########").fake::<String>(),
            price,
            date: DateTime(EN).fake(),
        }
    }
}

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
        let traits_config = fake::vec![u8; 4]
            .try_into()
            .expect("Couldn't convert to the array with length 4");
        let model = ModelKind::Lemon(Lemon::from_random(&traits_config));
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
