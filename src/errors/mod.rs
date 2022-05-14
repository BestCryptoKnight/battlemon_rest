use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};

use serde::{Deserialize, Serialize};
use actix_web::{HttpResponse, ResponseError};
use actix_web::http::header;

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

pub fn default_error_response<T>(error: &T) -> HttpResponse
where
    T: Debug + Display + ResponseError,
{
    let json_error = JsonError::new(error.to_string());
    HttpResponse::build(error.status_code())
        .insert_header((header::CONTENT_TYPE, "application/json"))
        .json(json_error)
}
