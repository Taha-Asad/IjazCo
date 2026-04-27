// src/handlers/suppliers.rs
// Supplier management endpoints
// CRUD operations for suppliers with statistics

use axum::{
    extract::{Path, Query, State},
    Json,
    routing::{get, post, put, delete},
};
use serde::Deserialize;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    config::{AppState, DbPool},
    middleware::auth::{AuthUser, verify_company_access},
    models::supplier::{
        CreateSupplierRequest, Supplier, SupplierWithStats, UpdateSupplierRequest,
    },
    utils::{
        error::{AppError, Result},
        response::{created, no_content, paginated},
    },
};

use super::users::PaginationParams;

// ===== SUPPLIER SEARCH PARAMETERS =====
#[derive(Debug, Deserialize, ToSchema)]
pub struct SupplierSearchParams {
    pub search: Option<String>,
    
    #[serde(default = "default_true")]
    pub active_only: bool,
    
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

fn default_true() -> bool { true }

// ===== LIST SUPPLIERS ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/suppliers",
    tag = "suppliers",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("search" = Option<String>, Query, description = "Search term"),
        ("active_only" = Option<bool>, Query, description = "Show only active suppliers"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page"),
    ),
    responses(
        (status = 200, description = "List of suppliers", body = Vec<Supplier>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_suppliers(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Query(params): Query<SupplierSearchParams>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        company_id = %auth_user.company_id,
        search = ?params.search,
        "Listing suppliers"
    );
    
    let (suppliers, total_count) = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            let s = if let Some(ref search_term) = params.search {
                Supplier::search_pg(
                    pool,
                    auth_user.company_id,
                    search_term,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            } else {
                Supplier::list_by_company_pg(
                    pool,
                    auth_user.company_id,
                    params.active_only,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            };

            let count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM suppliers WHERE company_id = $1 AND ($2 = false OR is_active = true)"
            )
            .bind(auth_user.company_id)
            .bind(params.active_only)
            .fetch_one(pool)
            .await?;

            (s, count)
        }
        DbPool::Sqlite(pool) => {
            let s = if let Some(ref search_term) = params.search {
                Supplier::search_sqlite(
                    pool,
                    auth_user.company_id,
                    search_term,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            } else {
                Supplier::list_by_company_sqlite(
                    pool,
                    auth_user.company_id,
                    params.active_only,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            };

            let count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM suppliers WHERE company_id = ? AND (? = 0 OR is_active = 1)"
            )
            .bind(auth_user.company_id)
            .bind(params.active_only)
            .fetch_one(pool)
            .await?;

            (s, count)
        }
    };
    
    tracing::debug!(
        count = suppliers.len(),
        total = total_count,
        "Suppliers retrieved successfully"
    );
    
    Ok(paginated(
        suppliers,
        params.pagination.page,
        params.pagination.per_page,
        total_count,
    ))
}

// ===== GET SUPPLIER ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/suppliers/{id}",
    tag = "suppliers",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Supplier ID")
    ),
    responses(
        (status = 200, description = "Supplier found", body = SupplierWithStats),
        (status = 404, description = "Supplier not found"),
        (status = 403, description = "Access denied")
    )
)]
pub async fn get_supplier(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<SupplierWithStats>> {
    tracing::debug!(
        user_id = %auth_user.id,
        supplier_id = %id,
        "Fetching supplier"
    );
    
    let supplier = match state.db.as_ref() {
        DbPool::Postgres(pool) => Supplier::get_with_stats_pg(pool, id).await?,
        DbPool::Sqlite(pool) => Supplier::get_with_stats_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Supplier not found".to_string()))?;

    verify_company_access(&auth_user, supplier.supplier.company_id)?;
    
    tracing::info!(
        supplier_id = %id,
        name = %supplier.supplier.name,
        "Supplier retrieved successfully"
    );
    
    Ok(Json(supplier))
}

// ===== CREATE SUPPLIER ENDPOINT =====
#[utoipa::path(
    post,
    path = "/api/v1/suppliers",
    tag = "suppliers",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateSupplierRequest,
    responses(
        (status = 201, description = "Supplier created successfully", body = Supplier),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Supplier code already exists"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn create_supplier(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(mut payload): Json<CreateSupplierRequest>,
) -> Result<impl axum::response::IntoResponse> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        supplier_code = %payload.supplier_code,
        "Creating supplier"
    );
    
    payload.company_id = auth_user.company_id;
    
    let supplier = match state.db.as_ref() {
        DbPool::Postgres(pool) => Supplier::create_pg(pool, payload, auth_user.id).await?,
        DbPool::Sqlite(pool) => Supplier::create_sqlite(pool, payload, auth_user.id).await?,
    };
    
    tracing::info!(
        supplier_id = %supplier.id,
        name = %supplier.name,
        "Supplier created successfully"
    );
    
    Ok(created("Supplier created successfully", supplier))
}

// ===== UPDATE SUPPLIER ENDPOINT =====
#[utoipa::path(
    put,
    path = "/api/v1/suppliers/{id}",
    tag = "suppliers",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Supplier ID")
    ),
    request_body = UpdateSupplierRequest,
    responses(
        (status = 200, description = "Supplier updated successfully", body = Supplier),
        (status = 404, description = "Supplier not found"),
        (status = 400, description = "Validation error"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn update_supplier(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateSupplierRequest>,
) -> Result<Json<Supplier>> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        supplier_id = %id,
        "Updating supplier"
    );
    
    let existing_supplier = match state.db.as_ref() {
        DbPool::Postgres(pool) => Supplier::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => Supplier::find_by_id_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Supplier not found".to_string()))?;

    verify_company_access(&auth_user, existing_supplier.company_id)?;
    
    let updated_supplier = match state.db.as_ref() {
        DbPool::Postgres(pool) => Supplier::update_pg(pool, id, payload, auth_user.id).await?,
        DbPool::Sqlite(pool) => Supplier::update_sqlite(pool, id, payload, auth_user.id).await?,
    };
    
    tracing::info!(
        supplier_id = %id,
        "Supplier updated successfully"
    );

    Ok(Json(updated_supplier))
}

// ===== DELETE SUPPLIER ENDPOINT =====
#[utoipa::path(
    delete,
    path = "/api/v1/suppliers/{id}",
    tag = "suppliers",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Supplier ID")
    ),
    responses(
        (status = 204, description = "Supplier deleted successfully"),
        (status = 404, description = "Supplier not found"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn delete_supplier(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        supplier_id = %id,
        "Deleting supplier"
    );
    
    let supplier = match state.db.as_ref() {
        DbPool::Postgres(pool) => Supplier::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => Supplier::find_by_id_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Supplier not found".to_string()))?;

    verify_company_access(&auth_user, supplier.company_id)?;
    
    match state.db.as_ref() {
        DbPool::Postgres(pool) => Supplier::delete_pg(pool, id).await?,
        DbPool::Sqlite(pool) => Supplier::delete_sqlite(pool, id).await?,
    };
    
    tracing::info!(
        supplier_id = %id,
        name = %supplier.name,
        "Supplier deleted successfully"
    );
    
    Ok(no_content())
}

// ===== SUPPLIERS ROUTER =====
pub fn suppliers_router() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/", get(list_suppliers).post(create_supplier))
        .route("/{id}", get(get_supplier).put(update_supplier).delete(delete_supplier))
}