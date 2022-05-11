#[derive(Debug, Copy, Clone)]
pub struct SaleLimit(i64);

const DEFAULT_LIMIT: i64 = 100;

impl SaleLimit {
    pub fn parse(limit: Option<i64>) -> Result<SaleLimit, String> {
        match limit {
            Some(n) if n.is_negative() => Err(format!("{n:?} must be positive")),
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
    use super::*;

    #[test]
    fn sales_limit_parsing_works() {
        todo!()
    }
}
