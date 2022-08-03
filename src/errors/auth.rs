use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use crate::errors::JsonError;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Bad request")]
    BadRequest(#[source] anyhow::Error),
    #[error("Invalid credentials")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl actix_web::ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AuthError::InvalidCredentials(_) => StatusCode::UNAUTHORIZED,
            AuthError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::BadRequest(_) => HttpResponse::BadRequest().json(JsonError::new(self)),
            AuthError::InvalidCredentials(_) => {
                HttpResponse::Unauthorized().json(JsonError::new(self))
            }
            AuthError::UnexpectedError(_) => {
                HttpResponse::InternalServerError().json(JsonError::new(self))
            }
        }
    }
}
