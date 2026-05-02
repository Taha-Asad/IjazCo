// src/handlers/auth.rs
// Authentication and authorization endpoints
// Handles login, registration, token refresh, and logout

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use validator::Validate;
use tracing;
use uuid::Uuid;

use crate::{
    AppState, config::DbPool, middleware::auth::AuthUser, 
    models::user::{CreateUserRequest, User, UserStatus},
    utils::{
        error::{AppError, Result},
        jwt::{TokenType, generate_jwt, refresh_access_token},
        password::{validate_password_strength, verify_password},
        response::{created, success},
    }
};


// ===== LOGIN REQUEST =====
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    // Username or email
    #[validate(length(min = 3, max = 100))]
    #[schema(example = "admin")]
    pub username: String,
    
    // Password
    #[validate(length(min = 8, max = 128))]
    #[schema(example = "Password123!")]
    pub password: String,
    
    // Remember me flag (extends token expiration)
    #[serde(default)]
    pub remember_me: bool,
}

// ===== LOGIN RESPONSE =====
#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    // Access token for API requests (short-lived)
    pub access_token: String,
    
    // Refresh token for obtaining new access tokens (long-lived)
    pub refresh_token: String,
    
    // Token type (always "Bearer")
    pub token_type: String,
    
    // Access token expiration in seconds
    pub expires_in: i64,
    
    // User information
    pub user: UserInfo,
}

// ===== USER INFO =====
#[derive(Debug, Serialize, ToSchema)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role_id: Uuid,
    pub company_id: Uuid,
}

// ===== LOGIN ENDPOINT =====
// Authenticate user and return JWT tokens
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 403, description = "Account locked or inactive")
    )
)]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(username = %payload.username, "Login attempt");
    
    // 1. Fetch user using the dual-db pattern
    let user = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            let found = if let Some(u) = User::find_by_username_pg(&pool, &payload.username).await? {
                Some(u)
            } else {
                User::find_by_email_pg(&pool, &payload.username).await?
            };
            found
        }
        DbPool::Sqlite(pool) => {
            let found = if let Some(u) = User::find_by_username_sqlite(&pool, &payload.username).await? {
                Some(u)
            } else {
                User::find_by_email_sqlite(&pool, &payload.username).await?
            };
            found
        }
    }.ok_or_else(|| {
        tracing::warn!(username = %payload.username, "Login failed: user not found");
        AppError::InvalidCredentials
    })?;

    // 2. Check if account is locked or inactive
    if user.is_locked() {
        return Err(AppError::AccountLocked);
    }
    
    if user.status != UserStatus::Active {
        return Err(AppError::AccountInactive);
    }
    
    // 3. Verify password
    let is_valid = verify_password(&payload.password, &user.password_hash)?;
    
    if !is_valid {
        // We must match on the DB again to call the increment function
        match state.db.as_ref() {
            DbPool::Postgres(pool) => User::increment_failed_login_pg(&pool, user.id, 5, 30).await?,
            DbPool::Sqlite(pool) => User::increment_failed_login_sqlite(&pool, user.id, 5, 30).await?,
        };
        return Err(AppError::InvalidCredentials);
    }
    
    // 4. Generate JWT tokens
let access_token = generate_jwt(
        user.id, user.company_id, user.role_id, &user.email, &user.username,
        TokenType::Access, &state.config.jwt_secret, // Fix: Added .config
    )?;
    
let refresh_token = generate_jwt(
    user.id,
    user.company_id,
    user.role_id,
    &user.email,
    &user.username,
    TokenType::Refresh,
    &state.config.jwt_secret, // Fix: Added .config
)?;
    let ip_address = None;
    // 5. Update last login timestamp using correct engine
    match state.db.as_ref() {
        DbPool::Postgres(pool) => User::update_last_login_pg(&pool, user.id,ip_address.clone()).await?,
        DbPool::Sqlite(pool) => User::update_last_login_sqlite(&pool, user.id,ip_address.clone()).await?,
    };

    Ok(Json(LoginResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: 3600,
        user: UserInfo {
            id: user.id,
            username: user.username,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            role_id: user.role_id,
            company_id: user.company_id,
        },
    }))
}
// ===== REGISTER REQUEST =====
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    // Company name (for new company creation)
    #[validate(length(min = 2, max = 100))]
    #[schema(example = "My Company")]
    pub company_name: Option<String>,
    
    // Company ID (use existing company - optional, company_name takes priority)
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub company_id: Option<Uuid>,
    
    // Username (unique)
    #[validate(length(min = 3, max = 50))]
    #[schema(example = "newuser")]
    pub username: String,

    
    // Email (unique)
    #[validate(email)]
    #[schema(example = "user@example.com")]
    pub email: String,
    
    // Password
    #[validate(length(min = 8, max = 128))]
    #[schema(example = "SecurePass123!")]
    pub password: String,
    
    // Confirm password
    #[validate(length(min = 8, max = 128))]
    #[validate(must_match = "password")]
    pub password_confirmation: String,
    
    // First name
    #[validate(length(min = 1, max = 100))]
    #[schema(example = "John")]
    pub first_name: String,
    
    // Last name
    #[validate(length(min = 1, max = 100))]
    #[schema(example = "Doe")]
    pub last_name: String,
    
    // Phone (optional)
    #[schema(example = "+1-555-1234")]
    pub phone: Option<String>,

    pub role_id: Uuid, // Role ID for the new user (required)
}

// ===== CUSTOM VALIDATION FUNCTIONS =====


// ===== REGISTER ENDPOINT =====
// Create new user account
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = UserInfo),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Username or email already exists")
    )
)]
// src/handlers/auth.rs
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl axum::response::IntoResponse> {
    // 1. Validate input
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    // 2. Verify passwords match
    if payload.password != payload.password_confirmation {
        return Err(AppError::ValidationError("Passwords do not match".to_string()));
    }
    
    // 3. Validate password strength
    validate_password_strength(&payload.password)?;
    
    // 4. Check if user already exists
    let user_exists = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            User::find_by_username_pg(&pool, &payload.username).await?.is_some() ||
            User::find_by_email_pg(&pool, &payload.email).await?.is_some()
        }
        DbPool::Sqlite(pool) => {
            User::find_by_username_sqlite(&pool, &payload.username).await?.is_some() ||
            User::find_by_email_sqlite(&pool, &payload.email).await?.is_some()
        }
    };

    if user_exists {
        return Err(AppError::DuplicateKey("Username or email already exists".to_string()));
    }

    // 5. Create user logic
    // IMPORTANT: We use the payload IDs. If they are missing in JSON, 
    // we use the SEED ID ('...001'), NOT the NIL ID ('...000').
    let seed_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    
    let first_name = payload.first_name.clone();
    let last_name = payload.last_name.clone();
    let username = payload.username.clone();
    let email = payload.email.clone();
    let role_id = payload.role_id;
    let company_id = payload.company_id.unwrap_or(seed_id);
    

    let create_request = CreateUserRequest {
        username: username.clone(),
        email: email.clone(),
        password: payload.password,
        first_name: first_name.clone(),
        last_name: last_name.clone(),
        phone: payload.phone,
        role_id,
        company_id,
        status: Some("active".to_string()),
    };
    
    let system_user_id = Some(Uuid::nil());
    
    match state.db.as_ref() {
        DbPool::Postgres(pool) => User::create_pg(&pool, create_request, system_user_id).await?,
        DbPool::Sqlite(pool) => User::create_sqlite(&pool, create_request, system_user_id).await?,
    };

    // 6. Build response - use payload data since we know insert succeeded
    let user_info = UserInfo {
        id: Uuid::new_v4(),
        username,
        email,
        first_name,
        last_name,
        role_id,
        company_id,
    };
    
    Ok(created("User registered successfully", user_info))
}


#[derive(Debug, Deserialize, ToSchema)]


pub struct RefreshTokenRequest {
    // Refresh token
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub refresh_token: String,
}

// ===== REFRESH TOKEN RESPONSE =====
#[derive(Debug, Serialize, ToSchema)]
pub struct RefreshTokenResponse {
    // New access token
    pub access_token: String,
    
    // Token type
    pub token_type: String,
    
    // Expiration in seconds
    pub expires_in: i64,
}

// ===== REFRESH TOKEN ENDPOINT =====
// Exchange refresh token for new access token
#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    tag = "auth",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = RefreshTokenResponse),
        (status = 401, description = "Invalid or expired refresh token")
    )
)]
pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<Json<RefreshTokenResponse>> {
    tracing::debug!("Processing token refresh request");
    
    // Validate refresh token and generate new access token
    let (access_token, _claims) = refresh_access_token(
        &payload.refresh_token,
        &state.config.jwt_secret,
    )?;
    
    tracing::info!("Token refreshed successfully");
    
    Ok(Json(RefreshTokenResponse {
        access_token,
        token_type: "Bearer".to_string(),
        expires_in: 3600,
    }))
}

// ===== LOGOUT ENDPOINT =====
// Logout user (client should discard tokens)
#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    tag = "auth",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Logout successful")
    )
)]
pub async fn logout(
    auth_user: AuthUser,
) -> Result<impl axum::response::IntoResponse> {
    // In a stateless JWT system, logout is handled client-side
    // Client should delete stored tokens
    
    // In production, you might:
    // 1. Add token to blacklist (Redis)
    // 2. Log the logout event
    // 3. Trigger any cleanup actions
    
    tracing::info!(user_id = %auth_user.id, "User logged out");
    
    Ok(success(
        "Logged out successfully",
        serde_json::json!({
            "message": "Please discard your tokens"
        }),
    ))
}

// ===== CHANGE PASSWORD REQUEST =====
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ChangePasswordRequest {
    // Current password
    #[validate(length(min = 8))]
    pub current_password: String,
    
    // New password
    #[validate(length(min = 8, max = 128))]
    pub new_password: String,
    
    // Confirm new password
    #[validate(length(min = 8, max = 128))]
    #[validate(must_match = "new_password")]
    pub new_password_confirmation: String,
}

// ===== CHANGE PASSWORD ENDPOINT =====
// Change user's password
#[utoipa::path(
    post,
    path = "/api/v1/auth/change-password",
    tag = "auth",
    security(
        ("bearer_auth" = [])
    ),
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully"),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Current password is incorrect")
    )
)]
pub async fn change_password(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<impl axum::response::IntoResponse> {
    // 1. Validation
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    if payload.new_password != payload.new_password_confirmation {
        return Err(AppError::ValidationError("New passwords do not match".to_string()));
    }
    
    validate_password_strength(&payload.new_password)?;
    
    // 2. Fetch User (Fix: Unwrap the Option)
    let user = match state.db.as_ref() {
        DbPool::Postgres(pool) => User::find_by_id_pg(&pool, auth_user.id).await?,
        DbPool::Sqlite(pool) => User::find_by_id_sqlite(&pool, auth_user.id).await?,
    }.ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    // 3. Verify current password
    let is_valid = verify_password(&payload.current_password, &user.password_hash)?;
    
    if !is_valid {
        return Err(AppError::InvalidCredentials);
    }
    
    // 4. Update password (Fix: Match DB to call the correct divided function)
    match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            User::update_password_pg(&pool, auth_user.id, &payload.new_password, auth_user.id).await?
        }
        DbPool::Sqlite(pool) => {
            User::update_password_sqlite(&pool, auth_user.id, &payload.new_password, auth_user.id).await?
        }
    };
    
    tracing::info!(user_id = %auth_user.id, "Password changed successfully");
    
    Ok(axum::http::StatusCode::OK)
}
// ===== ME ENDPOINT =====
// Get current authenticated user information
#[utoipa::path(
    get,
    path = "/api/v1/auth/me",
    tag = "auth",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Current user information", body = UserInfo)
    )
)]
pub async fn me(
    auth_user: AuthUser,
) -> Result<impl axum::response::IntoResponse> {
    let user_info = UserInfo {
        id: auth_user.id,
        username: auth_user.username,
        email: auth_user.email,
        first_name: auth_user.first_name,
        last_name: auth_user.last_name,
        role_id: auth_user.role_id,
        company_id: auth_user.company_id,
    };
    Ok(success("Current user", user_info))
}

// ===== VERIFY EMAIL REQUEST =====
#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyEmailRequest {
    // Verification token (sent via email)
    pub token: String,
}

// ===== VERIFY EMAIL ENDPOINT =====
// Verify user's email address
#[utoipa::path(
    post,
    path = "/api/v1/auth/verify-email",
    tag = "auth",
    request_body = VerifyEmailRequest,
    responses(
        (status = 200, description = "Email verified successfully"),
        (status = 400, description = "Invalid or expired token")
    )
)]
pub async fn verify_email(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<VerifyEmailRequest>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(token = %payload.token, "Email verification attempted");
    
    let user = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            let row_json: Option<String> = sqlx::query_scalar(
                "SELECT row_to_json(u)::text FROM (SELECT * FROM users WHERE password_reset_token = $1 AND deleted_at IS NULL) u"
            )
            .bind(&payload.token)
            .fetch_optional(pool).await
            .map_err(|e| AppError::InternalError(e.to_string()))?;
            
            match row_json {
                Some(json_str) => {
                    let user: User = serde_json::from_str(&json_str)
                        .map_err(|e| AppError::InternalError(format!("JSON parse error: {}", e)))?;
                    Some(user)
                }
                None => None
            }
        }
        DbPool::Sqlite(pool) => {
            User::find_by_reset_token_sqlite(pool, &payload.token).await.ok().flatten()
        }
    };
    
    let user = user.ok_or(AppError::InvalidToken)?;
    
    match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            sqlx::query(
                "UPDATE users SET is_email_verified = true, email_verified_at = CURRENT_TIMESTAMP, password_reset_token = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = $1"
            )
            .bind(user.id)
            .execute(pool).await
            .map_err(|e| AppError::InternalError(e.to_string()))?;
        }
        DbPool::Sqlite(pool) => {
            User::verify_email_sqlite(pool, user.id).await
            .map_err(|e| AppError::InternalError(e.to_string()))?;
        }
    };
    
    Ok(success(
        "Email verified successfully",
        serde_json::json!({
            "message": "Your email has been verified",
            "user_id": user.id.to_string()
        }),
    ))
}

// ===== REQUEST PASSWORD RESET =====
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RequestPasswordResetRequest {
    // Email address
    #[validate(email)]
    pub email: String,
}

// ===== REQUEST PASSWORD RESET ENDPOINT =====
// Send password reset email
#[utoipa::path(
    post,
    path = "/api/v1/auth/request-password-reset",
    tag = "auth",
    request_body = RequestPasswordResetRequest,
    responses(
        (status = 200, description = "Password reset email sent")
    )
)]
pub async fn request_password_reset(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RequestPasswordResetRequest>,
) -> Result<impl axum::response::IntoResponse> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(email = %payload.email, "Password reset requested");
    
    let user = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            let row_json: Option<String> = sqlx::query_scalar(
                "SELECT row_to_json(u)::text FROM (SELECT * FROM users WHERE email = $1 AND deleted_at IS NULL) u"
            )
            .bind(&payload.email)
            .fetch_optional(pool).await
            .map_err(|e| AppError::InternalError(e.to_string()))?;
            
            match row_json {
                Some(json_str) => {
                    let user: User = serde_json::from_str(&json_str)
                        .map_err(|e| AppError::InternalError(format!("JSON parse error: {}", e)))?;
                    Some(user)
                }
                None => None
            }
        }
        DbPool::Sqlite(pool) => {
            User::find_by_email_sqlite(pool, &payload.email).await.ok().flatten()
        }
    };
    
    if let Some(user) = user {
        let reset_token = uuid::Uuid::new_v4().to_string();
        let expires_at = chrono::Utc::now() + chrono::Duration::hours(1);
        
        match state.db.as_ref() {
            DbPool::Postgres(pool) => {
                User::set_reset_token_pg(pool, user.id, &reset_token, expires_at).await
                .map_err(|e| AppError::InternalError(e.to_string()))?;
            }
            DbPool::Sqlite(pool) => {
                sqlx::query(
                    "UPDATE users SET password_reset_token = ?, password_reset_expires_at = ? WHERE id = ?"
                )
                .bind(&reset_token)
                .bind(expires_at.to_rfc3339())
                .bind(user.id)
                .execute(pool).await
                .map_err(|e| AppError::InternalError(e.to_string()))?;
            }
        }
        
        tracing::info!(user_id = %user.id, "Reset token generated (email sending skipped in dev)");
        
        return Ok(success(
            "Password reset instructions sent",
            serde_json::json!({
                "message": "If the email exists, reset instructions have been sent",
                "reset_token": reset_token
            }),
        ));
    }
    
    Ok(success(
        "Password reset instructions sent",
        serde_json::json!({
            "message": "If the email exists, reset instructions have been sent"
        }),
    ))
}

// ===== RESET PASSWORD REQUEST =====
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ResetPasswordRequest {
    // Reset token
    pub token: String,
    
    // New password
    #[validate(length(min = 8, max = 128))]
    pub new_password: String,
    
    // Confirm password
    #[validate(length(min = 8, max = 128))]
    #[validate(must_match = "new_password")]
    pub new_password_confirmation: String,
}

// ===== RESET PASSWORD ENDPOINT =====
// Reset password using token
#[utoipa::path(
    post,
    path = "/api/v1/auth/reset-password",
    tag = "auth",
    request_body = ResetPasswordRequest,
    responses(
        (status = 200, description = "Password reset successfully"),
        (status = 400, description = "Invalid or expired token")
    )
)]
pub async fn reset_password(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<impl axum::response::IntoResponse> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    if payload.new_password != payload.new_password_confirmation {
        return Err(AppError::ValidationError(
            "Passwords do not match".to_string()
        ));
    }
    
    validate_password_strength(&payload.new_password)?;
    
    tracing::info!(token = %payload.token, "Password reset attempted");
    
    let user = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            let row_json: Option<String> = sqlx::query_scalar(
                "SELECT row_to_json(u)::text FROM (SELECT * FROM users WHERE password_reset_token = $1 AND deleted_at IS NULL) u"
            )
            .bind(&payload.token)
            .fetch_optional(pool).await
            .map_err(|e| AppError::InternalError(e.to_string()))?;
            
            match row_json {
                Some(json_str) => {
                    let user: User = serde_json::from_str(&json_str)
                        .map_err(|e| AppError::InternalError(format!("JSON parse error: {}", e)))?;
                    Some(user)
                }
                None => None
            }
        }
        DbPool::Sqlite(pool) => {
            User::find_by_reset_token_sqlite(pool, &payload.token).await.ok().flatten()
        }
    };
    
    let user = user.ok_or(AppError::InvalidToken)?;
    
    if let Some(expires_at) = user.password_reset_expires_at {
        if expires_at < chrono::Utc::now() {
            return Err(AppError::InvalidToken);
        }
    } else {
        return Err(AppError::InvalidToken);
    }
    
    match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            User::update_password_pg(pool, user.id, &payload.new_password, user.id).await
            .map_err(|e| AppError::InternalError(e.to_string()))?;
            User::clear_reset_token_pg(pool, user.id).await
            .map_err(|e| AppError::InternalError(e.to_string()))?;
        }
        DbPool::Sqlite(pool) => {
            User::update_password_sqlite(pool, user.id, &payload.new_password, user.id).await
            .map_err(|e| AppError::InternalError(e.to_string()))?;
            sqlx::query(
                "UPDATE users SET password_reset_token = NULL, password_reset_expires_at = NULL WHERE id = ?"
            )
            .bind(user.id)
            .execute(pool).await
            .map_err(|e| AppError::InternalError(e.to_string()))?;
        }
    }
    
    tracing::info!(user_id = %user.id, "Password reset successful");
    
    Ok(success(
        "Password reset successfully",
        serde_json::json!({
            "message": "Your password has been reset. Please login."
        }),
    ))
}

// ===== AUTH ROUTER =====
// Create the authentication router with all auth endpoints
pub fn auth_router() -> axum::Router<Arc<AppState>> {
    use axum::routing::{get, post};

    axum::Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/refresh", post(refresh_token))
        .route("/logout", post(logout))
        .route("/me", get(me))
        .route("/change-password", post(change_password))
        .route("/verify-email", post(verify_email))
        .route("/request-password-reset", post(request_password_reset))
        .route("/reset-password", post(reset_password))
}