use crate::domain::{PaidDays, PaidLimit, PaidOffset};

#[derive(Debug)]
pub struct PaidFilter {
    pub limit: PaidLimit,
    pub offset: PaidOffset,
    pub days: PaidDays,
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
