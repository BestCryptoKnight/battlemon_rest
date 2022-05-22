#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Limit(i64);

impl Default for Limit {
    fn default() -> Self {
        Self(100)
    }
}

impl Limit {
    pub fn get(self) -> i64 {
        self.0
    }
}

impl crate::domain::New for Limit {
    fn new(value: i64) -> Self {
        Self(value)
    }
}

impl crate::domain::ParseToPositiveInt for Limit {
    const ERROR: &'static str = "The limit value must be positive.";
}

#[cfg(test)]
mod tests {
    use crate::domain::helpers::PositiveIntegersFixture;
    use crate::domain::private::New;
    use crate::domain::ParseToPositiveInt;

    use super::*;

    #[quickcheck_macros::quickcheck]
    fn valid_limit_are_parsed_successfully(valid_limit: PositiveIntegersFixture) -> bool {
        Limit::parse(valid_limit.0).is_ok()
    }

    #[test]
    fn negative_limit_is_rejected() {
        let limit = Some(-10);
        let actual = Limit::parse(limit);
        assert!(
            actual.is_err(),
            "`Limit` isn't `Err`, actual value is {:?}",
            actual
        );
    }

    #[test]
    fn none_limit_equals_default() {
        let limit = None;
        let actual = Limit::parse(limit);
        assert!(
            actual.is_ok(),
            "The actual `Limit` isn't `Ok`, actual value is {:?}",
            actual
        );
        assert_eq!(
            actual,
            Ok(Limit::default()),
            "The actual `Limit` doesn't equal default value, actual value is {:?}",
            actual
        );
    }

    #[test]
    fn default_limit_is_100() {
        let actual = Limit::default();
        assert_eq!(
            actual,
            Limit::new(100),
            "The actual `Limit` doesn't contain `100i64`, actual value is {:?}",
            actual
        );
    }
}
