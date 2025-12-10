// src/fixtures/mod.rs
// Système de fixtures inspiré de zenstruck/foundry (PHP)
// Permet de créer des données de test avec un builder pattern fluide

mod factory;
mod user_factory;

pub use factory::Factory;
pub use user_factory::UserFactory;

use sea_orm::DatabaseConnection;
use tracing::info;

/// Load default fixtures into database
/// Equivalent de: bin/console doctrine:fixtures:load
pub async fn load_fixtures(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    info!("Loading fixtures...");

    // Utilise le nouveau système de factory
    let users = UserFactory::new()
        .create_many(db, 3)
        .await?;

    for user in &users {
        info!("Created user: {} ({})", user.username, user.email);
    }

    // Créer un admin spécifique
    let admin = UserFactory::new()
        .with_username("admin")
        .with_email("admin@example.com")
        .create(db)
        .await?;

    info!("Created admin: {} ({})", admin.username, admin.email);

    info!("Fixtures loaded successfully!");
    Ok(())
}
