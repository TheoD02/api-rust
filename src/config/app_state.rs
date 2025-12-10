// src/config/app_state.rs
// Equivalent de: Container de services Symfony

use crate::services::UserService;

/// AppState - Application state containing all services
/// Equivalent de: Service Container en Symfony
///
/// En Symfony, les services sont injectes via le container.
/// En Rust/Axum, on utilise un AppState partage via Arc<AppState>.
#[derive(Clone)]
pub struct AppState {
    /// UserService instance
    /// Equivalent de: #[Autowire] UserService $userService
    pub user_service: UserService,
}

impl AppState {
    /// Create a new AppState with all services
    pub fn new(user_service: UserService) -> Self {
        Self { user_service }
    }
}
