pub use limit::*;
pub use nft_token::*;
pub use nft_token_days::*;
pub use token_id::*;
pub use nft_token_user_id::*;
pub use offset::*;
pub use paid::*;
pub use paid_days::*;
pub use sale::*;
pub use sale_days::*;

use self::private::New;

mod limit;
mod nft_token;
mod nft_token_days;
mod token_id;
mod nft_token_user_id;
mod offset;
mod paid;
mod paid_days;
mod sale;
mod sale_days;

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
    fn parse(value: Option<T>) -> Result<Self, String>;
}

pub trait IntoInner {
    fn into_inner(self) -> Option<String>;
}

pub trait AsInner {
    fn as_inner(&self) -> Option<&str>;
}

macro_rules! impl_into_inner {
    ($t:ident) => {
        impl crate::domain::IntoInner for $t {
            fn into_inner(self) -> Option<String> {
                self.0
            }
        }
    };
}

macro_rules! impl_as_inner {
    ($t:ident) => {
        impl crate::domain::AsInner for $t {
            fn as_inner(&self) -> Option<&str> {
                self.0.as_deref()
            }
        }
    };
}

#[rustfmt::skip]
pub(crate) use {impl_into_inner, impl_as_inner};

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
