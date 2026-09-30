pub mod handlers;
pub mod jwt;
pub mod middleware;
pub mod models;
pub mod repository;
pub mod service;

// Re-export AuthUser for convenient access
pub use middleware::AuthUser;
