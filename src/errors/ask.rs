use std::fmt::{Debug, Formatter, Result};

use crate::errors::JsonError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;

use crate::errors::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum AskError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl Debug for AskError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        error_chain_fmt(self, f)
    }
}

impl actix_web::ResponseError for AskError {
    fn status_code(&self) -> StatusCode {
        match self {
            AskError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AskError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            AskError::ValidationError(_) => HttpResponse::BadRequest().json(JsonError::new(self)),
            AskError::UnexpectedError(_) => {
                HttpResponse::InternalServerError().json(JsonError::new(self))
            }
        }
    }
}
