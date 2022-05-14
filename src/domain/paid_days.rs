use crate::domain::{New, ParseToPositiveInt};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PaidDays(i64);

impl PaidDays {
    pub fn get(&self) -> i64 {
        self.0
    }
}

impl New for PaidDays {
    fn new(days: i64) -> Self {
        Self(days)
    }
}

impl ParseToPositiveInt for PaidDays {}

impl Default for PaidDays {
    fn default() -> Self {
        Self(1)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::helpers::PositiveIntegersFixture;

    use super::*;

    #[quickcheck_macros::quickcheck]
    fn valid_days_are_parsed_successfully(valid_days: PositiveIntegersFixture) -> bool {
        PaidDays::parse(valid_days.0).is_ok()
    }

    #[test]
    fn negative_days_is_rejected() {
        let days = Some(-5);
        let actual = PaidDays::parse(days);
        assert!(
            actual.is_err(),
            "`PaidDays` isn't `Err`, actual value is {:?}",
            actual
        );
    }

    #[test]
    fn none_days_equals_default() {
        let days = None;
        let actual = PaidDays::parse(days);
        assert!(
            actual.is_ok(),
            "The actual `PaidDays` isn't ok, actual value is {:?}",
            actual
        );
        assert_eq!(
            actual,
            Ok(PaidDays::default()),
            "The actual `PaidDays` doesn't equal `default`, actual value is {:?}",
            actual
        );
    }

    #[test]
    fn default_days_equals_1() {
        let actual = PaidDays::default();
        assert_eq!(
            actual,
            PaidDays(1),
            "The actual `PaidDays` value doesn't equal 1, actual value is {:?}",
            actual
        );
    }
}
