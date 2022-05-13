use crate::domain::{New, ParseToPositiveInt};

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct SaleOffset(i64);

impl New for SaleOffset {
    fn new(offset: i64) -> Self {
        Self(offset)
    }
}

impl ParseToPositiveInt for SaleOffset {
    const ERROR: &'static str = "The offset value must be positive.";
}

impl SaleOffset {
    pub fn get(self) -> i64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::helpers::PositiveIntegersFixture;

    use super::*;

    #[quickcheck_macros::quickcheck]
    fn valid_offset_are_parsed_successfully(valid_offset: PositiveIntegersFixture) -> bool {
        SaleOffset::parse(valid_offset.0).is_ok()
    }

    #[test]
    fn negative_offset_is_rejected() {
        let offset = Some(-10);
        let actual = SaleOffset::parse(offset);
        assert!(
            actual.is_err(),
            "`SaleOffset` isn't `Err`, actual value is {:?}",
            actual
        );
    }

    #[test]
    fn none_offset_equals_default() {
        let offset = None;
        let actual = SaleOffset::parse(offset);
        assert!(
            actual.is_ok(),
            "The actual `SaleOffset` isn't `Ok`, actual value is {:?}",
            actual
        );
        assert_eq!(
            actual,
            Ok(SaleOffset::default()),
            "The actual `SaleOffset` doesn't equal default value, actual value is {:?}",
            actual
        );
    }

    #[test]
    fn default_offset_is_100() {
        let actual = SaleOffset::default();
        assert_eq!(
            actual,
            SaleOffset::new(0),
            "The actual `SaleOffset` doesn't contain `0i64`, actual value is {:?}",
            actual
        );
    }
}
