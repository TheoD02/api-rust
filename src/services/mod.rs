// src/services/mod.rs
// Equivalent de: src/Service/ en Symfony

mod user_service;

pub use user_service::{PaginatedUsers, UserService};
