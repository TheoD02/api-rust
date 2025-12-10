// src/config/openapi.rs
// Equivalent de: config/packages/nelmio_api_doc.yaml

use utoipa::OpenApi;

use crate::controllers::health_controller::{__path_health, __path_index};
use crate::controllers::post_controller::{
    __path_create_post, __path_delete_post, __path_get_post, __path_list_posts, __path_update_post,
};
use crate::controllers::user_controller::{
    __path_create_user, __path_delete_user, __path_get_user, __path_list_users, __path_update_user,
};
use crate::dto::{
    AuthorResponse, CreatePostDto, CreatePostMetadataDto, CreatePostSettingsDto,
    CreateSeoMetadataDto, CreateTagDto, CreateUserDto, PaginationQuery, PostListItemResponse,
    PostMetadataResponse, PostResponse, PostSettingsResponse, SeoMetadataResponse, TagResponse,
    UpdatePostDto, UpdateUserDto, UserResponse,
};
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
        // Post endpoints
        list_posts,
        get_post,
        create_post,
        update_post,
        delete_post,
    ),
    components(
        schemas(
            // User DTOs
            CreateUserDto,
            UpdateUserDto,
            UserResponse,
            // Post DTOs
            CreatePostDto,
            UpdatePostDto,
            PostResponse,
            PostListItemResponse,
            // Nested objects - Input
            CreatePostMetadataDto,
            CreateTagDto,
            CreateSeoMetadataDto,
            CreatePostSettingsDto,
            // Nested objects - Output
            AuthorResponse,
            PostMetadataResponse,
            TagResponse,
            SeoMetadataResponse,
            PostSettingsResponse,
            // Pagination
            PaginationQuery,
            PaginationMeta,
            // Error
            ErrorResponse,
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "posts", description = "Post management with nested objects (tags, SEO, settings)")
    ),
    servers(
        (url = "http://localhost:8080", description = "Local development server"),
        (url = "https://api-rust.theo-corp.fr", description = "Production server"),
    )
)]
pub struct ApiDoc;
