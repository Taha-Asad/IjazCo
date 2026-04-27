// src/middleware/mod.rs
// Middleware module - exports all middleware functions

pub mod auth;          // JWT authentication middleware
pub mod rbac;          // Role-Based Access Control middleware
pub mod logging;       // Request/response logging middleware
pub mod rate_limit;    // Rate limiting middleware
pub mod cors;          // CORS configuration middleware

// Re-export commonly used middleware
pub use auth::{auth_middleware, optional_auth_middleware};
pub use rbac::{require_permission, require_role};
pub use logging::logging_middleware;