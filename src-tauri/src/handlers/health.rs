// src/handlers/health.rs
// Health check and system status endpoints
// Used for monitoring and load balancer health checks

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

use crate::{
    config::DbPool,
    utils::{error::Result},
    AppState,
};

// ===== ROOT ENDPOINT =====
#[utoipa::path(
    get,
    path = "/",
    tag = "health",
    responses(
        (status = 200, description = "API is running", body = WelcomeResponse)
    )
)]
pub async fn root() -> impl IntoResponse {
    Json(WelcomeResponse {
        message: "ERP Backend API".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        status: "operational".to_string(),
    })
}

// ===== WELCOME RESPONSE =====
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WelcomeResponse {
    pub message: String,
    pub version: String,
    pub status: String,
}

// ===== HEALTH CHECK ENDPOINT =====
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "System is healthy", body = HealthResponse),
        (status = 503, description = "System is unhealthy", body = HealthResponse)
    )
)]
pub async fn health_check(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    tracing::debug!("Processing health check request");
    
    // Check database connectivity
    let database_status = check_database_connection(&state).await;
    
    // Determine overall status
    let is_healthy = database_status.is_healthy;
    let status_code = if is_healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    
    // Build health response
    let health_response = HealthResponse {
        status: if is_healthy { "healthy" } else { "unhealthy" }.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        environment: state.config.env.clone(),
        uptime_seconds: get_uptime_seconds(),
        database: database_status,
        timestamp: chrono::Utc::now(),
    };
    
    tracing::info!(
        status = %health_response.status,
        database_healthy = %health_response.database.is_healthy,
        "Health check completed"
    );
    
    Ok((status_code, Json(health_response)))
}

// ===== HEALTH RESPONSE =====
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub environment: String,
    pub uptime_seconds: u64,
    pub database: DatabaseHealth,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

// ===== DATABASE HEALTH =====
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DatabaseHealth {
    pub is_healthy: bool,
    pub database_type: String,
    pub message: String,
    pub response_time_ms: Option<u64>,
}

// ===== CHECK DATABASE CONNECTION =====
async fn check_database_connection(state: &AppState) -> DatabaseHealth {
    use std::time::Instant;
    
    let start = Instant::now();
    
    match &*state.db {
        DbPool::Postgres(pool) => {
            match sqlx::query("SELECT 1")
                .fetch_one(pool)
                .await
            {
                Ok(_) => {
                    let duration = start.elapsed();
                    DatabaseHealth {
                        is_healthy: true,
                        database_type: "PostgreSQL".to_string(),
                        message: "Connected successfully".to_string(),
                        response_time_ms: Some(duration.as_millis() as u64),
                    }
                }
                Err(e) => {
                    tracing::error!(error = %e, "PostgreSQL health check failed");
                    DatabaseHealth {
                        is_healthy: false,
                        database_type: "PostgreSQL".to_string(),
                        message: format!("Connection failed: {}", e),
                        response_time_ms: None,
                    }
                }
            }
        }
        DbPool::Sqlite(pool) => {
            match sqlx::query("SELECT 1")
                .fetch_one(pool)
                .await
            {
                Ok(_) => {
                    let duration = start.elapsed();
                    DatabaseHealth {
                        is_healthy: true,
                        database_type: "SQLite".to_string(),
                        message: "Connected successfully".to_string(),
                        response_time_ms: Some(duration.as_millis() as u64),
                    }
                }
                Err(e) => {
                    tracing::error!(error = %e, "SQLite health check failed");
                    DatabaseHealth {
                        is_healthy: false,
                        database_type: "SQLite".to_string(),
                        message: format!("Connection failed: {}", e),
                        response_time_ms: None,
                    }
                }
            }
        }
    }
}

// ===== GET UPTIME =====
fn get_uptime_seconds() -> u64 {
    use std::time::{SystemTime};
    
    static START_TIME: std::sync::OnceLock<SystemTime> = std::sync::OnceLock::new();
    let start = START_TIME.get_or_init(|| SystemTime::now());
    
    SystemTime::now()
        .duration_since(*start)
        .unwrap_or_default()
        .as_secs()
}

// ===== READINESS CHECK =====
#[utoipa::path(
    get,
    path = "/ready",
    tag = "health",
    responses(
        (status = 200, description = "Service is ready"),
        (status = 503, description = "Service is not ready")
    )
)]
pub async fn readiness_check(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let db_health = check_database_connection(&state).await;
    
    if db_health.is_healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}

// ===== LIVENESS CHECK =====
#[utoipa::path(
    get,
    path = "/live",
    tag = "health",
    responses(
        (status = 200, description = "Service is alive")
    )
)]
pub async fn liveness_check() -> impl IntoResponse {
    StatusCode::OK
}

pub fn health_router() -> axum::Router<Arc<AppState>> {
    use axum::routing::get;

    axum::Router::new()
        .route("/", get(health_check))
        .route("/ready", get(readiness_check))
        .route("/live", get(liveness_check))
}