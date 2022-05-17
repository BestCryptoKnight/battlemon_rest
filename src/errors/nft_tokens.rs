use actix_web::http::StatusCode;

#[derive(thiserror::Error)]
pub enum NftTokensError {
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
            NftTokensError::ValidationError(_) => StatusCode::BAD_REQUEST,
            NftTokensError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        crate::errors::default_error_response(self)
    }
}
