pub use paid::*;
pub use paid_limit::*;
pub use paid_offset::*;
pub use sale::*;
pub use sale_days::*;
pub use sale_limit::*;
pub use sale_offset::*;

mod paid;
mod paid_limit;
mod paid_offset;
mod sale;
mod sale_days;
mod sale_limit;
mod sale_offset;

pub trait New {
    fn new(value: i64) -> Self;
}

pub trait ParseToPositiveInt: New + Default
where
    Self: Sized,
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
