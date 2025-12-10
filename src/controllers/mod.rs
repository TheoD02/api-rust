// src/controllers/mod.rs
// Equivalent de: src/Controller/ en Symfony

pub mod health_controller;
pub mod post_controller;
pub mod user_controller;

pub use health_controller::HealthController;
pub use post_controller::PostController;
pub use user_controller::UserController;
