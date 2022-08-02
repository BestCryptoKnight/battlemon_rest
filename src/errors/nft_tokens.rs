use crate::errors::JsonError;
use actix_web::http::header::HeaderValue;
use actix_web::http::{header, StatusCode};
use actix_web::HttpResponse;

#[derive(thiserror::Error)]
pub enum NftTokensError {
    #[error(transparent)]
    AuthError(#[from] crate::auth::password::AuthError),
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for NftTokensError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::errors::error_chain_fmt(self, f)
    }
}

impl actix_web::ResponseError for NftTokensError {
    fn status_code(&self) -> StatusCode {
        match self {
            NftTokensError::AuthError(_) => StatusCode::UNAUTHORIZED,
            NftTokensError::ValidationError(_) => StatusCode::BAD_REQUEST,
            NftTokensError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            NftTokensError::AuthError(_) => HttpResponse::Unauthorized()
                .append_header((
                    header::WWW_AUTHENTICATE,
                    HeaderValue::from_static(r#"Basic realm="nft_token""#),
                ))
                .json(JsonError::new(self)),
            NftTokensError::ValidationError(_) => {
                HttpResponse::BadRequest().json(JsonError::new(self))
            }
            NftTokensError::UnexpectedError(_) => {
                HttpResponse::InternalServerError().json(JsonError::new(self))
            }
        }
    }
}
