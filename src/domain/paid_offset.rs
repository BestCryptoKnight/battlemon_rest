use crate::domain::{New, ParseToPositiveInt};

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct PaidOffset(i64);

impl New for PaidOffset {
    fn new(offset: i64) -> Self {
        Self(offset)
    }
}

impl ParseToPositiveInt for PaidOffset {
    const ERROR: &'static str = "The offset value must be positive";
}

#[cfg(test)]
mod tests {
    use crate::domain::helpers::PositiveIntegersFixture;

    use super::*;

    #[quickcheck_macros::quickcheck]
    fn valid_offset_are_parsed_successfully(valid_offset: PositiveIntegersFixture) -> bool {
        PaidOffset::parse(valid_offset.0).is_ok()
    }

    #[test]
    fn negative_offset_is_rejected() {
        let offset = Some(-10);
        let actual = PaidOffset::parse(offset);
        assert!(
            actual.is_err(),
            "The actual `PaidOffset` isn't `Err`, actual value is {:?}",
            actual
        );
    }

    #[test]
    fn none_offset_equals_default() {
        let offset = None;
        let actual = PaidOffset::parse(offset);
        assert!(
            actual.is_ok(),
            "The actual `PaidOffset` isn't `Ok`, actual value is {:?}",
            actual
        );
        assert_eq!(
            actual,
            Ok(PaidOffset::default()),
            "The actual `PaidOffset` doesn't equal `Ok(PaidOffset(0))`, actual value is {:?}",
            actual
        );
    }

    #[test]
    fn default_offset_is_0() {
        let actual = PaidOffset::default();
        assert_eq!(
            actual,
            PaidOffset::new(0),
            "The actual `PaidOffset` doesn't contain `0i64`, actual value is {:?}",
            actual
        );
    }
}
