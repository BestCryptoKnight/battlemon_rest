pub struct SaleOffset(i64);

impl SaleOffset {
    pub fn parse(sale_offset: Option<i64>) -> Result<SaleOffset, String> {
        match sale_offset {
            Some(n) if n.is_negative() => Err(format!("{sale_offset:?} must be positive")),
            None => Ok(Self(0)),
            Some(n) => Ok(Self(n)),
        }
    }
}
