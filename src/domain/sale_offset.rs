pub struct SaleOffset(i64);

impl SaleOffset {
    pub fn parse(offset: Option<i64>) -> Result<SaleOffset, String> {
        match offset {
            Some(n) if n.is_negative() => Err(format!("{sale_offset:?} must be positive")),
            None => Ok(Self(0)),
            Some(n) => Ok(Self(n)),
        }
    }
}
