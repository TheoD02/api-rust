// src/dto/pagination.rs
// Pagination query parameters

use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

/// Pagination query parameters
/// Equivalent de: PaginationRequest en Symfony
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct PaginationQuery {
    /// Page number (1-based)
    #[serde(default = "default_page")]
    #[param(minimum = 1, default = 1)]
    pub page: u64,

    /// Items per page
    #[serde(default = "default_per_page")]
    #[param(minimum = 1, maximum = 100, default = 10)]
    pub per_page: u64,
}

fn default_page() -> u64 {
    1
}

fn default_per_page() -> u64 {
    10
}

impl Default for PaginationQuery {
    fn default() -> Self {
        Self {
            page: default_page(),
            per_page: default_per_page(),
        }
    }
}

impl PaginationQuery {
    /// Calculate offset for database query
    pub fn offset(&self) -> u64 {
        (self.page.saturating_sub(1)) * self.per_page
    }

    /// Get limit (per_page)
    pub fn limit(&self) -> u64 {
        self.per_page
    }
}
