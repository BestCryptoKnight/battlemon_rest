#[derive(Debug, Copy, Clone)]
pub struct SaleFilter {
    pub limit: crate::domain::Limit,
    pub offset: crate::domain::Offset,
}

impl SaleFilter {
    pub fn limit(&self) -> i64 {
        self.limit.get()
    }

    pub fn offset(&self) -> i64 {
        self.offset.get()
    }
}
