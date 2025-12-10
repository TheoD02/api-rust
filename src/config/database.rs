// src/config/database.rs
// Equivalent de: config/packages/doctrine.yaml

use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tracing::info;

use migration::Migrator;

/// Initialize database connection and run migrations
/// Equivalent de: doctrine:database:create + doctrine:migrations:migrate
pub async fn init_database() -> DatabaseConnection {
    // Connection URL (comme DATABASE_URL dans .env)
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./database.sqlite?mode=rwc".to_string());

    info!("Connecting to database...");
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    info!("Database connected!");

    // Run migrations (comme bin/console doctrine:migrations:migrate)
    info!("Running migrations...");
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");
    info!("Migrations completed!");

    db
}
