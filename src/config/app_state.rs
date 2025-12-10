// src/config/app_state.rs
// Equivalent de: Container de services Symfony

use crate::services::{PostService, UserService};

/// AppState - Application state containing all services
/// Equivalent de: Service Container en Symfony
///
/// En Symfony, les services sont injectes via le container.
/// En Rust/Axum, on utilise un AppState partage via Arc<AppState>.
#[derive(Clone)]
pub struct AppState {
    /// UserService instance
    pub user_service: UserService,
    /// PostService instance
    pub post_service: PostService,
}

impl AppState {
    /// Create a new AppState with all services
    pub fn new(user_service: UserService, post_service: PostService) -> Self {
        Self {
            user_service,
            post_service,
        }
    }
}
