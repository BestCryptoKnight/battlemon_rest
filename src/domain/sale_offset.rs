#[derive(Debug, Copy, Clone)]
pub struct SaleOffset(i64);

impl SaleOffset {
    pub fn parse(offset: Option<i64>) -> Result<SaleOffset, String> {
        match offset {
            Some(n) if n.is_negative() => {
                Err(format!("The offset value is {n}. It must be positive."))
            }
            None => Ok(Self(0)),
            Some(n) => Ok(Self(n)),
        }
    }

    pub fn into_inner(self) -> i64 {
        self.0
    }
}
