// src/fixtures/factory.rs
// Trait Factory générique inspiré de zenstruck/foundry

use async_trait::async_trait;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};
use std::sync::atomic::{AtomicU64, Ordering};

/// Compteur global pour les séquences
/// Utilisé pour générer des valeurs uniques (emails, usernames, etc.)
static SEQUENCE_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Réinitialise le compteur de séquence (utile pour les tests)
pub fn reset_sequence() {
    SEQUENCE_COUNTER.store(1, Ordering::SeqCst);
}

/// Retourne la prochaine valeur de séquence
pub fn next_sequence() -> u64 {
    SEQUENCE_COUNTER.fetch_add(1, Ordering::SeqCst)
}

/// Trait Factory - inspiré de zenstruck/foundry
///
/// # Exemple d'utilisation
/// ```ignore
/// let user = UserFactory::new()
///     .with_username("john")
///     .with_email("john@example.com")
///     .create(&db)
///     .await?;
/// ```
#[async_trait]
pub trait Factory: Sized + Clone + Send + Sync {
    /// Le type de modèle SeaORM (ex: user::Model)
    type Model: Send;

    /// L'entité SeaORM (ex: user::Entity)
    type Entity: EntityTrait<Model = Self::Model>;

    /// Crée une nouvelle instance de la factory avec des valeurs par défaut
    fn new() -> Self;

    /// Crée une entité en base de données
    async fn create(&self, db: &DatabaseConnection) -> Result<Self::Model, DbErr>;

    /// Crée plusieurs entités en base de données
    ///
    /// # Exemple
    /// ```ignore
    /// let users = UserFactory::new()
    ///     .create_many(&db, 10)
    ///     .await?;
    /// ```
    async fn create_many(&self, db: &DatabaseConnection, count: usize) -> Result<Vec<Self::Model>, DbErr> {
        let mut results = Vec::with_capacity(count);
        for _ in 0..count {
            // Clone la factory pour chaque création (génère de nouvelles séquences)
            let entity = self.clone().create(db).await?;
            results.push(entity);
        }
        Ok(results)
    }

    /// Crée une entité sans la persister (pour les tests unitaires)
    fn make(&self) -> Self::Model;

    /// Crée plusieurs entités sans les persister
    fn make_many(&self, count: usize) -> Vec<Self::Model> {
        (0..count).map(|_| self.clone().make()).collect()
    }
}

/// Trait pour les factories avec callbacks
/// Permet d'exécuter du code après la création
#[async_trait]
pub trait FactoryWithCallback: Factory {
    /// Callback exécuté après la création de l'entité
    async fn after_create(&self, _model: &Self::Model, _db: &DatabaseConnection) -> Result<(), DbErr> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_increments() {
        reset_sequence();
        assert_eq!(next_sequence(), 1);
        assert_eq!(next_sequence(), 2);
        assert_eq!(next_sequence(), 3);
    }

    #[test]
    fn test_reset_sequence() {
        next_sequence();
        next_sequence();
        reset_sequence();
        assert_eq!(next_sequence(), 1);
    }
}
