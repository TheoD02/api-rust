// src/config/mod.rs
// Equivalent de: config/ en Symfony

mod app_state;
mod database;
mod logging;
mod openapi;

pub use app_state::AppState;
pub use database::init_database;
pub use logging::init_logging;
pub use openapi::ApiDoc;
