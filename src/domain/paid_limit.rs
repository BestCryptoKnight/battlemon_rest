#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PaidLimit(i64);

crate::domain::impl_limit_for_domain!(PaidLimit);

#[cfg(test)]
mod tests {
    use crate::domain::helpers::PositiveIntegersFixture;
    use crate::domain::private::New;
    use crate::domain::ParseToPositiveInt;

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
