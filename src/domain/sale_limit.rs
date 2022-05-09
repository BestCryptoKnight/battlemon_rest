pub struct SaleLimit(i64);

const DEFAULT_LIMIT: i64 = 100;

impl SaleLimit {
    pub fn parse(sale_limit: Option<i64>) -> Result<SaleLimit, String> {
        match sale_limit {
            Some(n) if n.is_negative() => Err(format!("{sale_limit:?} must be positive")),
            Some(0) | None => Ok(Self(DEFAULT_LIMIT)),
            Some(n) => Ok(Self(n)),
        }
    }
}
