// src/controllers/health_controller.rs
// Equivalent de: src/Controller/HealthController.php

use axum::{routing::get, Router};
use tracing::info;

/// HealthController - Health check endpoints
pub struct HealthController;

impl HealthController {
    /// Register routes for this controller
    pub fn routes() -> Router {
        Router::new()
            .route("/", get(index))
            .route("/health", get(health))
    }
}

/// GET / - Index endpoint
#[utoipa::path(
    get,
    path = "/",
    tag = "health",
    responses(
        (status = 200, description = "API is running", body = String)
    )
)]
async fn index() -> &'static str {
    info!("Index endpoint called");
    "Hello from Rust API!"
}

/// GET /health - Health check
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Health check passed", body = String)
    )
)]
async fn health() -> &'static str {
    info!("Health check endpoint called");
    "OK"
}
