// src/error/mod.rs
// Equivalent de: src/Exception/ en Symfony

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;
use tracing::error;
use utoipa::ToSchema;

/// Error response format
/// Equivalent de: normalisation des erreurs en Symfony
#[derive(Debug, Serialize, ToSchema)]
#[schema(example = json!({ "error": "Resource not found" }))]
pub struct ErrorResponse {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

/// API Error types
/// Equivalent de: HttpException, NotFoundHttpException, etc. en Symfony
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Resource not found")]
    NotFound,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Validation error")]
    ValidationError(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error")]
    InternalError(String),

    #[error("Database error")]
    DatabaseError(#[from] sea_orm::DbErr),
}

impl ApiError {
    pub fn not_found() -> Self {
        Self::NotFound
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::InternalError(msg.into())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_response) = match &self {
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    error: "Resource not found".to_string(),
                    details: None,
                },
            ),
            ApiError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error: "Bad request".to_string(),
                    details: Some(msg.clone()),
                },
            ),
            ApiError::ValidationError(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorResponse {
                    error: "Validation error".to_string(),
                    details: Some(msg.clone()),
                },
            ),
            ApiError::Conflict(msg) => (
                StatusCode::CONFLICT,
                ErrorResponse {
                    error: "Conflict".to_string(),
                    details: Some(msg.clone()),
                },
            ),
            ApiError::InternalError(msg) => {
                error!(error = %msg, "Internal server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse {
                        error: "Internal server error".to_string(),
                        details: None, // Don't expose internal details
                    },
                )
            }
            ApiError::DatabaseError(err) => {
                error!(error = %err, "Database error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse {
                        error: "Database error".to_string(),
                        details: None,
                    },
                )
            }
        };

        (status, Json(error_response)).into_response()
    }
}

/// Result type alias for API handlers
pub type ApiResult<T> = Result<T, ApiError>;
