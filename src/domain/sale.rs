use super::sale_limit::SaleLimit;
use super::sale_offset::SaleOffset;
use crate::routes::PaginationQuery;

pub struct SaleFilter {
    pub limit: SaleLimit,
    pub offset: SaleOffset,
}

impl TryFrom<PaginationQuery> for SaleFilter {
    type Error = &'static str;

    fn try_from(value: PaginationQuery) -> Result<Self, Self::Error> {
        let limit = SaleLimit::parse(value.limit).unwrap();
        let offset = SaleOffset::parse(value.offset).unwrap();
        Ok(Self { limit, offset })
    }
}
