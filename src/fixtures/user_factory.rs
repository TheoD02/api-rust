// src/fixtures/user_factory.rs
// Factory pour l'entité User - inspiré de zenstruck/foundry

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};

use crate::entities::user;

use super::factory::{next_sequence, Factory};

/// UserFactory - Factory pour créer des utilisateurs de test
///
/// # Exemples
///
/// ```ignore
/// // Créer un utilisateur avec des valeurs par défaut
/// let user = UserFactory::new().create(&db).await?;
///
/// // Créer un utilisateur avec des valeurs personnalisées
/// let admin = UserFactory::new()
///     .with_username("admin")
///     .with_email("admin@example.com")
///     .create(&db)
///     .await?;
///
/// // Créer plusieurs utilisateurs
/// let users = UserFactory::new()
///     .create_many(&db, 10)
///     .await?;
///
/// // Créer sans persister (pour tests unitaires)
/// let user_model = UserFactory::new()
///     .with_username("test")
///     .make();
/// ```
#[derive(Clone)]
pub struct UserFactory {
    username: Option<String>,
    email: Option<String>,
    created_at: Option<chrono::NaiveDateTime>,
}

impl UserFactory {
    /// Définit le username
    ///
    /// ```ignore
    /// let user = UserFactory::new()
    ///     .with_username("john_doe")
    ///     .create(&db)
    ///     .await?;
    /// ```
    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    /// Définit l'email
    ///
    /// ```ignore
    /// let user = UserFactory::new()
    ///     .with_email("john@example.com")
    ///     .create(&db)
    ///     .await?;
    /// ```
    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// Définit la date de création
    ///
    /// ```ignore
    /// let user = UserFactory::new()
    ///     .with_created_at(chrono::Utc::now().naive_utc())
    ///     .create(&db)
    ///     .await?;
    /// ```
    pub fn with_created_at(mut self, created_at: chrono::NaiveDateTime) -> Self {
        self.created_at = Some(created_at);
        self
    }

    /// Génère un username par défaut avec séquence
    fn default_username() -> String {
        format!("user_{}", next_sequence())
    }

    /// Génère un email par défaut avec séquence
    fn default_email() -> String {
        format!("user_{}@example.com", next_sequence())
    }

    /// Construit l'ActiveModel pour SeaORM
    fn build_active_model(&self) -> user::ActiveModel {
        user::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            username: Set(self.username.clone().unwrap_or_else(Self::default_username)),
            email: Set(self.email.clone().unwrap_or_else(Self::default_email)),
            created_at: Set(self.created_at.unwrap_or_else(|| Utc::now().naive_utc())),
        }
    }
}

#[async_trait]
impl Factory for UserFactory {
    type Model = user::Model;
    type Entity = user::Entity;

    fn new() -> Self {
        Self {
            username: None,
            email: None,
            created_at: None,
        }
    }

    async fn create(&self, db: &DatabaseConnection) -> Result<Self::Model, DbErr> {
        let active_model = self.build_active_model();
        active_model.insert(db).await
    }

    fn make(&self) -> Self::Model {
        let seq = next_sequence();
        user::Model {
            id: seq as i32,
            username: self.username.clone().unwrap_or_else(|| format!("user_{}", seq)),
            email: self.email.clone().unwrap_or_else(|| format!("user_{}@example.com", seq)),
            created_at: self.created_at.unwrap_or_else(|| Utc::now().naive_utc()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixtures::factory::reset_sequence;

    #[test]
    fn test_make_creates_user_with_defaults() {
        reset_sequence();
        let user = UserFactory::new().make();

        assert!(user.username.starts_with("user_"));
        assert!(user.email.contains("@example.com"));
    }

    #[test]
    fn test_make_with_custom_values() {
        let user = UserFactory::new()
            .with_username("custom_user")
            .with_email("custom@test.com")
            .make();

        assert_eq!(user.username, "custom_user");
        assert_eq!(user.email, "custom@test.com");
    }

    #[test]
    fn test_make_many_creates_multiple_users() {
        reset_sequence();
        let users = UserFactory::new().make_many(5);

        assert_eq!(users.len(), 5);
        // Chaque user a un ID unique
        let ids: std::collections::HashSet<_> = users.iter().map(|u| u.id).collect();
        assert_eq!(ids.len(), 5);
    }

    #[test]
    fn test_factory_is_cloneable() {
        let factory = UserFactory::new().with_username("base");
        let cloned = factory.clone();

        let user1 = factory.make();
        let user2 = cloned.make();

        assert_eq!(user1.username, user2.username);
    }
}
