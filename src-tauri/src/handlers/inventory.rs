// src/handlers/inventory.rs
// Inventory item management endpoints
// CRUD operations for inventory items with stock information

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState, config::DbPool, middleware::auth::{AuthUser, verify_company_access}, models::inventory::{
        CreateItemRequest, InventoryItem, InventoryItemWithStock, UpdateItemRequest,
    }, utils::{
        error::{AppError, Result},
        response::{created, no_content, paginated, success},
    }
};

use super::users::PaginationParams;

// ===== INVENTORY SEARCH QUERY PARAMETERS =====
#[derive(Debug, Deserialize, ToSchema)]
pub struct InventorySearchParams {
    // Search term (SKU, name, barcode)
    pub search: Option<String>,
    
    // Filter by category
    pub category_id: Option<Uuid>,
    
    // Filter by active status
    #[serde(default = "default_true")]
    pub active_only: bool,
    
    // Include stock information
    #[serde(default)]
    pub include_stock: bool,
    
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

fn default_true() -> bool { true }

// ===== HELPER: Fetch inventory items based on search params =====
async fn fetch_inventory_items<DB: sqlx::Database>(
    pool: &sqlx::Pool<DB>,
    auth_user: &AuthUser,
    params: &InventorySearchParams,
) -> Result<Vec<InventoryItem>>
where
    // We need different implementations for each DB type
{
    // This function is called inside match arms where we know the DB type,
    // so we'll create separate implementations below the match
    unimplemented!("Use the match arms directly")
}

// ===== LIST INVENTORY ITEMS ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/inventory",
    tag = "inventory",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page"),
        ("search" = Option<String>, Query, description = "Search term"),
        ("category_id" = Option<Uuid>, Query, description = "Filter by category"),
        ("active_only" = Option<bool>, Query, description = "Show only active items"),
        ("include_stock" = Option<bool>, Query, description = "Include stock information"),
    ),
    responses(
        (status = 200, description = "List of inventory items", body = Vec<InventoryItem>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_items(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Query(params): Query<InventorySearchParams>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        company_id = %auth_user.company_id,
        search = ?params.search,
        "Listing inventory items"
    );

    // We execute the logic based on the DB type
    let (items, total_count) = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            // Fetch items based on search criteria
            let items = if let Some(ref search_term) = params.search {
                InventoryItem::search_pg(
                    pool,
                    auth_user.company_id,
                    search_term,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            } else if let Some(category_id) = params.category_id {
                InventoryItem::list_by_category_pg(
                    pool,
                    category_id,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            } else {
                InventoryItem::list_by_company_pg(
                    pool,
                    auth_user.company_id,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            };
            
            let total_count: i64 = sqlx::query_scalar(
                r#"
                SELECT COUNT(*) FROM inventory_items 
                WHERE company_id = $1 AND ($2 = false OR is_active = true)
                "#
            )
            .bind(auth_user.company_id)
            .bind(params.active_only)
            .fetch_one(pool)
            .await?;

            (items, total_count)
        }
        DbPool::Sqlite(pool) => {
            let items = if let Some(ref search_term) = params.search {
                InventoryItem::search_sqlite(
                    pool,
                    auth_user.company_id,
                    search_term,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            } else if let Some(category_id) = params.category_id {
                InventoryItem::list_by_category_sqlite(
                    pool,
                    category_id,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            } else {
                InventoryItem::list_by_company_sqlite(
                    pool,
                    auth_user.company_id,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            };
            
            let total_count: i64 = sqlx::query_scalar(
                r#"
                SELECT COUNT(*) FROM inventory_items 
                WHERE company_id = ? AND (? = 0 OR is_active = 1)
                "#
            )
            .bind(auth_user.company_id)
            .bind(params.active_only)
            .fetch_one(pool)
            .await?;

            (items, total_count)
        }
    };

    tracing::debug!(
        count = items.len(),
        total = total_count,
        "Inventory items retrieved successfully"
    );

    Ok(paginated(
        items,
        params.pagination.page(),
        params.pagination.per_page(),
        total_count,
    ))
}

// ===== GET INVENTORY ITEM ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/inventory/{id}",
    tag = "inventory",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Item ID")
    ),
    responses(
        (status = 200, description = "Item found", body = InventoryItemWithStock),
        (status = 404, description = "Item not found"),
        (status = 403, description = "Access denied")
    )
)]
pub async fn get_item(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<InventoryItemWithStock>> {
    tracing::debug!(
        user_id = %auth_user.id,
        item_id = %id,
        "Fetching inventory item"
    );
    
    // Get item with stock based on DB type
    let item_option = match state.db.as_ref() {
        DbPool::Postgres(pool) => InventoryItem::get_with_stock_pg(pool, id).await?,
        DbPool::Sqlite(pool) => InventoryItem::get_with_stock_sqlite(pool, id).await?,
    };

    let item = item_option.ok_or_else(|| AppError::NotFound("Inventory item not found".to_string()))?;

    // Verify item belongs to same company
    verify_company_access(&auth_user, item.item.company_id)?;
    
    tracing::info!(
        item_id = %id,
        sku = %item.item.sku,
        "Inventory item retrieved successfully"
    );
    
    Ok(Json(item))
}

// ===== CREATE INVENTORY ITEM ENDPOINT =====
#[utoipa::path(
    post,
    path = "/api/v1/inventory",
    tag = "inventory",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateItemRequest,
    responses(
        (status = 201, description = "Item created successfully", body = InventoryItem),
        (status = 400, description = "Validation error"),
        (status = 409, description = "SKU already exists"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn create_item(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(mut payload): Json<CreateItemRequest>,
) -> Result<impl axum::response::IntoResponse> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        sku = %payload.sku,
        "Creating inventory item"
    );
    
    // Force company_id to match authenticated user's company
    payload.company_id = auth_user.company_id;
    
    // Check if SKU already exists based on DB type
    let sku_exists = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            InventoryItem::find_by_sku_pg(pool, auth_user.company_id, &payload.sku)
                .await?
                .is_some()
        }
        DbPool::Sqlite(pool) => {
            InventoryItem::find_by_sku_sqlite(pool, auth_user.company_id, &payload.sku)
                .await?
                .is_some()
        }
    };

    if sku_exists {
        tracing::warn!(
            sku = %payload.sku,
            "Item creation failed: SKU already exists"
        );
        return Err(AppError::DuplicateKey("SKU already exists".to_string()));
    }
    
    // Create item based on DB type
    let item = match state.db.as_ref() {
        DbPool::Postgres(pool) => InventoryItem::create_pg(pool, payload, auth_user.id).await?,
        DbPool::Sqlite(pool) => InventoryItem::create_sqlite(pool, payload, auth_user.id).await?,
    };
    
    tracing::info!(
        item_id = %item.id,
        sku = %item.sku,
        "Inventory item created successfully"
    );
    
    Ok(created("Inventory item created successfully", item))
}

// ===== UPDATE INVENTORY ITEM ENDPOINT =====
#[utoipa::path(
    put,
    path = "/api/v1/inventory/{id}",
    tag = "inventory",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Item ID")
    ),
    request_body = UpdateItemRequest,
    responses(
        (status = 200, description = "Item updated successfully", body = InventoryItem),
        (status = 404, description = "Item not found"),
        (status = 400, description = "Validation error"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn update_item(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateItemRequest>,
) -> Result<Json<InventoryItem>> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        item_id = %id,
        "Updating inventory item"
    );
    
    // Fetch existing item based on DB type
    let existing_item = match state.db.as_ref() {
        DbPool::Postgres(pool) => InventoryItem::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => InventoryItem::find_by_id_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Inventory item not found".to_string()))?;
    
    // Verify item belongs to same company
    verify_company_access(&auth_user, existing_item.company_id)?;
    
    // Update item based on DB type
    let updated_item = match state.db.as_ref() {
        DbPool::Postgres(pool) => InventoryItem::update_pg(pool, id, payload, auth_user.id).await?,
        DbPool::Sqlite(pool) => InventoryItem::update_sqlite(pool, id, payload, auth_user.id).await?,
    };
    
    tracing::info!(
        item_id = %id,
        "Inventory item updated successfully"
    );
    
    Ok(Json(updated_item))
}

// ===== DELETE INVENTORY ITEM ENDPOINT =====
#[utoipa::path(
    delete,
    path = "/api/v1/inventory/{id}",
    tag = "inventory",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Item ID")
    ),
    responses(
        (status = 204, description = "Item deleted successfully"),
        (status = 404, description = "Item not found"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn delete_item(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        item_id = %id,
        "Deleting inventory item"
    );
    
    // Fetch item to verify company ownership based on DB type
    let item = match state.db.as_ref() {
        DbPool::Postgres(pool) => InventoryItem::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => InventoryItem::find_by_id_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Inventory item not found".to_string()))?;
    
    // Verify item belongs to same company
    verify_company_access(&auth_user, item.company_id)?;
    
    // Soft delete item based on DB type
    match state.db.as_ref() {
        DbPool::Postgres(pool) => InventoryItem::delete_pg(pool, id, auth_user.id).await?,
        DbPool::Sqlite(pool) => InventoryItem::delete_sqlite(pool, id, auth_user.id).await?,
    };
    
    tracing::info!(
        item_id = %id,
        sku = %item.sku,
        "Inventory item deleted successfully"
    );
    
    Ok(no_content())
}

// ===== GET ITEM STOCK ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/inventory/{id}/stock",
    tag = "inventory",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Item ID")
    ),
    responses(
        (status = 200, description = "Stock levels", body = Vec<Stock>),
        (status = 404, description = "Item not found"),
        (status = 403, description = "Access denied")
    )
)]
pub async fn get_item_stock(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<crate::models::stock::Stock>>> {
    tracing::debug!(
        user_id = %auth_user.id,
        item_id = %id,
        "Fetching item stock levels"
    );
    
    // Verify item exists and belongs to company based on DB type
    let item = match state.db.as_ref() {
        DbPool::Postgres(pool) => InventoryItem::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => InventoryItem::find_by_id_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Inventory item not found".to_string()))?;
    
    verify_company_access(&auth_user, item.company_id)?;
    
    // Get stock levels across all branches based on DB type
    let stock = match state.db.as_ref() {
        DbPool::Postgres(pool) => crate::models::stock::Stock::list_by_item_pg(pool, id).await?,
        DbPool::Sqlite(pool) => crate::models::stock::Stock::list_by_item_sqlite(pool, id).await?,
    };
    
    tracing::info!(
        item_id = %id,
        branches_count = stock.len(),
        "Item stock levels retrieved"
    );
    
    Ok(Json(stock))
}

// ===== LOW STOCK ITEMS ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/inventory/low-stock",
    tag = "inventory",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Low stock items", body = Vec<InventoryItemWithStock>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn low_stock_items(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
) -> Result<Json<Vec<InventoryItemWithStock>>> {
    tracing::info!(
        user_id = %auth_user.id,
        company_id = %auth_user.company_id,
        "Fetching low stock items"
    );
    
    // Get low stock items based on DB type
    let items = match state.db.as_ref() {
        DbPool::Postgres(pool) => InventoryItem::get_low_stock_items_pg(pool, auth_user.company_id).await?,
        DbPool::Sqlite(pool) => InventoryItem::get_low_stock_items_sqlite(pool, auth_user.company_id).await?,
    };
    
    tracing::info!(
        count = items.len(),
        "Low stock items retrieved"
    );
    
    Ok(Json(items))
}

pub fn inventory_router() -> axum::Router<Arc<AppState>> {
    use axum::routing::{delete, get, post, put};

    axum::Router::new()
        .route("/", get(list_items).post(create_item))
        .route("/low-stock", get(low_stock_items))
        .route("/:id", get(get_item).put(update_item).delete(delete_item))
        .route("/:id/stock", get(get_item_stock))
}