use std::fmt::{Debug, Formatter, Result};

use crate::errors::JsonError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;

use crate::errors::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum ContractError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl Debug for ContractError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        error_chain_fmt(self, f)
    }
}

impl actix_web::ResponseError for ContractError {
    fn status_code(&self) -> StatusCode {
        match self {
            ContractError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            ContractError::UnexpectedError(_) => {
                HttpResponse::InternalServerError().json(JsonError::new(self))
            }
        }
    }
}
