// src/middleware/rbac.rs
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use crate::{
    AppState, config::DbPool, middleware::auth::get_auth_user, models::user::{Role, UserRole}, utils::error::{AppError, Result}
};

// ✅ FIXED: Admin middleware using AppState
pub async fn require_admin(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Result<Response> {
    let auth_user = get_auth_user(&request)?;

let role_opt = if let Some(pg) = &state.pg() {
    Role::find_by_id_pg(pg, auth_user.role_id).await?
} else if let Some(sqlite) = &state.sqlite() {
    Role::find_by_id_sqlite(sqlite, auth_user.role_id).await?
} else {
    return Err(AppError::InternalError("No DB".into()));
};

let role = role_opt
    .ok_or_else(|| AppError::NotFound("Role not found".into()))?;

    if role.role_type == UserRole::Admin {
        Ok(next.run(request).await)
    } else {
        Err(AppError::Forbidden("Admin only".to_string()))
    }
}

// TODO: Refactor require_role to use AppState instead of DbPool
pub fn require_role(
    _required_role: UserRole,
) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response>> + Send>> + Clone {
    move |request: Request, next: Next| {
        Box::pin(async move {
            // TODO: Update this to use AppState from extensions
            Err(AppError::InternalError("require_role not implemented yet - needs refactoring to use AppState".to_string()))
        })
    }
}

// TODO: Refactor require_permission to use AppState instead of DbPool
pub fn require_permission(
    _resource: &'static str,
    _action: &'static str,
) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response>> + Send>> + Clone {
    move |_request: Request, _next: Next| {
        Box::pin(async move {
            // TODO: Update this to use AppState from extensions
            Err(AppError::InternalError("require_permission not implemented yet - needs refactoring to use AppState".to_string()))
        })
    }
}

// Helper function for ownership checks (doesn't need DB)
pub fn check_ownership(auth_user_id: uuid::Uuid, resource_user_id: uuid::Uuid) -> Result<()> {
    if auth_user_id != resource_user_id {
        return Err(AppError::Forbidden("You can only modify your own resources".to_string()));
    }
    Ok(())
}