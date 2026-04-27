// src/middleware/rate_limit.rs
// Rate limiting middleware
// Protects API from abuse by limiting requests per IP/user

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crate::utils::error::ErrorResponse;

// ===== RATE LIMITER STRUCTURE =====
// Tracks request counts per client
#[derive(Clone)]
pub struct RateLimiter {
    // Map of client identifier -> request tracker
    clients: Arc<Mutex<HashMap<String, ClientTracker>>>,
    
    // Maximum requests allowed per window
    max_requests: u32,
    
    // Time window duration
    window_duration: Duration,
}

// ===== CLIENT TRACKER =====
// Tracks requests for a single client
#[derive(Debug, Clone)]
struct ClientTracker {
    // Number of requests in current window
    request_count: u32,
    
    // Window start time
    window_start: Instant,
}

impl RateLimiter {
    // Create new rate limiter
    pub fn new(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_duration: Duration::from_secs(window_seconds),
        }
    }
    
    // Check if client can make request
    pub fn check_rate_limit(&self, client_id: &str) -> Result<(), RateLimitError> {
        let mut clients = self.clients.lock().unwrap();
        let now = Instant::now();
        
        // Get or create tracker for client
        let tracker = clients.entry(client_id.to_string()).or_insert(ClientTracker {
            request_count: 0,
            window_start: now,
        });
        
        // Check if window has expired
        if now.duration_since(tracker.window_start) >= self.window_duration {
            // Reset window
            tracker.window_start = now;
            tracker.request_count = 0;
        }
        
        // Check rate limit
        if tracker.request_count >= self.max_requests {
            let retry_after = self.window_duration
                .saturating_sub(now.duration_since(tracker.window_start))
                .as_secs();
            
            return Err(RateLimitError {
                retry_after,
                max_requests: self.max_requests,
                window_seconds: self.window_duration.as_secs(),
            });
        }
        
        // Increment request count
        tracker.request_count += 1;
        
        Ok(())
    }
    
    // Clean up old entries (run periodically)
    pub fn cleanup(&self) {
        let mut clients = self.clients.lock().unwrap();
        let now = Instant::now();
        
        // Remove clients whose windows have expired
        clients.retain(|_, tracker| {
            now.duration_since(tracker.window_start) < self.window_duration * 2
        });
        
        tracing::debug!(
            active_clients = clients.len(),
            "Rate limiter cleanup completed"
        );
    }
}

// ===== RATE LIMIT ERROR =====
#[derive(Debug)]
pub struct RateLimitError {
    pub retry_after: u64,
    pub max_requests: u32,
    pub window_seconds: u64,
}

impl IntoResponse for RateLimitError {
    fn into_response(self) -> Response {
        let error_response = ErrorResponse {
            status: 429,
            error_code: "RATE_LIMIT_EXCEEDED".to_string(),
            message: format!(
                "Rate limit exceeded. Maximum {} requests per {} seconds.",
                self.max_requests, self.window_seconds
            ),
            details: Some(serde_json::json!({
                "retry_after_seconds": self.retry_after,
                "max_requests": self.max_requests,
                "window_seconds": self.window_seconds,
            })),
            timestamp: chrono::Utc::now(),
        };
        
        let mut response = (StatusCode::TOO_MANY_REQUESTS, Json(error_response)).into_response();
        
        // Add Retry-After header
        response.headers_mut().insert(
            "Retry-After",
            self.retry_after.to_string().parse().unwrap(),
        );
        
        // Add rate limit headers
        response.headers_mut().insert(
            "X-RateLimit-Limit",
            self.max_requests.to_string().parse().unwrap(),
        );
        response.headers_mut().insert(
            "X-RateLimit-Remaining",
            "0".parse().unwrap(),
        );
        response.headers_mut().insert(
            "X-RateLimit-Reset",
            self.retry_after.to_string().parse().unwrap(),
        );
        
        response
    }
}

// ===== RATE LIMITING MIDDLEWARE =====
pub async fn rate_limit_middleware(
    limiter: Arc<RateLimiter>,
    request: Request,
    next: Next,
) -> Result<Response, RateLimitError> {
    // Extract client identifier (IP address)
    let client_id = request
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
        });
    
    // Check rate limit
    limiter.check_rate_limit(client_id)?;
    
    // Continue if within limits
    Ok(next.run(request).await)
}

// ===== AUTHENTICATED RATE LIMITING =====
// Stricter rate limiting for authenticated users
pub async fn authenticated_rate_limit_middleware(
    limiter: Arc<RateLimiter>,
    request: Request,
    next: Next,
) -> Result<Response, RateLimitError> {
    // Try to get user ID from auth context
    let client_id = if let Some(auth_user) = request.extensions().get::<crate::middleware::auth::AuthUser>() {
        auth_user.id.to_string()
    } else {
        // Fallback to IP if not authenticated
        request
            .headers()
            .get("x-forwarded-for")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown")
            .to_string()
    };
    
    // Check rate limit
    limiter.check_rate_limit(&client_id)?;
    
    Ok(next.run(request).await)
}