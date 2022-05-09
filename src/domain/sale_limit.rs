pub struct SaleLimit(i64);

const DEFAULT_LIMIT: i64 = 100;

impl SaleLimit {
    pub fn parse(limit: Option<i64>) -> Result<SaleLimit, String> {
        match limit {
            Some(n) if n.is_negative() => Err(format!("{limit:?} must be positive")),
            Some(0) | None => Ok(Self(DEFAULT_LIMIT)),
            Some(n) => Ok(Self(n)),
        }
    }
}
