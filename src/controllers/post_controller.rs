// src/controllers/post_controller.rs
// Controller pour les posts avec nested objects

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

use crate::config::AppState;
use crate::dto::{
    CreatePostDto, PaginationQuery, PostListItemResponse, PostResponse, UpdatePostDto,
};
use crate::error::{ApiResult, ErrorResponse};
use crate::response::{ApiResponse, ApiResponseBuilder, PaginatedResponse};
use crate::validation::ValidatedJson;

pub struct PostController;

impl PostController {
    pub fn routes() -> Router<Arc<AppState>> {
        Router::new()
            .route("/posts", get(list_posts))
            .route("/posts", post(create_post))
            .route("/posts/:id", get(get_post))
            .route("/posts/:id", put(update_post))
            .route("/posts/:id", delete(delete_post))
    }
}

/// GET /posts - Liste paginée des posts
#[utoipa::path(
    get,
    path = "/posts",
    tag = "posts",
    params(PaginationQuery),
    responses(
        (status = 200, description = "Liste paginée des posts", body = inline(PaginatedResponse<PostListItemResponse>)),
        (status = 500, description = "Erreur serveur", body = ErrorResponse)
    )
)]
async fn list_posts(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationQuery>,
) -> ApiResult<PaginatedResponse<PostListItemResponse>> {
    let result = state.post_service.find_all(&pagination).await?;

    let posts: Vec<PostListItemResponse> = result
        .posts
        .into_iter()
        .map(|pwa| PostListItemResponse::from_post_with_author(pwa.post, pwa.author))
        .collect();

    Ok(ApiResponseBuilder::paginated(
        posts,
        result.total,
        pagination.page,
        pagination.per_page,
    ))
}

/// GET /posts/:id - Détail d'un post avec nested objects
#[utoipa::path(
    get,
    path = "/posts/{id}",
    tag = "posts",
    params(
        ("id" = i32, Path, description = "Post ID")
    ),
    responses(
        (status = 200, description = "Post trouvé", body = inline(ApiResponse<PostResponse>)),
        (status = 404, description = "Post non trouvé", body = ErrorResponse),
        (status = 500, description = "Erreur serveur", body = ErrorResponse)
    )
)]
async fn get_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<ApiResponse<PostResponse>> {
    let result = state.post_service.find_by_id(id).await?;
    let response = PostResponse::from_post_with_author(result.post, result.author);
    Ok(ApiResponseBuilder::one(response))
}

/// POST /posts - Créer un post avec nested objects
///
/// # Exemple de body:
/// ```json
/// {
///     "title": "Mon article",
///     "content": "Contenu de l'article...",
///     "author_id": 1,
///     "published": false,
///     "metadata": {
///         "tags": [
///             { "name": "rust", "color": "#DEA584" },
///             { "name": "api", "color": "#3178C6" }
///         ],
///         "seo": {
///             "meta_title": "Mon article | Blog",
///             "meta_description": "Description pour les moteurs de recherche",
///             "keywords": ["rust", "api", "tutorial"]
///         },
///         "settings": {
///             "allow_comments": true,
///             "featured": false,
///             "reading_time_minutes": 5
///         }
///     }
/// }
/// ```
#[utoipa::path(
    post,
    path = "/posts",
    tag = "posts",
    request_body = CreatePostDto,
    responses(
        (status = 201, description = "Post créé", body = inline(ApiResponse<PostResponse>)),
        (status = 404, description = "Auteur non trouvé", body = ErrorResponse),
        (status = 422, description = "Erreur de validation", body = ErrorResponse),
        (status = 500, description = "Erreur serveur", body = ErrorResponse)
    )
)]
async fn create_post(
    State(state): State<Arc<AppState>>,
    ValidatedJson(dto): ValidatedJson<CreatePostDto>,
) -> ApiResult<(StatusCode, ApiResponse<PostResponse>)> {
    let result = state.post_service.create(dto).await?;
    let response = PostResponse::from_post_with_author(result.post, result.author);
    Ok(ApiResponseBuilder::created(response))
}

/// PUT /posts/:id - Modifier un post
#[utoipa::path(
    put,
    path = "/posts/{id}",
    tag = "posts",
    params(
        ("id" = i32, Path, description = "Post ID")
    ),
    request_body = UpdatePostDto,
    responses(
        (status = 200, description = "Post modifié", body = inline(ApiResponse<PostResponse>)),
        (status = 404, description = "Post non trouvé", body = ErrorResponse),
        (status = 422, description = "Erreur de validation", body = ErrorResponse),
        (status = 500, description = "Erreur serveur", body = ErrorResponse)
    )
)]
async fn update_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    ValidatedJson(dto): ValidatedJson<UpdatePostDto>,
) -> ApiResult<ApiResponse<PostResponse>> {
    let result = state.post_service.update(id, dto).await?;
    let response = PostResponse::from_post_with_author(result.post, result.author);
    Ok(ApiResponseBuilder::one(response))
}

/// DELETE /posts/:id - Supprimer un post
#[utoipa::path(
    delete,
    path = "/posts/{id}",
    tag = "posts",
    params(
        ("id" = i32, Path, description = "Post ID")
    ),
    responses(
        (status = 204, description = "Post supprimé"),
        (status = 404, description = "Post non trouvé", body = ErrorResponse),
        (status = 500, description = "Erreur serveur", body = ErrorResponse)
    )
)]
async fn delete_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<StatusCode> {
    state.post_service.delete(id).await?;
    Ok(ApiResponseBuilder::no_content())
}
