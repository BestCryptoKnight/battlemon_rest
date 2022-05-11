#[derive(Debug, Copy, Clone)]
pub struct SaleDays;

impl SaleDays {
    #[allow(unused_variables)]
    pub fn parse(days: Option<i64>) -> Result<(), String> {
        Err("query 'days' is prohibited for the `sales` route".into())
    }
}
