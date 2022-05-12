#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SaleLimit(i64);

impl Default for SaleLimit {
    fn default() -> Self {
        Self(100)
    }
}

impl SaleLimit {
    pub fn parse(limit: Option<i64>) -> Result<SaleLimit, String> {
        match limit {
            Some(n) if n.is_negative() => {
                Err(format!("The limit value is {n:?}. It must be positive"))
            }
            None => Ok(Self::default()),
            Some(n) => Ok(Self(n)),
        }
    }

    pub fn into_inner(self) -> i64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
