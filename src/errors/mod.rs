use std::error::Error;
use std::fmt::{self, Formatter};

use serde::{Deserialize, Serialize};

pub use paid::*;
pub use sale::*;

mod paid;
mod sale;

fn error_chain_fmt(error: &impl Error, f: &mut Formatter<'_>) -> fmt::Result {
    writeln!(f, "{}\n", error)?;
    let mut current = error.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonError {
    error: String,
}

impl JsonError {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}
