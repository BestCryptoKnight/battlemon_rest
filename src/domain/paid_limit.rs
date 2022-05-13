use crate::domain::{New, ParseToPositiveInt};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PaidLimit(i64);

impl PaidLimit {
    pub fn get(&self) -> i64 {
        self.0
    }
}

impl New for PaidLimit {
    fn new(limit: i64) -> Self {
        Self(limit)
    }
}

impl Default for PaidLimit {
    fn default() -> Self {
        Self(100)
    }
}

impl ParseToPositiveInt for PaidLimit {
    const ERROR: &'static str = "The limit value must be positive";
}

#[cfg(test)]
mod tests {
    use crate::domain::helpers::PositiveIntegersFixture;

    use super::*;

    #[quickcheck_macros::quickcheck]
    fn negative_limit_fails(valid_limit: PositiveIntegersFixture) -> bool {
        PaidLimit::parse(valid_limit.0).is_ok()
    }

    #[test]
    fn negative_limit_is_rejected() {
        let limit = Some(-10);
        let actual = PaidLimit::parse(limit);
        assert!(
            actual.is_err(),
            "The actual `PaidLimit` isn't `Err`, actual value is {:?}",
            actual
        );
    }

    #[test]
    fn none_limit_equals_default() {
        let limit = None;
        let actual = PaidLimit::parse(limit);
        assert!(
            actual.is_ok(),
            "The actual `PaidLimit` isn't `Ok`, actual value is {:?}",
            actual
        );
        assert_eq!(
            actual,
            Ok(PaidLimit::default()),
            "The actual `PaidLimit` doesn't equal default value, actual value is {:?}",
            actual
        )
    }

    #[test]
    fn default_limit_is_100() {
        let actual = PaidLimit::default();
        assert_eq!(
            actual,
            PaidLimit::new(100),
            "The actual `PaidLimit` doesn't contain `100i64`, actual value is {:?}",
            actual
        );
    }
}
