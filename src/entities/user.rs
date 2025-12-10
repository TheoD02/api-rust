// src/entities/user.rs
// Equivalent de: src/Entity/User.php

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// User Entity
/// Equivalent de: #[ORM\Entity] class User en Symfony/Doctrine
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    /// Primary key
    /// Equivalent de: #[ORM\Id] #[ORM\GeneratedValue]
    #[sea_orm(primary_key)]
    pub id: i32,

    /// Username
    /// Equivalent de: #[ORM\Column(length: 255)]
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub username: String,

    /// Email (unique)
    /// Equivalent de: #[ORM\Column(length: 255, unique: true)]
    #[sea_orm(column_type = "String(StringLen::N(255))", unique)]
    pub email: String,

    /// Creation timestamp
    /// Equivalent de: #[ORM\Column]
    pub created_at: DateTime,
}

/// Relations
/// Equivalent de: #[ORM\OneToMany], #[ORM\ManyToOne], etc.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::post::Entity")]
    Posts,
}

/// Relation: User has many Posts
impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Posts.def()
    }
}

/// Active Model Behavior
impl ActiveModelBehavior for ActiveModel {}
