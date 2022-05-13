use super::sale_limit::SaleLimit;
use super::sale_offset::SaleOffset;

#[derive(Debug)]
pub struct SaleFilter {
    pub limit: SaleLimit,
    pub offset: SaleOffset,
}

impl SaleFilter {
    pub fn limit(&self) -> i64 {
        self.limit.get()
    }

    pub fn offset(&self) -> i64 {
        self.offset.get()
    }
}
