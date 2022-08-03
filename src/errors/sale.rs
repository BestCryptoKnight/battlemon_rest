use std::fmt::{Debug, Formatter, Result};

use crate::errors::JsonError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;

use crate::errors::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum SaleError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl Debug for SaleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        error_chain_fmt(self, f)
    }
}

impl actix_web::ResponseError for SaleError {
    fn status_code(&self) -> StatusCode {
        match self {
            SaleError::ValidationError(_) => StatusCode::BAD_REQUEST,
            SaleError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            SaleError::ValidationError(_) => HttpResponse::BadRequest().json(JsonError::new(self)),
            SaleError::UnexpectedError(_) => {
                HttpResponse::InternalServerError().json(JsonError::new(self))
            }
        }
    }
}
