use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use sea_orm::TransactionError;
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database Error: {0}")]
    Database(#[from] sea_orm::DbErr),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not Found")]
    NotFound,
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Validation Error: {0}")]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("Argon2 Error: {0}")]
    Argon2Error(String),
    #[error("Session insert error: {0}")]
    SessionInsertError(#[from] actix_session::SessionInsertError),
    #[error("Already logged in")]
    AlreadyLoggedIn,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::Database(..) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::BadRequest(..) => StatusCode::BAD_REQUEST,
            ApiError::ValidationError(..) => StatusCode::BAD_REQUEST,
            ApiError::Argon2Error(..) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::SessionInsertError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::AlreadyLoggedIn => StatusCode::CONFLICT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

impl From<TransactionError<sea_orm::DbErr>> for ApiError {
    fn from(value: TransactionError<sea_orm::DbErr>) -> Self {
        Self::Database(match value {
            TransactionError::Connection(e) => e,
            TransactionError::Transaction(e) => e,
        })
    }
}

#[derive(Serialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

impl MessageResponse {
    /// Create a new message response
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
