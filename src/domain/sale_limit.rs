#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SaleLimit(i64);

const DEFAULT_LIMIT: i64 = 100;

impl SaleLimit {
    pub fn parse(limit: Option<i64>) -> Result<SaleLimit, String> {
        match limit {
            Some(n) if n.is_negative() => {
                Err(format!("The limit value is {n:?}. It must be positive"))
            }
            None => Ok(Self(DEFAULT_LIMIT)),
            Some(n) => Ok(Self(n)),
        }
    }

    pub fn into_inner(self) -> i64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use claim::{assert_err, assert_ok_eq};

    use super::*;

    #[test]
    fn negative_limit_is_rejected() {
        let limit = Some(-10);
        assert_err!(SaleLimit::parse(limit));
    }

    #[test]
    fn none_limit_equals_default() {
        let limit = None;
        assert_ok_eq!(SaleLimit::parse(limit), SaleLimit(DEFAULT_LIMIT));
    }
}
