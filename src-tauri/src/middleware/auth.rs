// src/middleware/auth.rs
// JWT authentication middleware
// Validates JWT tokens and attaches user claims to request extensions

use axum::{
    async_trait,
    extract::FromRequestParts,
    extract::Request,
    http::request::Parts,
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::user::{User, UserStatus},
    utils::{
        error::{AppError, Result},
        jwt::{extract_token_from_header, validate_jwt},
    },
};

// Import AppState from config
use crate::config::AppState;

// ===== AUTHENTICATED USER CONTEXT =====
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub company_id: Uuid,
    pub role_id: Uuid,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub user: User,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        parts
            .extensions
            .get::<AuthUser>()
            .cloned()
            .ok_or(AppError::MissingToken)
    }
}

// Helper function to get the JWT secret from state
fn get_jwt_secret(state: &AppState) -> &str {
    &state.config.jwt_secret
}

// Helper function to access the database pool
fn get_db_pool(state: &AppState) -> &DbPool {
    // Option 1: If db is directly on AppState
    // &state.db
    
    // Option 2: If db is wrapped in Arc
    // &state.db
    
    &state.db
}

// ===== JWT AUTHENTICATION MIDDLEWARE =====
pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response> {
    let uri = request.uri().path().to_string();
    
    // Skip authentication for public auth routes
    let public_routes = ["/login", "/register", "/refresh", "/logout", "/verify-email", "/request-password-reset", "/reset-password"];
    if public_routes.iter().any(|r| uri.ends_with(r)) {
        return Ok(next.run(request).await);
    }
    
    let state = request.extensions().get::<Arc<AppState>>().cloned()
        .ok_or(AppError::InternalError("Missing app state".to_string()))?;
    
    let headers = request.headers().clone();
    
    // Extract Authorization header
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::MissingToken)?;
    
    tracing::debug!("Processing authentication for request");
    
    // Extract token from "Bearer <token>" format
    let token = extract_token_from_header(auth_header)?;
    
    // Validate JWT token
    let jwt_secret = get_jwt_secret(&state);
    let claims = validate_jwt(token, jwt_secret)?;
    
    // Parse UUIDs from claims
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::InvalidToken)?;
    let company_id = Uuid::parse_str(&claims.company_id)
        .map_err(|_| AppError::InvalidToken)?;
    let role_id = Uuid::parse_str(&claims.role_id)
        .map_err(|_| AppError::InvalidToken)?;
    
    // Load user from database to verify account status
    let db = get_db_pool(&state);
    let user = match db {
        DbPool::Postgres(pool) => User::find_by_id_pg(pool, user_id).await?,
        DbPool::Sqlite(pool) => User::find_by_id_sqlite(pool, user_id).await?,
    };
    
    // Verify user exists
    let user = user.ok_or_else(|| {
        tracing::warn!(user_id = %user_id, "User not found during authentication");
        AppError::InvalidToken
    })?;
    
    // Check if account is locked
    if user.is_locked() {
        tracing::warn!(user_id = %user_id, "Attempted login to locked account");
        return Err(AppError::AccountLocked);
    }
    
    // Check if account is active
    if user.status != UserStatus::Active {
        tracing::warn!(
            user_id = %user_id,
            status = ?user.status,
            "Attempted login to inactive account"
        );
        return Err(AppError::AccountInactive);
    }
    
    // Create AuthUser context
    let auth_user = AuthUser {
        id: user_id,
        company_id,
        role_id,
        email: claims.email.clone(),
        username: claims.username.clone(),
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        user: user.clone(),
    };
    
    tracing::info!(
        user_id = %user_id,
        username = %auth_user.username,
        company_id = %company_id,
        "User authenticated successfully"
    );
    
    // Add AuthUser to request extensions
    request.extensions_mut().insert(auth_user);
    
    // Continue to next middleware/handler
    Ok(next.run(request).await)
}

// ===== OPTIONAL AUTHENTICATION MIDDLEWARE =====
pub async fn optional_auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response> {
    let state = request.extensions().get::<Arc<AppState>>().cloned();
    
    if let Some(state) = state {
        let headers = request.headers().clone();
        
        if let Some(auth_header) = headers.get("Authorization").and_then(|h| h.to_str().ok()) {
            if let Ok(token) = extract_token_from_header(auth_header) {
                let jwt_secret = get_jwt_secret(&state);
                if let Ok(claims) = validate_jwt(token, jwt_secret) {
                    if let (Ok(user_id), Ok(company_id), Ok(role_id)) = (
                        Uuid::parse_str(&claims.sub),
                        Uuid::parse_str(&claims.company_id),
                        Uuid::parse_str(&claims.role_id),
                    ) {
                        let db = get_db_pool(&state);
                        let user_result = match db {
                            DbPool::Postgres(pool) => User::find_by_id_pg(pool, user_id).await,
                            DbPool::Sqlite(pool) => User::find_by_id_sqlite(pool, user_id).await,
                        };
                        
                        if let Ok(Some(user)) = user_result {
                            if user.status == UserStatus::Active && !user.is_locked() {
                                let auth_user = AuthUser {
                                    id: user_id,
                                    company_id,
                                    role_id,
                                    email: claims.email,
                                    username: claims.username,
                                    first_name: user.first_name.clone(),
                                    last_name: user.last_name.clone(),
                                    user,
                                };
                                
                                request.extensions_mut().insert(auth_user);
                                
                                tracing::debug!(
                                    user_id = %user_id,
                                    "Optional authentication successful"
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(next.run(request).await)
}

// ===== EXTRACT AUTH USER FROM REQUEST =====
pub fn get_auth_user(request: &Request) -> Result<&AuthUser> {
    request
        .extensions()
        .get::<AuthUser>()
        .ok_or(AppError::MissingToken)
}

// ===== VERIFY COMPANY ACCESS =====
pub fn verify_company_access(auth_user: &AuthUser, resource_company_id: Uuid) -> Result<()> {
    if auth_user.company_id != resource_company_id {
        tracing::warn!(
            user_company_id = %auth_user.company_id,
            resource_company_id = %resource_company_id,
            "Cross-company access attempt blocked"
        );
        return Err(AppError::Forbidden(
            "Access denied: resource belongs to different company".to_string()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_auth_user_creation() {
        let user_id = Uuid::new_v4();
        let company_id = Uuid::new_v4();
        let role_id = Uuid::new_v4();
        
        let user = User {
            id: user_id,
            company_id,
            role_id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hash".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            phone: None,
            avatar_url: None,
            status: UserStatus::Active,
            is_email_verified: true,
            email_verified_at: None,
            last_login_at: None,
            last_login_ip: None,
            failed_login_attempts: 0,
            locked_until: None,
            password_reset_token: None,
            password_reset_expires_at: None,
            two_factor_enabled: false,
            two_factor_secret: None,
            preferences: serde_json::json!({}),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            created_by: None,
            updated_by: None,
            deleted_at: None,
        };
        
        let auth_user = AuthUser {
            id: user_id,
            company_id,
            role_id,
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            user,
        };
        
        assert_eq!(auth_user.id, user_id);
        assert_eq!(auth_user.company_id, company_id);
    }
    
    #[test]
    fn test_verify_company_access() {
        let company_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let role_id = Uuid::new_v4();
        
        let user = User {
            id: user_id,
            company_id,
            role_id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hash".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            phone: None,
            avatar_url: None,
            status: UserStatus::Active,
            is_email_verified: true,
            email_verified_at: None,
            last_login_at: None,
            last_login_ip: None,
            failed_login_attempts: 0,
            locked_until: None,
            password_reset_token: None,
            password_reset_expires_at: None,
            two_factor_enabled: false,
            two_factor_secret: None,
            preferences: serde_json::json!({}),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            created_by: None,
            updated_by: None,
            deleted_at: None,
        };
        
        let auth_user = AuthUser {
            id: user_id,
            company_id,
            role_id,
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            user,
        };
        
        assert!(verify_company_access(&auth_user, company_id).is_ok());
        
        let other_company = Uuid::new_v4();
        assert!(verify_company_access(&auth_user, other_company).is_err());
    }
}