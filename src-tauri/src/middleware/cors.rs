// src/middleware/cors.rs
// CORS (Cross-Origin Resource Sharing) configuration
// Allows frontend applications to access the API

use tower_http::cors::{Any, CorsLayer};
use axum::http::{HeaderValue, Method};

// ===== CONFIGURE CORS =====
// Create CORS layer with appropriate settings
pub fn configure_cors(allowed_origins: Vec<String>) -> CorsLayer {
    // Parse allowed origins
    let origins: Vec<HeaderValue> = allowed_origins
        .iter()
        .filter_map(|origin| origin.parse().ok())
        .collect();
    
    CorsLayer::new()
        // Allow specific origins (from environment variable)
        .allow_origin(origins)
        // Allow all HTTP methods
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        // Allow common headers
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
            axum::http::header::ACCEPT,
        ])
        // Allow credentials (cookies, auth headers)
        .allow_credentials(true)
        // Cache preflight requests for 1 hour
        .max_age(std::time::Duration::from_secs(3600))
}

// ===== DEVELOPMENT CORS =====
// Permissive CORS for development (allows all origins)
pub fn development_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any) // Allow all origins
        .allow_methods(Any) // Allow all methods
        .allow_headers(Any) // Allow all headers
        .allow_credentials(true)
}