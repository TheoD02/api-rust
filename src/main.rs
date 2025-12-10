// src/main.rs
// Equivalent de: public/index.php + bin/console server:start

// === Module declarations ===
mod config;
mod controllers;
mod dto;
mod entities;
mod error;
mod response;
mod services;
mod validation;

// === Imports ===
use std::sync::Arc;

use axum::Router;
use tower_http::trace::TraceLayer;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use config::{init_database, init_logging, ApiDoc, AppState};
use controllers::{HealthController, PostController, UserController};
use services::{PostService, UserService};

/// Build the application router
fn build_router(state: Arc<AppState>) -> Router {
    // Routes with state
    let user_routes = UserController::routes();
    let post_routes = PostController::routes();

    // Health routes (no state needed)
    let health_routes = HealthController::routes();

    Router::new()
        // Merge routes that need state
        .merge(user_routes)
        .merge(post_routes)
        // Then apply state
        .with_state(state)
        // Then merge stateless routes
        .merge(health_routes)
        // Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // HTTP request logging middleware
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                    )
                })
                .on_response(
                    |response: &axum::http::Response<_>,
                     latency: std::time::Duration,
                     _span: &tracing::Span| {
                        tracing::info!(
                            status = %response.status().as_u16(),
                            latency_ms = %latency.as_millis(),
                            "Response sent"
                        );
                    },
                ),
        )
}

/// Application entry point
#[tokio::main]
async fn main() {
    // Initialize logging
    init_logging();

    info!("Starting Rust API...");

    // Initialize database
    let db = init_database().await;

    // Create services
    let user_service = UserService::new(db.clone());
    let post_service = PostService::new(db);

    // Create application state
    let state = Arc::new(AppState::new(user_service, post_service));

    // Build router with all routes
    let app = build_router(state);

    // Start server
    let addr = std::env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());

    info!(address = %addr, "Server starting...");
    info!("Swagger UI: http://localhost:8080/swagger-ui/");
    info!("OpenAPI JSON: http://localhost:8080/api-docs/openapi.json");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!(address = %addr, "Server running!");

    axum::serve(listener, app).await.unwrap();
}
