#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct NftTokenOffset(i64);

crate::domain::impl_offset_for_domain!(NftTokenOffset);

#[cfg(test)]
mod tests {
    use crate::domain::helpers::PositiveIntegersFixture;
    use crate::domain::private::New;
    use crate::domain::ParseToPositiveInt;

    use super::*;

    #[quickcheck_macros::quickcheck]
    fn valid_offset_are_parsed_successfully(valid_offset: PositiveIntegersFixture) -> bool {
        NftTokenOffset::parse(valid_offset.0).is_ok()
    }

    #[test]
    fn negative_offset_is_rejected() {
        let offset = Some(-10);
        let actual = NftTokenOffset::parse(offset);
        assert!(
            actual.is_err(),
            "`NftTokenOffset` isn't `Err`, actual value is {:?}",
            actual
        );
    }

    #[test]
    fn none_offset_equals_default() {
        let offset = None;
        let actual = NftTokenOffset::parse(offset);
        assert!(
            actual.is_ok(),
            "The actual `NftTokenOffset` isn't `Ok`, actual value is {:?}",
            actual
        );
        assert_eq!(
            actual,
            Ok(NftTokenOffset::default()),
            "The actual `NftTokenOffset` doesn't equal default value, actual value is {:?}",
            actual
        );
    }

    #[test]
    fn default_offset_is_100() {
        let actual = NftTokenOffset::default();
        assert_eq!(
            actual,
            NftTokenOffset::new(0),
            "The actual `NftTokenOffset` doesn't contain `0i64`, actual value is {:?}",
            actual
        );
    }
}
