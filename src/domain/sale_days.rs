use crate::domain::{New, ParseToPositiveInt};

#[derive(Debug, Copy, Clone, Default)]
pub struct SaleDays;

impl New for SaleDays {
    fn new(_: i64) -> Self {
        unreachable!("days prohibit for sales filter")
    }
}

impl ParseToPositiveInt for SaleDays {
    fn parse(days: Option<i64>) -> Result<Self, &'static str> {
        match days {
            None => Ok(Self),
            Some(_) => Err("query 'days' is prohibited for the `sales` route"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_value_is_valid_for_being_parsed_successfully_by_sale_days() {
        let days = None;
        let actual = SaleDays::parse(days);
        assert!(
            actual.is_ok(),
            "The actual `SaleDays` isn't `Ok`, actual value is {:?}",
            actual
        )
    }

    #[test]
    fn any_some_values_for_days_is_rejected() {
        let days = Some(10);
        let actual = SaleDays::parse(days);
        assert!(
            actual.is_err(),
            "The actual `SaleDays` isn't `Err(..)`, actual value is {:?}",
            actual
        )
    }
}
