// src/config/openapi.rs
// Equivalent de: config/packages/nelmio_api_doc.yaml

use utoipa::OpenApi;

use crate::controllers::health_controller::{__path_index, __path_health};
use crate::controllers::user_controller::{
    __path_list_users, __path_get_user, __path_create_user, __path_update_user, __path_delete_user
};
use crate::dto::{CreateUserDto, PaginationQuery, UpdateUserDto, UserResponse};
use crate::error::ErrorResponse;
use crate::response::PaginationMeta;

/// OpenAPI Documentation
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Rust API",
        version = "1.0.0",
        description = "A REST API built with Rust, Axum and SeaORM\n\nStructure inspired by Symfony:\n- Controllers: HTTP handlers\n- Services: Business logic\n- DTOs: Data transfer objects\n- Entities: Database models\n\n## Response Format\n\nAll responses follow this structure:\n```json\n{\n  \"data\": { ... } or [...],\n  \"meta\": { ... } // optional, for pagination\n}\n```",
        contact(
            name = "API Support",
            email = "support@example.com"
        ),
        license(
            name = "MIT"
        )
    ),
    paths(
        // Health endpoints
        index,
        health,
        // User endpoints
        list_users,
        get_user,
        create_user,
        update_user,
        delete_user,
    ),
    components(
        schemas(
            // DTOs
            CreateUserDto,
            UpdateUserDto,
            UserResponse,
            PaginationQuery,
            // Response
            PaginationMeta,
            // Error
            ErrorResponse,
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "users", description = "User management endpoints")
    ),
    servers(
        (url = "http://localhost:8080", description = "Local development server")
    )
)]
pub struct ApiDoc;
