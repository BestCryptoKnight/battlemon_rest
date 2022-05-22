#[derive(Debug)]
pub struct PaidFilter {
    pub limit: crate::domain::Limit,
    pub offset: crate::domain::Offset,
    pub days: crate::domain::PaidDays,
}

impl PaidFilter {
    pub fn limit(&self) -> i64 {
        self.limit.get()
    }

    pub fn offset(&self) -> i64 {
        self.offset.get()
    }

    pub fn days(&self) -> i64 {
        self.days.get()
    }
}
