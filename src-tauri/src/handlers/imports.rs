// src/handlers/imports.rs
// Import order management endpoints
// International shipment tracking with customs and duties

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    config::DbPool,
    middleware::auth::{verify_company_access, AuthUser},
    models::import::{CreateImportOrderRequest, ImportOrder, ImportOrderWithDetails, UpdateImportOrderRequest},
    utils::{
        error::{AppError, Result},
        response::{created, no_content, paginated, success},
    },
    AppState,
};

use super::users::PaginationParams;

// ===== IMPORT ORDER QUERY PARAMETERS =====
#[derive(Debug, Deserialize, ToSchema)]
pub struct ImportOrderQueryParams {
    pub status: Option<String>,
    pub supplier_id: Option<Uuid>,
    
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

// ===== LIST IMPORT ORDERS ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/imports",
    tag = "imports",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("status" = Option<String>, Query, description = "Filter by status"),
        ("supplier_id" = Option<Uuid>, Query, description = "Filter by supplier"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page"),
    ),
    responses(
        (status = 200, description = "List of import orders", body = Vec<ImportOrder>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_imports(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Query(params): Query<ImportOrderQueryParams>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        company_id = %auth_user.company_id,
        "Listing import orders"
    );
    
    let (imports, total_count) = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            let imports = if let Some(supplier_id) = params.supplier_id {
                ImportOrder::list_by_supplier_pg(
                    pool,
                    supplier_id,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            } else {
                ImportOrder::list_by_company_pg(
                    pool,
                    auth_user.company_id,
                    params.status,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            };
            
            let total_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM import_orders WHERE company_id = $1"
            )
            .bind(auth_user.company_id)
            .fetch_one(pool)
            .await?;
            
            (imports, total_count)
        }
        DbPool::Sqlite(pool) => {
            let imports = if let Some(supplier_id) = params.supplier_id {
                ImportOrder::list_by_supplier_sqlite(
                    pool,
                    supplier_id,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            } else {
                ImportOrder::list_by_company_sqlite(
                    pool,
                    auth_user.company_id,
                    params.status,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            };
            
            let total_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM import_orders WHERE company_id = ?"
            )
            .bind(auth_user.company_id)
            .fetch_one(pool)
            .await?;
            
            (imports, total_count)
        }
    };
    
    tracing::debug!(
        count = imports.len(),
        total = total_count,
        "Import orders retrieved successfully"
    );
    
    Ok(paginated(
        imports,
        params.pagination.page,
        params.pagination.per_page,
        total_count,
    ))
}

// ===== GET IMPORT ORDER ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/imports/{id}",
    tag = "imports",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Import Order ID")
    ),
    responses(
        (status = 200, description = "Import order found", body = ImportOrderWithDetails),
        (status = 404, description = "Import order not found"),
        (status = 403, description = "Access denied")
    )
)]
pub async fn get_import(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ImportOrderWithDetails>> {
    tracing::debug!(
        user_id = %auth_user.id,
        import_id = %id,
        "Fetching import order"
    );
    
    let import_order = match state.db.as_ref() {
        DbPool::Postgres(pool) => ImportOrder::get_with_details_pg(pool, id).await?,
        DbPool::Sqlite(pool) => ImportOrder::get_with_details_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Import order not found".to_string()))?;
    
    verify_company_access(&auth_user, import_order.import_order.company_id)?;
    
    tracing::info!(
        import_id = %id,
        import_number = %import_order.import_order.import_number,
        "Import order retrieved successfully"
    );
    
    Ok(Json(import_order))
}

// ===== CREATE IMPORT ORDER ENDPOINT =====
#[utoipa::path(
    post,
    path = "/api/v1/imports",
    tag = "imports",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateImportOrderRequest,
    responses(
        (status = 201, description = "Import order created successfully", body = ImportOrder),
        (status = 400, description = "Validation error"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn create_import(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(mut payload): Json<CreateImportOrderRequest>,
) -> Result<impl axum::response::IntoResponse> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        supplier_id = %payload.supplier_id,
        "Creating import order"
    );
    
    payload.company_id = auth_user.company_id;
    
    // Verify supplier belongs to same company
    let supplier = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            crate::models::supplier::Supplier::find_by_id_pg(pool, payload.supplier_id).await?
        }
        DbPool::Sqlite(pool) => {
            crate::models::supplier::Supplier::find_by_id_sqlite(pool, payload.supplier_id).await?
        }
    }
    .ok_or_else(|| AppError::NotFound("Supplier not found".to_string()))?;
    
    verify_company_access(&auth_user, supplier.company_id)?;
    
    let import_order = match state.db.as_ref() {
        DbPool::Postgres(pool) => ImportOrder::create_pg(pool, payload, auth_user.id).await?,
        DbPool::Sqlite(pool) => ImportOrder::create_sqlite(pool, payload, auth_user.id).await?,
    };
    
    tracing::info!(
        import_id = %import_order.id,
        import_number = %import_order.import_number,
        "Import order created successfully"
    );
    
    Ok(created("Import order created successfully", import_order))
}

// ===== UPDATE IMPORT ORDER ENDPOINT =====
#[utoipa::path(
    put,
    path = "/api/v1/imports/{id}",
    tag = "imports",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Import Order ID")
    ),
    request_body = UpdateImportOrderRequest,
    responses(
        (status = 200, description = "Import order updated successfully", body = ImportOrder),
        (status = 404, description = "Import order not found"),
        (status = 400, description = "Validation error"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn update_import(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateImportOrderRequest>,
) -> Result<Json<ImportOrder>> {
    tracing::info!(
        user_id = %auth_user.id,
        import_id = %id,
        "Updating import order"
    );
    
    let existing_import = match state.db.as_ref() {
        DbPool::Postgres(pool) => ImportOrder::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => ImportOrder::find_by_id_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Import order not found".to_string()))?;
    
    verify_company_access(&auth_user, existing_import.company_id)?;
    
    let updated_import = match state.db.as_ref() {
        DbPool::Postgres(pool) => ImportOrder::update_pg(pool, id, payload, auth_user.id).await?,
        DbPool::Sqlite(pool) => ImportOrder::update_sqlite(pool, id, payload, auth_user.id).await?,
    };
    
    tracing::info!(
        import_id = %id,
        "Import order updated successfully"
    );
    
    Ok(Json(updated_import))
}

// ===== DELETE IMPORT ORDER ENDPOINT =====
#[utoipa::path(
    delete,
    path = "/api/v1/imports/{id}",
    tag = "imports",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Import Order ID")
    ),
    responses(
        (status = 204, description = "Import order deleted successfully"),
        (status = 404, description = "Import order not found"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn delete_import(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        import_id = %id,
        "Deleting import order"
    );
    
    let import_order = match state.db.as_ref() {
        DbPool::Postgres(pool) => ImportOrder::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => ImportOrder::find_by_id_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Import order not found".to_string()))?;
    
    verify_company_access(&auth_user, import_order.company_id)?;
    
    match state.db.as_ref() {
        DbPool::Postgres(pool) => ImportOrder::delete_pg(pool, id).await?,
        DbPool::Sqlite(pool) => ImportOrder::delete_sqlite(pool, id).await?,
    };
    
    tracing::info!(
        import_id = %id,
        import_number = %import_order.import_number,
        "Import order deleted successfully"
    );
    
    Ok(no_content())
}

pub fn imports_router() -> axum::Router<Arc<AppState>> {
    use axum::routing::{delete, get, post, put};

    axum::Router::new()
        .route("/", get(list_imports).post(create_import))
        .route("/:id", get(get_import).put(update_import).delete(delete_import))
}