use std::error;
use std::fmt::{self, Formatter};

use actix_web::ResponseError;

fn error_chain_fmt(error: &impl error::Error, f: &mut Formatter<'_>) -> fmt::Result {
    writeln!(f, "{}\n", error)?;
    let mut current = error.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }

    Ok(())
}

pub struct QuerySalesError(pub sqlx::Error);

impl fmt::Debug for QuerySalesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl fmt::Display for QuerySalesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "A database error was encountered while trying to get sales."
        )
    }
}

impl ResponseError for QuerySalesError {}

impl error::Error for QuerySalesError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.0)
    }
}
