use std::fmt::{self, Debug, Formatter};

use actix_web::{http::StatusCode, HttpResponse, ResponseError};

use crate::errors::{self, error_chain_fmt};

#[derive(thiserror::Error)]
pub enum PaidError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl Debug for PaidError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for PaidError {
    fn status_code(&self) -> StatusCode {
        match self {
            PaidError::ValidationError(_) => StatusCode::BAD_REQUEST,
            PaidError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        errors::default_error_response(self)
    }
}
