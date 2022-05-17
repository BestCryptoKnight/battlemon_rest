pub use nft_token::*;
pub use nft_token_token_id::*;
pub use nft_token_user_id::*;
pub use paid::*;
pub use paid_days::*;
pub use paid_limit::*;
pub use paid_offset::*;
pub use sale::*;
pub use sale_days::*;
pub use sale_limit::*;
pub use sale_offset::*;

use self::private::New;

mod nft_token;
mod nft_token_token_id;
mod nft_token_user_id;
mod paid;
mod paid_days;
mod paid_limit;
mod paid_offset;
mod sale;
mod sale_days;
mod sale_limit;
mod sale_offset;

pub(self) mod private {
    pub enum Local {}

    pub trait New {
        fn new(value: i64) -> Self;
    }

    impl New for Local {
        fn new(_: i64) -> Self {
            unreachable!()
        }
    }
}

pub trait ParseToPositiveInt
where
    Self: Sized + New + Default,
{
    const ERROR: &'static str = "The parsed value must be positive.";

    fn parse(value: Option<i64>) -> Result<Self, &'static str> {
        match value {
            Some(v) if v.is_negative() => Err(Self::ERROR),
            None => Ok(Self::default()),
            Some(v) => Ok(Self::new(v)),
        }
    }
}

pub trait Parse<T>: Sized {
    fn parse(value: Option<T>) -> Result<Option<Self>, String>;
}

#[cfg(test)]
pub mod helpers {
    use fake::Fake;
    use rand::prelude::StdRng;
    use rand::SeedableRng;

    #[derive(Debug, Clone)]
    pub struct PositiveIntegersFixture(pub Option<i64>);

    impl quickcheck::Arbitrary for PositiveIntegersFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            let limit = (0..10000).fake_with_rng(&mut rng);
            Self(Some(limit))
        }
    }
}
