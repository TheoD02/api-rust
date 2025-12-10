// tests/common/mod.rs
// Equivalent de: tests/WebTestCase.php ou KernelTestCase

use axum::Router;
use axum_test::TestServer;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use std::sync::Arc;

use rust_api::config::AppState;
use rust_api::controllers::{HealthController, UserController};
use rust_api::services::UserService;
use migration::Migrator;

/// Create a test server with in-memory SQLite database
/// Equivalent de: static::createClient() en Symfony
pub async fn create_test_server() -> TestServer {
    let app = create_test_app().await;
    TestServer::new(app).unwrap()
}

/// Create the test application router
async fn create_test_app() -> Router {
    let db = create_test_database().await;
    let user_service = UserService::new(db);
    let state = Arc::new(AppState::new(user_service));

    let api_routes = UserController::routes();
    let health_routes = HealthController::routes();

    Router::new()
        .merge(api_routes)
        .with_state(state)
        .merge(health_routes)
}

/// Create an in-memory SQLite database for testing
/// Equivalent de: DAMA doctrine test bundle / sqlite in-memory
async fn create_test_database() -> DatabaseConnection {
    // Use in-memory SQLite for isolated tests
    let db = Database::connect("sqlite::memory:")
        .await
        .expect("Failed to connect to test database");

    // Run migrations
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");

    db
}
