// src/middleware/logging.rs
// Request/response logging middleware
// Logs all HTTP requests with timing and status information

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use uuid::Uuid;

// ===== REQUEST LOGGING MIDDLEWARE =====
// Logs HTTP request and response details with timing
pub async fn logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    // Generate request ID for tracing
    let request_id = Uuid::new_v4();
    
    // Extract request information
    let method = request.method().clone();
    let uri = request.uri().clone();
    let version = request.version();
    
    // Try to get authenticated user (if any)
    let user_id = request
        .extensions()
        .get::<crate::middleware::auth::AuthUser>()
        .map(|u| u.id.to_string())
        .unwrap_or_else(|| "anonymous".to_string());
    
    // Get client IP from headers or connection
    let client_ip = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or("unknown").trim())
        .unwrap_or_else(|| {
            request
                .headers()
                .get("x-real-ip")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("unknown")
        })
        .to_string();
    
    // Start timing
    let start = Instant::now();
    
    // Log incoming request
    tracing::info!(
        request_id = %request_id,
        method = %method,
        uri = %uri,
        version = ?version,
        user_id = %user_id,
        client_ip = %client_ip,
        "Incoming request"
    );
    
    // Process request through remaining middleware/handlers
    let response = next.run(request).await;
    
    // Calculate request duration
    let duration = start.elapsed();
    
    // Get response status
    let status = response.status();
    
    // Determine log level based on status code
    match status {
        StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => {
            tracing::info!(
                request_id = %request_id,
                method = %method,
                uri = %uri,
                status = %status.as_u16(),
                duration_ms = %duration.as_millis(),
                user_id = %user_id,
                "Request completed successfully"
            );
        },
        StatusCode::BAD_REQUEST | StatusCode::NOT_FOUND => {
            tracing::warn!(
                request_id = %request_id,
                method = %method,
                uri = %uri,
                status = %status.as_u16(),
                duration_ms = %duration.as_millis(),
                user_id = %user_id,
                "Request completed with client error"
            );
        },
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
            tracing::warn!(
                request_id = %request_id,
                method = %method,
                uri = %uri,
                status = %status.as_u16(),
                duration_ms = %duration.as_millis(),
                user_id = %user_id,
                client_ip = %client_ip,
                "Authentication/authorization failure"
            );
        },
        _ if status.is_server_error() => {
            tracing::error!(
                request_id = %request_id,
                method = %method,
                uri = %uri,
                status = %status.as_u16(),
                duration_ms = %duration.as_millis(),
                user_id = %user_id,
                "Request failed with server error"
            );
        },
        _ => {
            tracing::info!(
                request_id = %request_id,
                method = %method,
                uri = %uri,
                status = %status.as_u16(),
                duration_ms = %duration.as_millis(),
                user_id = %user_id,
                "Request completed"
            );
        }
    }
    
    // Add request ID to response headers for client-side tracing
    let mut response = response;
    response.headers_mut().insert(
        "X-Request-ID",
        request_id.to_string().parse().unwrap(),
    );
    
    response
}

// ===== PERFORMANCE MONITORING =====
// Log slow requests for performance optimization
pub async fn performance_monitoring_middleware(
    request: Request,
    next: Next,
) -> Response {
    let uri = request.uri().clone();
    let method = request.method().clone();
    
    let start = Instant::now();
    let response = next.run(request).await;
    let duration = start.elapsed();
    
    // Warn if request takes longer than 1 second
    if duration.as_secs() >= 1 {
        tracing::warn!(
            method = %method,
            uri = %uri,
            duration_ms = %duration.as_millis(),
            "Slow request detected"
        );
    }
    
    // Alert if request takes longer than 5 seconds
    if duration.as_secs() >= 5 {
        tracing::error!(
            method = %method,
            uri = %uri,
            duration_ms = %duration.as_millis(),
            "Very slow request - performance issue detected"
        );
    }
    
    response
}

// ===== SECURITY HEADERS MIDDLEWARE =====
// Add security-related HTTP headers to all responses
pub async fn security_headers_middleware(
    request: Request,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;
    
    let headers = response.headers_mut();
    
    // Prevent MIME type sniffing
    headers.insert(
        "X-Content-Type-Options",
        "nosniff".parse().unwrap(),
    );
    
    // Enable XSS protection
    headers.insert(
        "X-XSS-Protection",
        "1; mode=block".parse().unwrap(),
    );
    
    // Prevent clickjacking
    headers.insert(
        "X-Frame-Options",
        "DENY".parse().unwrap(),
    );
    
    // Strict transport security (HTTPS only)
    headers.insert(
        "Strict-Transport-Security",
        "max-age=31536000; includeSubDomains".parse().unwrap(),
    );
    
    // Content Security Policy
    headers.insert(
        "Content-Security-Policy",
        "default-src 'self'".parse().unwrap(),
    );
    
    // Referrer policy
    headers.insert(
        "Referrer-Policy",
        "strict-origin-when-cross-origin".parse().unwrap(),
    );
    
    response
}