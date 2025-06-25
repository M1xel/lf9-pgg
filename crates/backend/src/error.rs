use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use sea_orm::TransactionError;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Error, Debug)]
pub enum ApiError {
    // Database errors
    #[error("Database Error: {0}")]
    Database(#[from] sea_orm::DbErr),

    // Generic HTTP errors
    #[error("Bad Request: {0}")]
    BadRequest(String), // 400 Bad Request
    #[error("Unauthorized")]
    Unauthorized, // 401 Unauthorized
    #[error("Not Found")]
    NotFound, // 404 Not Found
    #[error("Internal Server Error for endpoint: {0}")]
    InternalServerError(String), // 500 Internal Server Error

    // Session errors
    #[error("Already logged in")]
    AlreadyLoggedIn,
    #[error("Session insert error: {0}")]
    SessionInsertError(#[from] actix_session::SessionInsertError),

    // Validation errors
    #[error("Validation Error: {0}")]
    ValidationError(#[from] validator::ValidationErrors),

    // Argon2 errors
    #[error("Argon2 Error: {0}")]
    Argon2Error(String),

    // User errors
    #[error("User with username - {0} - already exists")]
    UserAlreadyExists(String),
}
impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            // Database errors
            ApiError::Database(..) => StatusCode::INTERNAL_SERVER_ERROR, // 500 Internal Server Error

            // Generic HTTP errors
            ApiError::BadRequest(..) => StatusCode::BAD_REQUEST, // 400 Bad Request
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,  // 401 Unauthorized
            ApiError::NotFound => StatusCode::NOT_FOUND,         // 404 Not Found
            ApiError::InternalServerError(..) => StatusCode::INTERNAL_SERVER_ERROR, // 500 Internal Server Error

            // Session errors
            ApiError::AlreadyLoggedIn => StatusCode::CONFLICT, // 409 Conflict
            ApiError::SessionInsertError(..) => StatusCode::INTERNAL_SERVER_ERROR, // 500 Internal Server Error

            // Validation errors
            ApiError::ValidationError(..) => StatusCode::BAD_REQUEST, // 400 Bad Request

            // Argon2 errors
            ApiError::Argon2Error(..) => StatusCode::INTERNAL_SERVER_ERROR, // 500 Internal Server Error

            // User errors
            ApiError::UserAlreadyExists(..) => StatusCode::CONFLICT, // 409 Conflict
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
    /// Response message
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
