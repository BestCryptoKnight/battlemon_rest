pub struct SaleLimit(i64);

const DEFAULT_LIMIT: i64 = 100;

impl SaleLimit {
    pub fn parse(sale_limit: Option<i64>) -> Result<SaleLimit, String> {
        match sale_limit {
            Some(i64::MIN..=-1) => Err(format!("value inside {sale_limit:?} must be positive")),
            Some(0) | None => Ok(Self(DEFAULT_LIMIT)),
            Some(n) => Ok(Self(n)),
        }
    }
}
