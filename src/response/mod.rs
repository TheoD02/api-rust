// src/response/mod.rs
// Standardized API response wrapper

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

/// Pagination metadata
#[derive(Debug, Serialize, ToSchema)]
#[schema(example = json!({
    "total": 100,
    "page": 1,
    "per_page": 10,
    "total_pages": 10
}))]
pub struct PaginationMeta {
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
}

impl PaginationMeta {
    pub fn new(total: u64, page: u64, per_page: u64) -> Self {
        let total_pages = (total + per_page - 1) / per_page;
        Self {
            total,
            page,
            per_page,
            total_pages,
        }
    }
}

/// Standard API response wrapper
/// Format: { "data": T, "meta": Option<M> }
#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T: Serialize, M: Serialize = ()> {
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<M>,
}

impl<T: Serialize> ApiResponse<T, ()> {
    /// Create response with data only (no meta)
    /// Example: { "data": { "id": 1, "username": "john" } }
    pub fn data(data: T) -> Self {
        Self { data, meta: None }
    }
}

impl<T: Serialize, M: Serialize> ApiResponse<T, M> {
    /// Create response with data and meta
    /// Example: { "data": [...], "meta": { "total": 100, "page": 1 } }
    pub fn with_meta(data: T, meta: M) -> Self {
        Self {
            data,
            meta: Some(meta),
        }
    }
}

impl<T: Serialize, M: Serialize> IntoResponse for ApiResponse<T, M> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

/// Helper type for list responses with pagination
pub type PaginatedResponse<T> = ApiResponse<Vec<T>, PaginationMeta>;

/// Response builder for common patterns
pub struct ApiResponseBuilder;

impl ApiResponseBuilder {
    /// Single item response
    /// { "data": { ... } }
    pub fn one<T: Serialize>(data: T) -> ApiResponse<T> {
        ApiResponse::data(data)
    }

    /// List response without pagination
    /// { "data": [...] }
    pub fn list<T: Serialize>(data: Vec<T>) -> ApiResponse<Vec<T>> {
        ApiResponse::data(data)
    }

    /// List response with pagination
    /// { "data": [...], "meta": { "total": 100, ... } }
    pub fn paginated<T: Serialize>(
        data: Vec<T>,
        total: u64,
        page: u64,
        per_page: u64,
    ) -> PaginatedResponse<T> {
        ApiResponse::with_meta(data, PaginationMeta::new(total, page, per_page))
    }

    /// Created response (201)
    pub fn created<T: Serialize>(data: T) -> (StatusCode, ApiResponse<T>) {
        (StatusCode::CREATED, ApiResponse::data(data))
    }

    /// No content response (204)
    pub fn no_content() -> StatusCode {
        StatusCode::NO_CONTENT
    }
}
