#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Offset(i64);

impl crate::domain::New for Offset {
    fn new(offset: i64) -> Self {
        Self(offset)
    }
}

impl crate::domain::ParseToPositiveInt for Offset {
    const ERROR: &'static str = "The offset value must be positive.";
}

impl Offset {
    pub fn get(self) -> i64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::helpers::PositiveIntegersFixture;
    use crate::domain::private::New;
    use crate::domain::ParseToPositiveInt;

    use super::*;

    #[quickcheck_macros::quickcheck]
    fn valid_offset_are_parsed_successfully(valid_offset: PositiveIntegersFixture) -> bool {
        Offset::parse(valid_offset.0).is_ok()
    }

    #[test]
    fn negative_offset_is_rejected() {
        let offset = Some(-10);
        let actual = Offset::parse(offset);
        assert!(
            actual.is_err(),
            "`Offset` isn't `Err`, actual value is {:?}",
            actual
        );
    }

    #[test]
    fn none_offset_equals_default() {
        let offset = None;
        let actual = Offset::parse(offset);
        assert!(
            actual.is_ok(),
            "The actual `Offset` isn't `Ok`, actual value is {:?}",
            actual
        );
        assert_eq!(
            actual,
            Ok(Offset::default()),
            "The actual `Offset` doesn't equal default value, actual value is {:?}",
            actual
        );
    }

    #[test]
    fn default_offset_is_100() {
        let actual = Offset::default();
        assert_eq!(
            actual,
            Offset::new(0),
            "The actual `Offset` doesn't contain `0i64`, actual value is {:?}",
            actual
        );
    }
}
