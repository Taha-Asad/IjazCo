// src/handlers/roles.rs
// Role management endpoints
// CRUD operations for roles and permissions

use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    config::DbPool,
    middleware::auth::{ AuthUser, verify_company_access },
    models::user::{ CreateRoleRequest, Role, UpdateRoleRequest },
    utils::{ error::{ AppError, Result }, response::{ created, no_content } },
};


// ===== LIST ROLES ENDPOINT =====
// Get all roles for a company
#[utoipa::path(
    get,
    path = "/api/v1/roles",
    tag = "roles",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "List of roles", body = Vec<Role>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_roles(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser
) -> Result<Json<Vec<Role>>> {
    tracing::info!(
        user_id = %auth_user.id,
        company_id = %auth_user.company_id,
        "Listing roles"
    );
    
    // Get database pool    

    // Fetch roles for company
    // let roles = Role::list_by_company(pool, auth_user.company_id).await?;
    let roles = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            // SaaS / Production Cloud Branch
            Role::list_by_company_pg(&pool, auth_user.company_id).await?
        }
        DbPool::Sqlite(pool) => {
            // Desktop / Offline-First Branch [cite: 7, 37]
            Role::list_by_company_sqlite(&pool, auth_user.company_id).await?
        }
    };
    tracing::debug!(
        count = roles.len(),
        "Roles retrieved successfully"
    );
    
    Ok(Json(roles))
}

// ===== GET ROLE ENDPOINT =====
// Get a single role by ID
#[utoipa::path(
    get,
    path = "/api/v1/roles/{id}",
    tag = "roles",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Role ID")
    ),
    responses(
        (status = 200, description = "Role found", body = Role),
        (status = 404, description = "Role not found"),
        (status = 403, description = "Access denied")
    )
)]
pub async fn get_role(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Role>> {
    tracing::debug!(
        user_id = %auth_user.id,
        role_id = %id,
        "Fetching role"
    );
    
    // // Get database pool
    // let pool = state.get_pool()?;
    
    // // Fetch role
    // let role = Role::find_by_id(pool, id)
    //     .await?
    //     .ok_or_else(|| AppError::NotFound("Role not found".to_string()))?;
    

    let role_option = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            // SaaS / Production Cloud Branch
            Role::find_by_id_pg(&pool, id).await?
        }
        DbPool::Sqlite(pool) => {
            // Desktop / Offline-First Branch [cite: 7, 37]
            Role::find_by_id_sqlite(&pool, id).await?
        }
    };

    let role = role_option.ok_or_else(|| {
        AppError::NotFound(format!("Role with ID {} not found", id))
    })?;
    // Verify role belongs to same company
    verify_company_access(&auth_user, role.company_id)?;
    
    tracing::info!(
        role_id = %id,
        role_name = %role.name,
        "Role retrieved successfully"
    );
    
    Ok(Json(role))
}

// ===== CREATE ROLE ENDPOINT =====
// Create a new role
#[utoipa::path(
    post,
    path = "/api/v1/roles",
    tag = "roles",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateRoleRequest,
    responses(
        (status = 201, description = "Role created successfully", body = Role),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Role name already exists"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn create_role(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(mut payload): Json<CreateRoleRequest>,
) -> Result<impl axum::response::IntoResponse> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        role_name = %payload.name,
        "Creating new role"
    );
    
    // Force company_id to match authenticated user's company
    payload.company_id = auth_user.company_id;
    
    // Get database pool
    let role = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            // SaaS / Production Cloud Branch
            Role::create_pg(&pool, payload , auth_user.id).await?
        }
        DbPool::Sqlite(pool) => {
            // Desktop / Offline-First Branch [cite: 7, 37] 
            Role::create_sqlite(&pool, payload, auth_user.id).await?
        }
    };    
    // Create role
    
    tracing::info!(
        role_id = %role.id,
        role_name = %role.name,
        "Role created successfully"
    );
    
    Ok(created("Role created successfully", role))
}

// ===== UPDATE ROLE ENDPOINT =====
// Update an existing role
#[utoipa::path(
    put,
    path = "/api/v1/roles/{id}",
    tag = "roles",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Role ID")
    ),
    request_body = UpdateRoleRequest,
    responses(
        (status = 200, description = "Role updated successfully", body = Role),
        (status = 404, description = "Role not found"),
        (status = 400, description = "Validation error"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn update_role(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<Json<Role>> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        role_id = %id,
        "Updating role"
    );
    
    // Get database pool
    let existing_role = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            // SaaS / Production Cloud Branch
            Role::find_by_id_pg(&pool, id).await?
        }
        DbPool::Sqlite(pool) => {
            // Desktop / Offline-First Branch [cite: 7, 37]
            Role::find_by_id_sqlite(&pool, id).await?
        }
    };    
    // Fetch existing role
        let role = existing_role.ok_or_else(|| {
        AppError::NotFound(format!("Role with ID {} not found", id))
    })?;
    // Verify role belongs to same company
    verify_company_access(&auth_user, role.company_id)?;
    
    // Prevent modifying system roles
    if role.is_system {
        return Err(AppError::OperationNotAllowed(
            "Cannot modify system-defined roles".to_string()
        ));
    }

        let updated_role = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            // SaaS / Production Cloud Branch
            Role::update_pg(&pool, id, payload, auth_user.id).await?
        }
        DbPool::Sqlite(pool) => {
            // Desktop / Offline-First Branch [cite: 7, 37]
            Role::update_sqlite(&pool, id, payload, auth_user.id).await?
        }
    };    
    
    // Update role
    
    tracing::info!(
        role_id = %id,
        "Role updated successfully"
    );
    
    Ok(Json(updated_role))
}

// ===== DELETE ROLE ENDPOINT =====
// Delete a role
#[utoipa::path(
    delete,
    path = "/api/v1/roles/{id}",
    tag = "roles",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Role ID")
    ),
    responses(
        (status = 204, description = "Role deleted successfully"),
        (status = 404, description = "Role not found"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "Role is in use")
    )
)]
pub async fn delete_role(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        role_id = %id,
        "Deleting role"
    );
    
    // Get database pool
// 1. Fetch role to verify existence and company ownership
    let role_option = match state.db.as_ref() {
        DbPool::Postgres(pool) => Role::find_by_id_pg(&pool, id).await?,
        DbPool::Sqlite(pool) => Role::find_by_id_sqlite(&pool, id).await?,
    };

    let role = role_option.ok_or_else(|| AppError::NotFound("Role not found".to_string()))?;

    // 2. Multi-tenant Check (SaaS Requirement)
    verify_company_access(&auth_user, role.company_id)?;

    // 3. System Role Check (RBAC Protection)
    if role.is_system {
        return Err(AppError::BadRequest("Cannot delete system roles".to_string()));
    }

    // 4. Divided Delete Execution
    match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            Role::delete_pg(&pool, id).await?;
        }
        DbPool::Sqlite(pool) => {
            Role::delete_sqlite(&pool, id).await?;
        }
    }

   
    tracing::info!(
        role_id = %id,
        role_name = %role.name,
        "Role deleted successfully"
    );
    
    Ok(no_content())
}

// ===== GET ROLE PERMISSIONS ENDPOINT =====
// Get detailed permissions for a role
#[utoipa::path(
    get,
    path = "/api/v1/roles/{id}/permissions",
    tag = "roles",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Role ID")
    ),
    responses(
        (status = 200, description = "Role permissions", body = PermissionsResponse),
        (status = 404, description = "Role not found"),
        (status = 403, description = "Access denied")
    )
)]
pub async fn get_role_permissions(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<PermissionsResponse>> {
    tracing::debug!(
        user_id = %auth_user.id,
        role_id = %id,
        "Fetching role permissions"
    );
    
    let role_option = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            // SaaS / Production Cloud Branch
            Role::find_by_id_pg(&pool, id).await?
        }
        DbPool::Sqlite(pool) => {
            // Desktop / Offline-First Branch [cite: 7, 37]
            Role::find_by_id_sqlite(&pool, id).await?
        }
    };
                let role = role_option.ok_or_else(|| {
        AppError::NotFound(format!("Role with ID {} not found", id))
    })?;
    // Verify company access
    verify_company_access(&auth_user, role.company_id)?;
    
    Ok(Json(PermissionsResponse {
        role_id: role.id,
        role_name: role.name,
        permissions: role.permissions,
    }))
}

// ===== PERMISSIONS RESPONSE =====
#[derive(Debug, Serialize, ToSchema)]
pub struct PermissionsResponse {
    pub role_id: Uuid,
    pub role_name: String,
    pub permissions: serde_json::Value,
}

// ===== UPDATE ROLE PERMISSIONS ENDPOINT =====
// Update permissions for a role
#[utoipa::path(
    put,
    path = "/api/v1/roles/{id}/permissions",
    tag = "roles",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Role ID")
    ),
    request_body = UpdatePermissionsRequest,
    responses(
        (status = 200, description = "Permissions updated", body = Role),
        (status = 404, description = "Role not found"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn update_role_permissions(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePermissionsRequest>,
) -> Result<Json<Role>> { // Added AppError to Result
    tracing::info!(
        user_id = %auth_user.id,
        role_id = %id,
        "Updating role permissions"
    );
    
    // 1. Fetch existing role using dual-db branching
    let role_option = match state.db.as_ref() {
        DbPool::Postgres(pool) => Role::find_by_id_pg(&pool, id).await?,
        DbPool::Sqlite(pool) => Role::find_by_id_sqlite(&pool, id).await?,
    };

    let role = role_option.ok_or_else(|| {
        AppError::NotFound(format!("Role with ID {} not found", id))
    })?;

    // 2. Multi-tenant Security Check (SRS Requirement)
    verify_company_access(&auth_user, role.company_id)?;
    
    // 3. RBAC Protection: Prevent modifying system roles
    if role.is_system {
        return Err(AppError::OperationNotAllowed(
            "Cannot modify permissions of system-defined roles".to_string()
        ));
    }
    
    // 4. Create Update Request using Default
    let update_request = UpdateRoleRequest {
        permissions: Some(payload.permissions),
        ..Default::default() // This now works
    };
    
    // 5. Execute Update using divided functions
    let updated_role = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            Role::update_pg(&pool, id, update_request, auth_user.id).await?
        }
        DbPool::Sqlite(pool) => {
            Role::update_sqlite(&pool, id, update_request, auth_user.id).await?
        }
    };    

    tracing::info!(role_id = %id, "Role permissions updated successfully");
    Ok(Json(updated_role))
}

// ===== UPDATE PERMISSIONS REQUEST =====
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePermissionsRequest {
    pub permissions: serde_json::Value,
}