// src/services/mod.rs
// Equivalent de: src/Service/ en Symfony

mod post_service;
mod user_service;

pub use post_service::{PaginatedPosts, PostService, PostWithAuthor};
pub use user_service::{PaginatedUsers, UserService};
