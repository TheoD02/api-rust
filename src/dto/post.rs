// src/dto/post.rs
// DTOs pour Post avec nested objects et validation

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::entities::post::{PostMetadata, PostSettings, SeoMetadata, Tag};

// ============================================================================
// INPUT DTOs (Request Bodies)
// ============================================================================

/// DTO pour créer un tag (nested input)
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateTagDto {
    #[validate(length(min = 1, max = 50, message = "Le nom du tag doit faire entre 1 et 50 caractères"))]
    pub name: String,

    #[validate(length(max = 7, message = "La couleur doit être un code hex (ex: #FF0000)"))]
    pub color: Option<String>,
}

/// DTO pour les métadonnées SEO (nested input)
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateSeoMetadataDto {
    #[validate(length(max = 70, message = "Le meta title ne doit pas dépasser 70 caractères"))]
    pub meta_title: Option<String>,

    #[validate(length(max = 160, message = "La meta description ne doit pas dépasser 160 caractères"))]
    pub meta_description: Option<String>,

    #[validate(length(max = 10, message = "Maximum 10 keywords autorisés"))]
    pub keywords: Option<Vec<String>>,
}

/// DTO pour les settings du post (nested input)
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreatePostSettingsDto {
    #[serde(default)]
    pub allow_comments: bool,

    #[serde(default)]
    pub featured: bool,

    #[validate(range(min = 1, max = 60, message = "Le temps de lecture doit être entre 1 et 60 minutes"))]
    pub reading_time_minutes: Option<i32>,
}

/// DTO pour les metadata complètes (nested input)
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreatePostMetadataDto {
    /// Liste des tags (validation nested)
    #[validate(length(max = 10, message = "Maximum 10 tags autorisés"))]
    #[validate(nested)]
    pub tags: Option<Vec<CreateTagDto>>,

    /// SEO metadata (validation nested)
    #[validate(nested)]
    pub seo: Option<CreateSeoMetadataDto>,

    /// Settings (validation nested)
    #[validate(nested)]
    pub settings: Option<CreatePostSettingsDto>,
}

/// DTO pour créer un post (INPUT principal)
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreatePostDto {
    #[validate(length(min = 3, max = 255, message = "Le titre doit faire entre 3 et 255 caractères"))]
    pub title: String,

    #[validate(length(min = 10, message = "Le contenu doit faire au moins 10 caractères"))]
    pub content: String,

    #[validate(range(min = 1, message = "L'ID auteur doit être positif"))]
    pub author_id: i32,

    /// Metadata avec objets imbriqués (tags, seo, settings)
    #[validate(nested)]
    pub metadata: Option<CreatePostMetadataDto>,

    #[serde(default)]
    pub published: bool,
}

/// DTO pour modifier un post (INPUT)
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatePostDto {
    #[validate(length(min = 3, max = 255, message = "Le titre doit faire entre 3 et 255 caractères"))]
    pub title: Option<String>,

    #[validate(length(min = 10, message = "Le contenu doit faire au moins 10 caractères"))]
    pub content: Option<String>,

    /// Metadata avec objets imbriqués
    #[validate(nested)]
    pub metadata: Option<CreatePostMetadataDto>,

    pub published: Option<bool>,
}

// ============================================================================
// OUTPUT DTOs (Response Bodies)
// ============================================================================

/// Response DTO pour un tag
#[derive(Debug, Serialize, ToSchema)]
pub struct TagResponse {
    pub name: String,
    pub color: Option<String>,
}

/// Response DTO pour les métadonnées SEO
#[derive(Debug, Serialize, ToSchema)]
pub struct SeoMetadataResponse {
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub keywords: Vec<String>,
}

/// Response DTO pour les settings
#[derive(Debug, Serialize, ToSchema)]
pub struct PostSettingsResponse {
    pub allow_comments: bool,
    pub featured: bool,
    pub reading_time_minutes: Option<i32>,
}

/// Response DTO pour les metadata complètes
#[derive(Debug, Serialize, ToSchema)]
pub struct PostMetadataResponse {
    pub tags: Vec<TagResponse>,
    pub seo: Option<SeoMetadataResponse>,
    pub settings: Option<PostSettingsResponse>,
}

/// Response DTO pour l'auteur (nested dans PostResponse)
#[derive(Debug, Serialize, ToSchema)]
pub struct AuthorResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}

/// Response DTO pour un post (OUTPUT principal)
#[derive(Debug, Serialize, ToSchema)]
pub struct PostResponse {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,

    /// Auteur du post (nested object)
    pub author: AuthorResponse,

    /// Metadata avec objets imbriqués
    pub metadata: PostMetadataResponse,
}

/// Response simplifiée pour les listes (sans contenu complet)
#[derive(Debug, Serialize, ToSchema)]
pub struct PostListItemResponse {
    pub id: i32,
    pub title: String,
    /// Extrait du contenu (100 premiers caractères)
    pub excerpt: String,
    pub published: bool,
    pub created_at: chrono::NaiveDateTime,
    pub author: AuthorResponse,
    pub tags: Vec<TagResponse>,
}

// ============================================================================
// CONVERSIONS
// ============================================================================

impl From<Tag> for TagResponse {
    fn from(tag: Tag) -> Self {
        Self {
            name: tag.name,
            color: tag.color,
        }
    }
}

impl From<SeoMetadata> for SeoMetadataResponse {
    fn from(seo: SeoMetadata) -> Self {
        Self {
            meta_title: seo.meta_title,
            meta_description: seo.meta_description,
            keywords: seo.keywords,
        }
    }
}

impl From<PostSettings> for PostSettingsResponse {
    fn from(settings: PostSettings) -> Self {
        Self {
            allow_comments: settings.allow_comments,
            featured: settings.featured,
            reading_time_minutes: settings.reading_time_minutes,
        }
    }
}

impl From<PostMetadata> for PostMetadataResponse {
    fn from(metadata: PostMetadata) -> Self {
        Self {
            tags: metadata.tags.into_iter().map(Into::into).collect(),
            seo: metadata.seo.map(Into::into),
            settings: metadata.settings.map(Into::into),
        }
    }
}

impl From<crate::entities::user::Model> for AuthorResponse {
    fn from(user: crate::entities::user::Model) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
        }
    }
}

impl PostResponse {
    /// Crée une réponse à partir du post et de son auteur
    pub fn from_post_with_author(
        post: crate::entities::post::Model,
        author: crate::entities::user::Model,
    ) -> Self {
        let metadata = post.get_metadata();

        Self {
            id: post.id,
            title: post.title,
            content: post.content,
            published: post.published,
            created_at: post.created_at,
            updated_at: post.updated_at,
            author: author.into(),
            metadata: metadata.into(),
        }
    }
}

impl PostListItemResponse {
    /// Crée une réponse liste à partir du post et de son auteur
    pub fn from_post_with_author(
        post: crate::entities::post::Model,
        author: crate::entities::user::Model,
    ) -> Self {
        let metadata = post.get_metadata();
        let excerpt = if post.content.len() > 100 {
            format!("{}...", &post.content[..100])
        } else {
            post.content.clone()
        };

        Self {
            id: post.id,
            title: post.title,
            excerpt,
            published: post.published,
            created_at: post.created_at,
            author: author.into(),
            tags: metadata.tags.into_iter().map(Into::into).collect(),
        }
    }
}

// ============================================================================
// DTO -> Entity Conversions
// ============================================================================

impl From<CreateTagDto> for Tag {
    fn from(dto: CreateTagDto) -> Self {
        Self {
            name: dto.name,
            color: dto.color,
        }
    }
}

impl From<CreateSeoMetadataDto> for SeoMetadata {
    fn from(dto: CreateSeoMetadataDto) -> Self {
        Self {
            meta_title: dto.meta_title,
            meta_description: dto.meta_description,
            keywords: dto.keywords.unwrap_or_default(),
        }
    }
}

impl From<CreatePostSettingsDto> for PostSettings {
    fn from(dto: CreatePostSettingsDto) -> Self {
        Self {
            allow_comments: dto.allow_comments,
            featured: dto.featured,
            reading_time_minutes: dto.reading_time_minutes,
        }
    }
}

impl From<CreatePostMetadataDto> for PostMetadata {
    fn from(dto: CreatePostMetadataDto) -> Self {
        Self {
            tags: dto
                .tags
                .unwrap_or_default()
                .into_iter()
                .map(Into::into)
                .collect(),
            seo: dto.seo.map(Into::into),
            settings: dto.settings.map(Into::into),
        }
    }
}

impl CreatePostMetadataDto {
    /// Convertit en JSON Value pour stockage en DB
    pub fn to_json(&self) -> serde_json::Value {
        let metadata: PostMetadata = self.clone().into();
        serde_json::to_value(metadata).unwrap_or(serde_json::json!({}))
    }
}

impl Default for CreatePostMetadataDto {
    fn default() -> Self {
        Self {
            tags: None,
            seo: None,
            settings: None,
        }
    }
}
