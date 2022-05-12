#[derive(Debug, Copy, Clone)]
pub struct SaleDays;

impl SaleDays {
    #[allow(unused_variables)]
    pub fn parse(days: Option<i64>) -> Result<(), String> {
        match days {
            None => Ok(()),
            Some(_) => Err("query 'days' is prohibited for the `sales` route".into()),
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
