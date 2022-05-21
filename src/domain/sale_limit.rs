#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SaleLimit(i64);

crate::domain::impl_limit_for_domain!(SaleLimit);

#[cfg(test)]
mod tests {
    use crate::domain::helpers::PositiveIntegersFixture;
    use crate::domain::ParseToPositiveInt;
    use crate::domain::private::New;

    use super::*;

    #[quickcheck_macros::quickcheck]
    fn valid_limit_are_parsed_successfully(valid_limit: PositiveIntegersFixture) -> bool {
        SaleLimit::parse(valid_limit.0).is_ok()
    }

    #[test]
    fn negative_limit_is_rejected() {
        let limit = Some(-10);
        let actual = SaleLimit::parse(limit);
        assert!(
            actual.is_err(),
            "`SaleLimit` isn't `Err`, actual value is {:?}",
            actual
        );
    }

    #[test]
    fn none_limit_equals_default() {
        let limit = None;
        let actual = SaleLimit::parse(limit);
        assert!(
            actual.is_ok(),
            "The actual `SaleLimit` isn't `Ok`, actual value is {:?}",
            actual
        );
        assert_eq!(
            actual,
            Ok(SaleLimit::default()),
            "The actual `SaleLimit` doesn't equal default value, actual value is {:?}",
            actual
        );
    }

    #[test]
    fn default_limit_is_100() {
        let actual = SaleLimit::default();
        assert_eq!(
            actual,
            SaleLimit::new(100),
            "The actual `SaleLimit` doesn't contain `100i64`, actual value is {:?}",
            actual
        );
    }
}
