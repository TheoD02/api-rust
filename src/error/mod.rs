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

// ============================================================
// Service Errors - Business logic errors (no HTTP concepts)
// ============================================================

/// Service layer errors - pure business logic errors
/// Equivalent de: Domain exceptions en Symfony (sans HTTP)
#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Entity not found")]
    NotFound,

    #[error("Entity already exists: {0}")]
    AlreadyExists(String),

    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
}

// ============================================================
// API Errors - HTTP layer errors
// ============================================================

/// Error response format
/// Equivalent de: normalisation des erreurs en Symfony
#[derive(Debug, Serialize, ToSchema)]
#[schema(example = json!({ "error": "Resource not found" }))]
pub struct ErrorResponse {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

/// API Error types - HTTP layer errors
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

/// Convert ServiceError to ApiError
/// This is where business errors become HTTP errors
impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::NotFound => ApiError::NotFound,
            ServiceError::AlreadyExists(msg) => ApiError::Conflict(msg),
            ServiceError::Database(db_err) => ApiError::DatabaseError(db_err),
        }
    }
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
