use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::fmt;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    Unauthorized,
    Forbidden,
    Internal(String),
    Conflict(String),
    ValidationError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            ApiError::Unauthorized => write!(f, "Unauthorized"),
            ApiError::Forbidden => write!(f, "Forbidden"),
            ApiError::Internal(msg) => write!(f, "Internal error: {}", msg),
            ApiError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            ApiError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            ApiError::ValidationError(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
        };

        let body = Json(ErrorResponse {
            error: status.as_str().to_string(),
            message: error_message.clone(),
            details: Some(self.to_string()),
        });

        (status, body).into_response()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::Internal(err.to_string())
    }
}

impl From<uuid::Error> for ApiError {
    fn from(err: uuid::Error) -> Self {
        ApiError::BadRequest(format!("Invalid UUID: {}", err))
    }
}