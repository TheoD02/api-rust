// src/controllers/user_controller.rs
// Equivalent de: src/Controller/UserController.php

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;

use crate::config::AppState;
use crate::dto::{CreateUserDto, PaginationQuery, UpdateUserDto, UserResponse};
use crate::error::{ApiResult, ErrorResponse};
use crate::response::{ApiResponse, ApiResponseBuilder, PaginatedResponse};
use crate::validation::ValidatedJson;

/// UserController - User management endpoints
pub struct UserController;

impl UserController {
    /// Register routes for this controller
    pub fn routes() -> Router<Arc<AppState>> {
        Router::new()
            .route("/users", get(list_users))
            .route("/users", post(create_user))
            .route("/users/:id", get(get_user))
            .route("/users/:id", put(update_user))
            .route("/users/:id", delete(delete_user))
    }
}

/// GET /users - List all users with pagination
/// Response: { "data": [...], "meta": { "total": 100, "page": 1, ... } }
#[utoipa::path(
    get,
    path = "/users",
    tag = "users",
    params(PaginationQuery),
    responses(
        (status = 200, description = "Paginated list of users", body = inline(PaginatedResponse<UserResponse>)),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
async fn list_users(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationQuery>,
) -> ApiResult<PaginatedResponse<UserResponse>> {
    let result = state.user_service.find_all(&pagination).await?;
    Ok(ApiResponseBuilder::paginated(
        result.users,
        result.total,
        pagination.page,
        pagination.per_page,
    ))
}

/// GET /users/:id - Get user by ID
/// Response: { "data": { ... } }
#[utoipa::path(
    get,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = inline(ApiResponse<UserResponse>)),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<ApiResponse<UserResponse>> {
    let user = state.user_service.find_by_id(id).await?;
    Ok(ApiResponseBuilder::one(user))
}

/// POST /users - Create a new user
/// Response: { "data": { ... } }
#[utoipa::path(
    post,
    path = "/users",
    tag = "users",
    request_body = CreateUserDto,
    responses(
        (status = 201, description = "User created successfully", body = inline(ApiResponse<UserResponse>)),
        (status = 409, description = "Email already exists", body = ErrorResponse),
        (status = 422, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
async fn create_user(
    State(state): State<Arc<AppState>>,
    ValidatedJson(dto): ValidatedJson<CreateUserDto>,
) -> ApiResult<(StatusCode, ApiResponse<UserResponse>)> {
    let user = state.user_service.create(dto).await?;
    Ok(ApiResponseBuilder::created(user))
}

/// PUT /users/:id - Update a user
/// Response: { "data": { ... } }
#[utoipa::path(
    put,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    request_body = UpdateUserDto,
    responses(
        (status = 200, description = "User updated successfully", body = inline(ApiResponse<UserResponse>)),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 409, description = "Email already exists", body = ErrorResponse),
        (status = 422, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    ValidatedJson(dto): ValidatedJson<UpdateUserDto>,
) -> ApiResult<ApiResponse<UserResponse>> {
    let user = state.user_service.update(id, dto).await?;
    Ok(ApiResponseBuilder::one(user))
}

/// DELETE /users/:id - Delete a user
#[utoipa::path(
    delete,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "User ID to delete")
    ),
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<StatusCode> {
    state.user_service.delete(id).await?;
    Ok(ApiResponseBuilder::no_content())
}
