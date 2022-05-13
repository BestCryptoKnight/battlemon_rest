pub use paid::*;
pub use paid_limit::*;
pub use sale::*;
pub use sale_days::*;
pub use sale_limit::*;
pub use sale_offset::*;

mod paid;
mod paid_limit;
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
