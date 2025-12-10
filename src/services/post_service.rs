// src/services/post_service.rs
// Service pour la gestion des posts avec nested objects

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};
use tracing::{info, warn};

use crate::dto::{CreatePostDto, PaginationQuery, UpdatePostDto};
use crate::entities::{post, user};
use crate::error::ServiceError;

/// Post avec son auteur chargé
pub struct PostWithAuthor {
    pub post: post::Model,
    pub author: user::Model,
}

/// Résultat paginé de posts
pub struct PaginatedPosts {
    pub posts: Vec<PostWithAuthor>,
    pub total: u64,
}

/// PostService - Logique métier pour les posts
#[derive(Clone)]
pub struct PostService {
    db: DatabaseConnection,
}

impl PostService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Liste paginée des posts avec leurs auteurs
    pub async fn find_all(&self, pagination: &PaginationQuery) -> Result<PaginatedPosts, ServiceError> {
        info!(page = pagination.page, per_page = pagination.per_page, "Fetching posts");

        let total = post::Entity::find().count(&self.db).await?;

        let posts = post::Entity::find()
            .order_by_desc(post::Column::CreatedAt)
            .offset(pagination.offset())
            .limit(pagination.limit())
            .all(&self.db)
            .await?;

        // Charger les auteurs pour chaque post
        let mut posts_with_authors = Vec::with_capacity(posts.len());
        for p in posts {
            let author = user::Entity::find_by_id(p.author_id)
                .one(&self.db)
                .await?
                .ok_or(ServiceError::NotFound)?;

            posts_with_authors.push(PostWithAuthor { post: p, author });
        }

        info!(count = posts_with_authors.len(), total = total, "Posts fetched");

        Ok(PaginatedPosts {
            posts: posts_with_authors,
            total,
        })
    }

    /// Trouver un post par ID avec son auteur
    pub async fn find_by_id(&self, id: i32) -> Result<PostWithAuthor, ServiceError> {
        info!(post_id = id, "Fetching post by ID");

        let post = post::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| {
                warn!(post_id = id, "Post not found");
                ServiceError::NotFound
            })?;

        let author = user::Entity::find_by_id(post.author_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| {
                warn!(author_id = post.author_id, "Author not found");
                ServiceError::NotFound
            })?;

        info!(post_id = id, title = %post.title, "Post found");

        Ok(PostWithAuthor { post, author })
    }

    /// Créer un nouveau post
    pub async fn create(&self, dto: CreatePostDto) -> Result<PostWithAuthor, ServiceError> {
        info!(title = %dto.title, author_id = dto.author_id, "Creating post");

        // Vérifier que l'auteur existe
        let author = user::Entity::find_by_id(dto.author_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| {
                warn!(author_id = dto.author_id, "Author not found");
                ServiceError::NotFound
            })?;

        // Convertir metadata DTO en JSON
        let metadata_json = dto
            .metadata
            .unwrap_or_default()
            .to_json();

        let new_post = post::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            title: Set(dto.title),
            content: Set(dto.content),
            author_id: Set(dto.author_id),
            metadata: Set(metadata_json),
            published: Set(dto.published),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(None),
        };

        let post = new_post.insert(&self.db).await?;

        info!(post_id = post.id, title = %post.title, "Post created");

        Ok(PostWithAuthor { post, author })
    }

    /// Modifier un post
    pub async fn update(&self, id: i32, dto: UpdatePostDto) -> Result<PostWithAuthor, ServiceError> {
        info!(post_id = id, "Updating post");

        let existing = post::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| {
                warn!(post_id = id, "Post not found for update");
                ServiceError::NotFound
            })?;

        let author = user::Entity::find_by_id(existing.author_id)
            .one(&self.db)
            .await?
            .ok_or(ServiceError::NotFound)?;

        let mut active_model: post::ActiveModel = existing.into();

        if let Some(title) = dto.title {
            active_model.title = Set(title);
        }
        if let Some(content) = dto.content {
            active_model.content = Set(content);
        }
        if let Some(metadata) = dto.metadata {
            active_model.metadata = Set(metadata.to_json());
        }
        if let Some(published) = dto.published {
            active_model.published = Set(published);
        }

        active_model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));

        let updated = active_model.update(&self.db).await?;

        info!(post_id = id, "Post updated");

        Ok(PostWithAuthor {
            post: updated,
            author,
        })
    }

    /// Supprimer un post
    pub async fn delete(&self, id: i32) -> Result<(), ServiceError> {
        info!(post_id = id, "Deleting post");

        let result = post::Entity::delete_by_id(id).exec(&self.db).await?;

        if result.rows_affected == 0 {
            warn!(post_id = id, "Post not found for deletion");
            return Err(ServiceError::NotFound);
        }

        info!(post_id = id, "Post deleted");
        Ok(())
    }

    /// Trouver les posts d'un auteur
    pub async fn find_by_author(&self, author_id: i32) -> Result<Vec<PostWithAuthor>, ServiceError> {
        info!(author_id = author_id, "Fetching posts by author");

        let author = user::Entity::find_by_id(author_id)
            .one(&self.db)
            .await?
            .ok_or(ServiceError::NotFound)?;

        let posts = post::Entity::find()
            .filter(post::Column::AuthorId.eq(author_id))
            .order_by_desc(post::Column::CreatedAt)
            .all(&self.db)
            .await?;

        let posts_with_authors = posts
            .into_iter()
            .map(|p| PostWithAuthor {
                post: p,
                author: author.clone(),
            })
            .collect();

        Ok(posts_with_authors)
    }

    /// Trouver les posts publiés uniquement
    pub async fn find_published(&self, pagination: &PaginationQuery) -> Result<PaginatedPosts, ServiceError> {
        let total = post::Entity::find()
            .filter(post::Column::Published.eq(true))
            .count(&self.db)
            .await?;

        let posts = post::Entity::find()
            .filter(post::Column::Published.eq(true))
            .order_by_desc(post::Column::CreatedAt)
            .offset(pagination.offset())
            .limit(pagination.limit())
            .all(&self.db)
            .await?;

        let mut posts_with_authors = Vec::with_capacity(posts.len());
        for p in posts {
            let author = user::Entity::find_by_id(p.author_id)
                .one(&self.db)
                .await?
                .ok_or(ServiceError::NotFound)?;

            posts_with_authors.push(PostWithAuthor { post: p, author });
        }

        Ok(PaginatedPosts {
            posts: posts_with_authors,
            total,
        })
    }
}
