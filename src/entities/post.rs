// src/entities/post.rs
// Entity Post avec champ JSON pour nested objects

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Metadata imbriquée stockée en JSON
/// Contient les tags et autres métadonnées du post
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostMetadata {
    /// Liste des tags
    pub tags: Vec<Tag>,
    /// SEO metadata
    pub seo: Option<SeoMetadata>,
    /// Paramètres additionnels
    pub settings: Option<PostSettings>,
}

/// Tag avec nom et couleur
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub color: Option<String>,
}

/// SEO metadata nested object
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeoMetadata {
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub keywords: Vec<String>,
}

/// Settings nested object
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostSettings {
    pub allow_comments: bool,
    pub featured: bool,
    pub reading_time_minutes: Option<i32>,
}

impl Default for PostMetadata {
    fn default() -> Self {
        Self {
            tags: vec![],
            seo: None,
            settings: None,
        }
    }
}

/// Post Entity
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub title: String,

    #[sea_orm(column_type = "Text")]
    pub content: String,

    /// Foreign key vers User
    pub author_id: i32,

    /// JSON field contenant les nested objects (tags, seo, settings)
    #[sea_orm(column_type = "Json")]
    pub metadata: serde_json::Value,

    pub published: bool,

    pub created_at: DateTime,

    pub updated_at: Option<DateTime>,
}

impl Model {
    /// Parse le JSON metadata en struct typée
    pub fn get_metadata(&self) -> PostMetadata {
        serde_json::from_value(self.metadata.clone()).unwrap_or_default()
    }

    /// Helper pour récupérer les tags
    pub fn get_tags(&self) -> Vec<Tag> {
        self.get_metadata().tags
    }
}

/// Relations
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::AuthorId",
        to = "super::user::Column::Id"
    )]
    Author,
}

/// Relation inverse: Post appartient à User
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Author.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
