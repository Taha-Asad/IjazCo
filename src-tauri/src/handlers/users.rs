// src/handlers/users.rs
// User management endpoints
// CRUD operations for users with permission checks and multi-tenant isolation

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState, config::DbPool, middleware::auth::{AuthUser, verify_company_access}, models::user::{CreateUserRequest, UpdateUserRequest, User, UserSafe}, utils::{
        error::{AppError, Result},
        response::{created, no_content, paginated, success},
    }
};

// ===== PAGINATION QUERY PARAMETERS =====
#[derive(Debug, Deserialize, ToSchema)]
pub struct PaginationParams {
    // Page number (1-based)
    #[serde(default = "default_page")]
    pub page: Option<String>,
    
    // Items per page
    #[serde(default = "default_per_page")]
    pub per_page: Option<String>,
}

fn default_page() -> Option<String> { Some("1".to_string()) }
fn default_per_page() -> Option<String> { Some("20".to_string()) }

impl PaginationParams {
    // Get page as i64
    pub fn page(&self) -> i64 {
        self.page.as_ref()
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(1)
    }
    
    // Get per_page as i64
    pub fn per_page(&self) -> i64 {
        self.per_page.as_ref()
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(20)
    }
    
    // Calculate SQL OFFSET
    pub fn offset(&self) -> i64 {
        (self.page() - 1) * self.per_page()
    }
    
    // Get SQL LIMIT
    pub fn limit(&self) -> i64 {
        self.per_page()
    }
}

// ===== USER SEARCH QUERY PARAMETERS =====
#[derive(Debug, Deserialize, ToSchema)]
pub struct UserSearchParams {
    // Search term (searches username, email, name)
    pub search: Option<String>,
    
    // Filter by status
    pub status: Option<String>,
    
    // Filter by role
    pub role_id: Option<Uuid>,
    
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

// ===== LIST USERS ENDPOINT =====
// Get paginated list of users for a company
#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = "users",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 20)"),
        ("search" = Option<String>, Query, description = "Search term"),
        ("status" = Option<String>, Query, description = "Filter by status"),
        ("role_id" = Option<Uuid>, Query, description = "Filter by role ID"),
    ),
    responses(
        (status = 200, description = "List of users", body = Vec<UserSafe>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn list_users(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Query(params): Query<UserSearchParams>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        company_id = %auth_user.company_id,
        "Listing users"
    );

    // 1. Fetch users and total count inside the match to handle different SQL dialects
    let (users, total_count) = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            let u = User::list_by_company_pg(
                pool,
                auth_user.company_id,
                params.pagination.limit(),
                params.pagination.offset(),
            ).await?;

            let count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM users WHERE company_id = $1 AND deleted_at IS NULL"
            )
            .bind(auth_user.company_id)
            .fetch_one(pool) // Use the pool from this match arm
            .await?;

            (u, count)
        }
        DbPool::Sqlite(pool) => {
            let u = User::list_by_company_sqlite(
                pool,
                auth_user.company_id,
                params.pagination.limit(),
                params.pagination.offset(),
            ).await?;

            // SQLite uses different placeholder ($1 vs ?) depending on driver config, 
            // but sqlx usually normalizes. Check if your sqlite query needs "?"
            let count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM users WHERE company_id = $1 AND deleted_at IS NULL"
            )
            .bind(auth_user.company_id)
            .fetch_one(pool)
            .await?;

            (u, count)
        }
    };

    // 2. Convert to safe user objects
    // Note: If you were getting the `i32` error here, ensure `u.into()` 
    // is not calling unwrap on a field that is already a primitive.
    let safe_users: Vec<UserSafe> = users
        .into_iter()
        .map(|u| u.into())
        .collect();

    tracing::debug!(
        count = safe_users.len(),
        total = total_count,
        "Users retrieved successfully"
    );

    // 3. Return paginated response
    // Ensure params.pagination fields are i32/i64 as expected by your helper
    Ok(paginated(
        safe_users,
        params.pagination.page(),
        params.pagination.per_page(),
        total_count,
    ))
}
// ===== GET USER ENDPOINT =====
// Get a single user by ID
#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    tag = "users",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = UserSafe),
        (status = 404, description = "User not found"),
        (status = 403, description = "Access denied")
    )
)]
pub async fn get_user(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<UserSafe>> {
    tracing::debug!(
        user_id = %auth_user.id,
        target_user_id = %id,
        "Fetching user"
    );
    
    // Get database pool

    let user_option = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            // SaaS / Production Cloud Branch
            User::find_by_id_pg(pool, id).await?
        }
        DbPool::Sqlite(pool) => {
            // Desktop / Offline-First Branch [cite: 7, 37]
            User::find_by_id_sqlite(pool, id).await?
        }
    };

    let user = user_option.ok_or_else(|| {
        AppError::NotFound(format!("User with ID {} not found", id))
    })?;
    // Verify user belongs to same company (multi-tenant isolation)
    verify_company_access(&auth_user, user.company_id)?;
    
    tracing::info!(
        target_user_id = %id,
        username = %user.username,
        "User retrieved successfully"
    );
    
    Ok(Json(user.into()))
}

// ===== CREATE USER ENDPOINT =====
// Create a new user
#[utoipa::path(
    post,
    path = "/api/v1/users",
    tag = "users",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserSafe),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Username or email already exists"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(mut payload): Json<CreateUserRequest>,
) -> Result<impl axum::response::IntoResponse> {
    // 1. Validate input
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        new_username = %payload.username,
        "Creating new user"
    );
    
    // 2. Force multi-tenant isolation
    payload.company_id = auth_user.company_id;
    
    // 3. Handle Database Logic Agnostically
    let user = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            // Check for existing user in Postgres
            if User::find_by_username_pg(pool, &payload.username).await?.is_some() {
                return Err(AppError::DuplicateKey("Username already exists".into()));
            }
            if User::find_by_email_pg(pool, &payload.email).await?.is_some() {
                return Err(AppError::DuplicateKey("Email already exists".into()));
            }
            // Create in Postgres
            User::create_pg(pool, payload, Some(auth_user.id)).await?
        }
        DbPool::Sqlite(pool) => {
            // Check for existing user in Sqlite
            if User::find_by_username_sqlite(pool, &payload.username).await?.is_some() {
                return Err(AppError::DuplicateKey("Username already exists".into()));
            }
            if User::find_by_email_sqlite(pool, &payload.email).await?.is_some() {
                return Err(AppError::DuplicateKey("Email already exists".into()));
            }
            // Create in Sqlite
            User::create_sqlite(pool, payload, Some(auth_user.id)).await?
        }
    };
    
    tracing::info!(
        new_user_id = %user.id,
        username = %user.username,
        "User created successfully"
    );
    
    // 4. Convert to safe user object
    let safe_user: UserSafe = user.into();
    
    Ok(created("User created successfully", safe_user))
}// ===== UPDATE USER ENDPOINT =====
// Update an existing user
#[utoipa::path(
    put,
    path = "/api/v1/users/{id}",
    tag = "users",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = UserSafe),
        (status = 404, description = "User not found"),
        (status = 400, description = "Validation error"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn update_user(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserSafe>> {
    // 1. Validate input
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        target_user_id = %id,
        "Updating user"
    );
    
    // 2. FIXED: Use agnostic database matching instead of hardcoded state.get_sqlite_pool()
    // This allows the update logic to work on both Postgres (Cloud) and Sqlite (Desktop)
    let updated_user = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            // Fetch existing user via Postgres
            let existing = User::find_by_id_pg(pool, id)
                .await?
                .ok_or_else(|| AppError::NotFound("User not found".into()))?;
            
            // 3. FIXED: Maintain multi-tenant isolation
            verify_company_access(&auth_user, existing.company_id)?;
            
            // 4. FIXED: Duplicate email check logic for Postgres
            if let Some(ref new_email) = payload.email {
                if new_email != &existing.email {
                    if let Some(other_user) = User::find_by_email_pg(pool, new_email).await? {
                        if other_user.id != id {
                            return Err(AppError::DuplicateKey("Email already in use".into()));
                        }
                    }
                }
            }
            
            // Perform update in Postgres
            User::update_pg(pool, id, payload, auth_user.id).await?
        }
        DbPool::Sqlite(pool) => {
            // Fetch existing user via Sqlite
            let existing = User::find_by_id_sqlite(pool, id)
                .await?
                .ok_or_else(|| AppError::NotFound("User not found".into()))?;
            
            // Verify access
            verify_company_access(&auth_user, existing.company_id)?;
            
            // Duplicate email check for Sqlite
            if let Some(ref new_email) = payload.email {
                if new_email != &existing.email {
                    if let Some(other_user) = User::find_by_email_sqlite(pool, new_email).await? {
                        if other_user.id != id {
                            return Err(AppError::DuplicateKey("Email already in use".into()));
                        }
                    }
                }
            }
            
            // Perform update in Sqlite
            User::update_sqlite(pool, id, payload, auth_user.id).await?
        }
    };
    
    tracing::info!(
        target_user_id = %id,
        "User updated successfully"
    );
    
    // 5. FIXED: Ensure the into() conversion for UserSafe is called on the result of the match
    Ok(Json(updated_user.into()))
}
// ===== DELETE USER ENDPOINT =====
// Soft delete a user
#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}",
    tag = "users",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = 404, description = "User not found"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        target_user_id = %id,
        "Deleting user"
    );
    
    // Prevent self-deletion
    if id == auth_user.id {
        return Err(AppError::OperationNotAllowed(
            "Cannot delete your own account".to_string()
        ));
    }
    
    let user_option = match state.db.as_ref() {
        DbPool::Postgres(pool) => User::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => User::find_by_id_sqlite(pool, id).await?,
    };

    let user = user_option.ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    // Verify user belongs to same company
    verify_company_access(&auth_user, user.company_id)?;
    
        match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            User::delete_pg(pool, id, auth_user.id).await?;
        }
        DbPool::Sqlite(pool) => {
            User::delete_sqlite(pool, id, auth_user.id).await?;
        }
    }
    // Soft delete user
    
    tracing::info!(
        target_user_id = %id,
        username = %user.username,
        "User deleted successfully"
    );
    
    Ok(no_content())
}

// ===== CHANGE USER PASSWORD ENDPOINT =====
// Admin endpoint to change another user's password
#[utoipa::path(
    post,
    path = "/api/v1/users/{id}/change-password",
    tag = "users",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = AdminChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully"),
        (status = 404, description = "User not found"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn admin_change_password(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<AdminChangePasswordRequest>,
) -> Result<impl axum::response::IntoResponse> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        admin_id = %auth_user.id,
        target_user_id = %id,
        "Admin changing user password"
    );
    
    // Get database pool
    let user_option = match state.db.as_ref() {
        DbPool::Postgres(pool) => User::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => User::find_by_id_sqlite(pool, id).await?,
    };

    
    let user = user_option.ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
    // Verify user belongs to same company
    verify_company_access(&auth_user, user.company_id)?;
    
    // Validate password strength
    crate::utils::password::validate_password_strength(&payload.new_password)?;
    
    // Update password
            match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            User::update_password_pg(pool, id, &payload.new_password, auth_user.id).await?;
        }
        DbPool::Sqlite(pool) => {
            User::update_password_sqlite(pool, id, &payload.new_password, auth_user.id).await?;
        }
    };
    tracing::info!(
        target_user_id = %id,
        "User password changed by admin"
    );
    
    Ok(success(
        "Password changed successfully",
        serde_json::json!({
            "message": "User should login with new password"
        }),
    ))
}

// ===== ADMIN CHANGE PASSWORD REQUEST =====
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AdminChangePasswordRequest {
    // New password
    #[validate(length(min = 8, max = 128))]
    pub new_password: String,
}

// ===== ACTIVATE/DEACTIVATE USER ENDPOINT =====
// Toggle user active status
#[utoipa::path(
    patch,
    path = "/api/v1/users/{id}/status",
    tag = "users",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = UpdateUserStatusRequest,
    responses(
        (status = 200, description = "User status updated", body = UserSafe),
        (status = 404, description = "User not found"),
        (status = 403, description = "Forbidden")
    )
)]

pub async fn update_user_status(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserStatusRequest>,
) -> Result<Json<UserSafe>> {
    tracing::info!(
        user_id = %auth_user.id,
        target_user_id = %id,
        new_status = ?payload.status,
        "Updating user status"
    );
    
    if id == auth_user.id {
        return Err(AppError::OperationNotAllowed(
            "Cannot change your own status".to_string()
        ));
    }
    
    let user_option = match state.db.as_ref() {
        DbPool::Postgres(pool) => User::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => User::find_by_id_sqlite(pool, id).await?,
    };

    let user = user_option.ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
    verify_company_access(&auth_user, user.company_id)?;
    
    // NOTE: This requires #[derive(Default)] on UpdateUserRequest struct
    let update_request = UpdateUserRequest {
        status: Some(payload.status),
        ..Default::default() 
    };
    
    // FIXED: Removed semicolons at the end of the await? lines
    // This ensures the match expression evaluates to a User, not ()
    let updated_user = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            User::update_pg(pool, id, update_request, auth_user.id).await?
        }
        DbPool::Sqlite(pool) => {
            User::update_sqlite(pool, id, update_request, auth_user.id).await?
        }
    };
    
    tracing::info!(
        target_user_id = %id,
        "User status updated successfully"
    );
    
    Ok(Json(updated_user.into()))
}
  
// ===== UPDATE USER STATUS REQUEST =====
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUserStatusRequest {
    pub status: crate::models::user::UserStatus,
}

pub fn users_router() -> axum::Router<Arc<AppState>> {
    use axum::routing::{get, patch, post};

    axum::Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
        .route("/:id/change-password", post(admin_change_password))
        .route("/:id/status", patch(update_user_status))
}