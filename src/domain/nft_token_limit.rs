use crate::domain::{New, ParseToPositiveInt};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NftTokenLimit(i64);

impl Default for NftTokenLimit {
    fn default() -> Self {
        Self(100)
    }
}

impl NftTokenLimit {
    pub fn get(self) -> i64 {
        self.0
    }
}

impl New for NftTokenLimit {
    fn new(value: i64) -> Self {
        Self(value)
    }
}

impl ParseToPositiveInt for NftTokenLimit {
    const ERROR: &'static str = "The limit value must be positive.";
}

#[cfg(test)]
mod tests {
    use crate::domain::helpers::PositiveIntegersFixture;

    use super::*;

    #[quickcheck_macros::quickcheck]
    fn valid_limit_are_parsed_successfully(valid_limit: PositiveIntegersFixture) -> bool {
        NftTokenLimit::parse(valid_limit.0).is_ok()
    }

    #[test]
    fn negative_limit_is_rejected() {
        let limit = Some(-10);
        let actual = NftTokenLimit::parse(limit);
        assert!(
            actual.is_err(),
            "`NftTokenLimit` isn't `Err`, actual value is {:?}",
            actual
        );
    }

    #[test]
    fn none_limit_equals_default() {
        let limit = None;
        let actual = NftTokenLimit::parse(limit);
        assert!(
            actual.is_ok(),
            "The actual `NftTokenLimit` isn't `Ok`, actual value is {:?}",
            actual
        );
        assert_eq!(
            actual,
            Ok(NftTokenLimit::default()),
            "The actual `NftTokenLimit` doesn't equal default value, actual value is {:?}",
            actual
        );
    }

    #[test]
    fn default_limit_is_100() {
        let actual = NftTokenLimit::default();
        assert_eq!(
            actual,
            NftTokenLimit::new(100),
            "The actual `NftTokenLimit` doesn't contain `100i64`, actual value is {:?}",
            actual
        );
    }
}
