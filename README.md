# Rust API

API REST moderne construite avec **Axum** et **SeaORM**, inspirée de l'architecture Symfony/PHP.

## Stack Technique

| Composant | Technologie |
|-----------|-------------|
| Framework Web | Axum 0.7 |
| ORM | SeaORM 1 |
| Base de données | SQLite (PostgreSQL/MySQL supportés) |
| Validation | validator 0.19 |
| Documentation | utoipa (OpenAPI/Swagger) |
| Logging | tracing |
| Tests | axum-test |

## Installation

### Prérequis

- Rust 1.70+ avec Cargo
- SQLite 3+ (ou PostgreSQL/MySQL)

### Démarrage rapide

```bash
# Cloner le projet
git clone <repo-url>
cd rust-api

# Compiler et lancer
cargo run

# Le serveur démarre sur http://localhost:8080
```

### Variables d'environnement

```bash
# Base de données (défaut: SQLite local)
DATABASE_URL=sqlite:./database.sqlite?mode=rwc

# Adresse du serveur (défaut: 0.0.0.0:8080)
SERVER_ADDR=0.0.0.0:8080

# Niveau de logs
RUST_LOG=rust_api=info,tower_http=info,sea_orm=warn
```

### Docker

```bash
docker build -t rust-api .
docker run -p 8080:8080 rust-api
```

## Structure du Projet

```
src/
├── main.rs              # Point d'entrée
├── lib.rs               # Exports pour les tests
├── config/              # Configuration
│   ├── app_state.rs     # Container de services
│   ├── database.rs      # Connexion DB + migrations
│   ├── logging.rs       # Configuration tracing
│   └── openapi.rs       # Documentation Swagger
├── controllers/         # Handlers HTTP
├── services/            # Logique métier
├── entities/            # Modèles SeaORM
├── dto/                 # Data Transfer Objects
├── validation/          # Validation des requêtes
├── error/               # Gestion des erreurs
├── response/            # Formatage des réponses
└── fixtures/            # Factories pour les tests
```

## Endpoints

| Méthode | Route | Description |
|---------|-------|-------------|
| GET | `/health` | Health check |
| GET | `/users` | Liste paginée |
| GET | `/users/:id` | Détail utilisateur |
| POST | `/users` | Créer utilisateur |
| PUT | `/users/:id` | Modifier utilisateur |
| DELETE | `/users/:id` | Supprimer utilisateur |

**Documentation Swagger:** http://localhost:8080/swagger-ui/

---

# Guide de Développement

## Créer une nouvelle Entité (CRUD complet)

### 1. Migration

Créer la migration pour la table :

```bash
cd migration
cargo run -- generate create_posts_table
```

Éditer le fichier généré `migration/src/m20241210_XXXXXX_create_posts_table.rs` :

```rust
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Posts::Table)
                    .if_not_exists()
                    .col(pk_auto(Posts::Id))
                    .col(string(Posts::Title))
                    .col(text(Posts::Content))
                    .col(integer(Posts::UserId))
                    .col(timestamp(Posts::CreatedAt))
                    .col(timestamp_null(Posts::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Posts::Table, Posts::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
    Title,
    Content,
    UserId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
```

Enregistrer dans `migration/src/lib.rs` :

```rust
mod m20241210_XXXXXX_create_posts_table;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241210_000001_create_users_table::Migration),
            Box::new(m20241210_XXXXXX_create_posts_table::Migration), // Ajouter ici
        ]
    }
}
```

Appliquer la migration :

```bash
cargo run -- up
```

### 2. Entity

Créer `src/entities/post.rs` :

```rust
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub title: String,

    #[sea_orm(column_type = "Text")]
    pub content: String,

    pub user_id: i32,

    pub created_at: DateTime,

    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
```

Exporter dans `src/entities/mod.rs` :

```rust
pub mod post;
pub mod user;
```

### 3. DTOs (Input/Output/Validation)

Créer `src/dto/post.rs` :

```rust
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// DTO pour créer un post (INPUT)
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreatePostDto {
    #[validate(length(min = 3, max = 255, message = "Le titre doit faire entre 3 et 255 caractères"))]
    pub title: String,

    #[validate(length(min = 10, message = "Le contenu doit faire au moins 10 caractères"))]
    pub content: String,

    #[validate(range(min = 1, message = "L'ID utilisateur doit être positif"))]
    pub user_id: i32,
}

/// DTO pour modifier un post (INPUT)
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatePostDto {
    #[validate(length(min = 3, max = 255, message = "Le titre doit faire entre 3 et 255 caractères"))]
    pub title: Option<String>,

    #[validate(length(min = 10, message = "Le contenu doit faire au moins 10 caractères"))]
    pub content: Option<String>,
}

/// DTO de réponse (OUTPUT)
#[derive(Debug, Serialize, ToSchema)]
pub struct PostResponse {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

/// Conversion automatique Entity -> Response
impl From<crate::entities::post::Model> for PostResponse {
    fn from(post: crate::entities::post::Model) -> Self {
        Self {
            id: post.id,
            title: post.title,
            content: post.content,
            user_id: post.user_id,
            created_at: post.created_at,
            updated_at: post.updated_at,
        }
    }
}
```

Exporter dans `src/dto/mod.rs` :

```rust
mod pagination;
mod post;
mod user;

pub use pagination::PaginationQuery;
pub use post::{CreatePostDto, PostResponse, UpdatePostDto};
pub use user::{CreateUserDto, UpdateUserDto, UserResponse};
```

### 4. Service (Logique Métier)

Créer `src/services/post_service.rs` :

```rust
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use tracing::info;

use crate::dto::{CreatePostDto, PaginationQuery, UpdatePostDto};
use crate::entities::post;
use crate::error::ServiceError;

pub struct PaginatedPosts {
    pub posts: Vec<post::Model>,
    pub total: u64,
}

#[derive(Clone)]
pub struct PostService {
    db: DatabaseConnection,
}

impl PostService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Liste paginée des posts
    pub async fn find_all(&self, pagination: &PaginationQuery) -> Result<PaginatedPosts, ServiceError> {
        let total = post::Entity::find().count(&self.db).await?;

        let posts = post::Entity::find()
            .order_by_desc(post::Column::CreatedAt)
            .offset(pagination.offset())
            .limit(pagination.limit())
            .all(&self.db)
            .await?;

        Ok(PaginatedPosts { posts, total })
    }

    /// Trouver par ID
    pub async fn find_by_id(&self, id: i32) -> Result<post::Model, ServiceError> {
        post::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(ServiceError::NotFound)
    }

    /// Créer un post
    pub async fn create(&self, dto: CreatePostDto) -> Result<post::Model, ServiceError> {
        info!(title = %dto.title, user_id = dto.user_id, "Creating post");

        let new_post = post::ActiveModel {
            title: Set(dto.title),
            content: Set(dto.content),
            user_id: Set(dto.user_id),
            created_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };

        let post = new_post.insert(&self.db).await?;
        info!(post_id = post.id, "Post created");

        Ok(post)
    }

    /// Modifier un post
    pub async fn update(&self, id: i32, dto: UpdatePostDto) -> Result<post::Model, ServiceError> {
        let existing = self.find_by_id(id).await?;

        let mut active_model: post::ActiveModel = existing.into();

        if let Some(title) = dto.title {
            active_model.title = Set(title);
        }
        if let Some(content) = dto.content {
            active_model.content = Set(content);
        }
        active_model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));

        let updated = active_model.update(&self.db).await?;
        info!(post_id = updated.id, "Post updated");

        Ok(updated)
    }

    /// Supprimer un post
    pub async fn delete(&self, id: i32) -> Result<(), ServiceError> {
        let post = self.find_by_id(id).await?;
        post::Entity::delete_by_id(post.id).exec(&self.db).await?;
        info!(post_id = id, "Post deleted");
        Ok(())
    }

    /// Trouver les posts d'un utilisateur
    pub async fn find_by_user(&self, user_id: i32) -> Result<Vec<post::Model>, ServiceError> {
        let posts = post::Entity::find()
            .filter(post::Column::UserId.eq(user_id))
            .order_by_desc(post::Column::CreatedAt)
            .all(&self.db)
            .await?;

        Ok(posts)
    }
}
```

Exporter dans `src/services/mod.rs` :

```rust
mod post_service;
mod user_service;

pub use post_service::{PaginatedPosts, PostService};
pub use user_service::{PaginatedUsers, UserService};
```

### 5. Controller (Endpoints HTTP)

Créer `src/controllers/post_controller.rs` :

```rust
use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};

use crate::config::AppState;
use crate::dto::{CreatePostDto, PaginationQuery, PostResponse, UpdatePostDto};
use crate::error::ApiResult;
use crate::response::{ApiResponse, ApiResponseBuilder, PaginatedResponse};
use crate::validation::ValidatedJson;

pub struct PostController;

impl PostController {
    pub fn routes() -> Router<Arc<AppState>> {
        Router::new()
            .route("/posts", get(list_posts))
            .route("/posts", post(create_post))
            .route("/posts/{id}", get(get_post))
            .route("/posts/{id}", put(update_post))
            .route("/posts/{id}", delete(delete_post))
    }
}

/// GET /posts - Liste paginée
#[utoipa::path(
    get,
    path = "/posts",
    tag = "posts",
    params(PaginationQuery),
    responses(
        (status = 200, description = "Liste des posts", body = PaginatedResponse<PostResponse>)
    )
)]
async fn list_posts(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationQuery>,
) -> ApiResult<Json<PaginatedResponse<PostResponse>>> {
    let result = state.post_service.find_all(&pagination).await?;

    let posts: Vec<PostResponse> = result.posts.into_iter().map(Into::into).collect();

    Ok(Json(ApiResponseBuilder::paginated(
        posts,
        result.total,
        pagination.page,
        pagination.per_page,
    )))
}

/// GET /posts/:id - Détail
#[utoipa::path(
    get,
    path = "/posts/{id}",
    tag = "posts",
    params(("id" = i32, Path, description = "Post ID")),
    responses(
        (status = 200, description = "Post trouvé", body = ApiResponse<PostResponse>),
        (status = 404, description = "Post non trouvé")
    )
)]
async fn get_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<Json<ApiResponse<PostResponse>>> {
    let post = state.post_service.find_by_id(id).await?;
    Ok(Json(ApiResponse::data(post.into())))
}

/// POST /posts - Créer
#[utoipa::path(
    post,
    path = "/posts",
    tag = "posts",
    request_body = CreatePostDto,
    responses(
        (status = 201, description = "Post créé", body = ApiResponse<PostResponse>),
        (status = 422, description = "Erreur de validation")
    )
)]
async fn create_post(
    State(state): State<Arc<AppState>>,
    ValidatedJson(dto): ValidatedJson<CreatePostDto>,
) -> ApiResult<(StatusCode, Json<ApiResponse<PostResponse>>)> {
    let post = state.post_service.create(dto).await?;
    Ok(ApiResponseBuilder::created(post.into()))
}

/// PUT /posts/:id - Modifier
#[utoipa::path(
    put,
    path = "/posts/{id}",
    tag = "posts",
    params(("id" = i32, Path, description = "Post ID")),
    request_body = UpdatePostDto,
    responses(
        (status = 200, description = "Post modifié", body = ApiResponse<PostResponse>),
        (status = 404, description = "Post non trouvé"),
        (status = 422, description = "Erreur de validation")
    )
)]
async fn update_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    ValidatedJson(dto): ValidatedJson<UpdatePostDto>,
) -> ApiResult<Json<ApiResponse<PostResponse>>> {
    let post = state.post_service.update(id, dto).await?;
    Ok(Json(ApiResponse::data(post.into())))
}

/// DELETE /posts/:id - Supprimer
#[utoipa::path(
    delete,
    path = "/posts/{id}",
    tag = "posts",
    params(("id" = i32, Path, description = "Post ID")),
    responses(
        (status = 204, description = "Post supprimé"),
        (status = 404, description = "Post non trouvé")
    )
)]
async fn delete_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<StatusCode> {
    state.post_service.delete(id).await?;
    Ok(ApiResponseBuilder::no_content())
}
```

### 6. Enregistrer les routes

Mettre à jour `src/main.rs` :

```rust
fn build_router(state: Arc<AppState>) -> Router {
    let user_routes = UserController::routes();
    let post_routes = PostController::routes();  // Ajouter
    let health_routes = HealthController::routes();

    Router::new()
        .merge(user_routes)
        .merge(post_routes)  // Ajouter
        .with_state(state)
        .merge(health_routes)
        // ...
}
```

Mettre à jour `src/config/app_state.rs` :

```rust
use crate::services::{PostService, UserService};

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub post_service: PostService,  // Ajouter
}

impl AppState {
    pub fn new(user_service: UserService, post_service: PostService) -> Self {
        Self { user_service, post_service }
    }
}
```

### 7. Documentation OpenAPI

Mettre à jour `src/config/openapi.rs` :

```rust
#[derive(OpenApi)]
#[openapi(
    paths(
        // ... existing paths
        list_posts, get_post, create_post, update_post, delete_post
    ),
    components(schemas(
        // ... existing schemas
        CreatePostDto, UpdatePostDto, PostResponse
    )),
    tags(
        // ... existing tags
        (name = "posts", description = "Gestion des posts")
    )
)]
pub struct ApiDoc;
```

---

## Validation

### Règles disponibles

```rust
use validator::Validate;

#[derive(Validate)]
pub struct MyDto {
    // Longueur
    #[validate(length(min = 3, max = 100))]
    pub name: String,

    // Email
    #[validate(email)]
    pub email: String,

    // URL
    #[validate(url)]
    pub website: Option<String>,

    // Range numérique
    #[validate(range(min = 0, max = 150))]
    pub age: i32,

    // Regex
    #[validate(regex(path = *PHONE_REGEX))]
    pub phone: String,

    // Obligatoire (non-Option)
    #[validate(required)]
    pub required_field: Option<String>,

    // Nested validation
    #[validate(nested)]
    pub address: AddressDto,

    // Custom message
    #[validate(length(min = 8, message = "Le mot de passe doit faire au moins 8 caractères"))]
    pub password: String,
}
```

### Réponse d'erreur de validation

```json
{
  "error": "Validation failed",
  "violations": [
    {
      "field": "username",
      "messages": ["Le nom doit faire entre 3 et 50 caractères"]
    },
    {
      "field": "email",
      "messages": ["Format email invalide"]
    }
  ]
}
```

---

## Fixtures (Données de Test)

### Créer une Factory

`src/fixtures/post_factory.rs` :

```rust
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};

use crate::entities::post;
use super::factory::{next_sequence, Factory};

#[derive(Clone)]
pub struct PostFactory {
    title: Option<String>,
    content: Option<String>,
    user_id: Option<i32>,
}

impl PostFactory {
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn with_user_id(mut self, user_id: i32) -> Self {
        self.user_id = Some(user_id);
        self
    }
}

#[async_trait]
impl Factory for PostFactory {
    type Model = post::Model;
    type Entity = post::Entity;

    fn new() -> Self {
        Self {
            title: None,
            content: None,
            user_id: None,
        }
    }

    async fn create(&self, db: &DatabaseConnection) -> Result<Self::Model, DbErr> {
        let seq = next_sequence();
        let active_model = post::ActiveModel {
            title: Set(self.title.clone().unwrap_or_else(|| format!("Post {}", seq))),
            content: Set(self.content.clone().unwrap_or_else(|| format!("Content for post {}", seq))),
            user_id: Set(self.user_id.unwrap_or(1)),
            created_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };
        active_model.insert(db).await
    }

    fn make(&self) -> Self::Model {
        let seq = next_sequence();
        post::Model {
            id: seq as i32,
            title: self.title.clone().unwrap_or_else(|| format!("Post {}", seq)),
            content: self.content.clone().unwrap_or_else(|| format!("Content {}", seq)),
            user_id: self.user_id.unwrap_or(1),
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}
```

### Utilisation

```rust
use crate::fixtures::{Factory, UserFactory, PostFactory};

// Créer un utilisateur puis ses posts
let user = UserFactory::new()
    .with_username("author")
    .create(&db)
    .await?;

let posts = PostFactory::new()
    .with_user_id(user.id)
    .create_many(&db, 5)
    .await?;

// Sans base de données (tests unitaires)
let post = PostFactory::new()
    .with_title("Test Post")
    .make();
```

---

## Tests

### Structure

```
tests/
├── common/
│   └── mod.rs           # Utilitaires (create_test_server)
├── health_controller_test.rs
└── user_controller_test.rs
```

### Exemple de test

```rust
use axum::http::StatusCode;
use serde_json::json;

mod common;

#[tokio::test]
async fn test_create_post_success() {
    let server = common::create_test_server().await;

    // Créer d'abord un utilisateur
    let user_response = server
        .post("/users")
        .json(&json!({
            "username": "author",
            "email": "author@test.com"
        }))
        .await;

    let user: serde_json::Value = user_response.json();
    let user_id = user["data"]["id"].as_i64().unwrap();

    // Créer le post
    let response = server
        .post("/posts")
        .json(&json!({
            "title": "Mon premier post",
            "content": "Contenu du post avec au moins 10 caractères",
            "user_id": user_id
        }))
        .await;

    response.assert_status(StatusCode::CREATED);

    let body: serde_json::Value = response.json();
    assert_eq!(body["data"]["title"], "Mon premier post");
}

#[tokio::test]
async fn test_create_post_validation_error() {
    let server = common::create_test_server().await;

    let response = server
        .post("/posts")
        .json(&json!({
            "title": "AB",  // Trop court
            "content": "Court",  // Trop court
            "user_id": 1
        }))
        .await;

    response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
}
```

### Lancer les tests

```bash
# Tous les tests
cargo test

# Un fichier spécifique
cargo test --test post_controller_test

# Un test spécifique
cargo test test_create_post_success

# Avec logs
RUST_LOG=debug cargo test -- --nocapture
```

---

## Migrations

### Commandes

```bash
cd migration

# Créer une migration
cargo run -- generate create_comments_table

# Appliquer toutes les migrations
cargo run -- up

# Rollback de la dernière migration
cargo run -- down

# Tout recréer (drop + migrate)
cargo run -- fresh

# Voir le statut
cargo run -- status
```

### Types de colonnes

```rust
// migration/src/m20241210_XXXXXX_example.rs
use sea_orm_migration::{prelude::*, schema::*};

Table::create()
    .col(pk_auto(Posts::Id))           // INTEGER PRIMARY KEY AUTOINCREMENT
    .col(string(Posts::Title))          // VARCHAR(255)
    .col(string_len(Posts::Code, 50))   // VARCHAR(50)
    .col(string_uniq(Posts::Slug))      // VARCHAR(255) UNIQUE
    .col(text(Posts::Content))          // TEXT
    .col(integer(Posts::Count))         // INTEGER
    .col(big_integer(Posts::Views))     // BIGINT
    .col(float(Posts::Rating))          // FLOAT
    .col(boolean(Posts::Published))     // BOOLEAN
    .col(timestamp(Posts::CreatedAt))   // TIMESTAMP NOT NULL
    .col(timestamp_null(Posts::DeletedAt)) // TIMESTAMP NULL
    .col(json(Posts::Metadata))         // JSON
```

---

## Commandes Utiles

```bash
# Développement
cargo run                    # Démarrer le serveur
cargo watch -x run           # Hot reload (nécessite cargo-watch)
cargo check                  # Vérifier la compilation
cargo clippy                 # Linter

# Tests
cargo test                   # Lancer tous les tests
cargo test -- --nocapture    # Avec output

# Production
cargo build --release        # Build optimisé
./target/release/rust-api    # Lancer le binaire

# Base de données
cd migration && cargo run -- up     # Migrer
cd migration && cargo run -- down   # Rollback
cd migration && cargo run -- fresh  # Recréer
```

---

## Architecture des Erreurs

```
ServiceError (couche métier)
    ↓ conversion automatique
ApiError (couche HTTP)
    ↓ IntoResponse
Réponse HTTP JSON
```

| ServiceError | ApiError | HTTP Status |
|--------------|----------|-------------|
| NotFound | NotFound | 404 |
| AlreadyExists | Conflict | 409 |
| Database | DatabaseError | 500 |

---

## Licence

MIT
