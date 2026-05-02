// src/handlers/stock.rs
// Stock management endpoints
// Stock adjustments, transfers, and movement tracking

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
    models::stock::{
        LowStockAlert, PhysicalCountRequest,
        Stock, StockAdjustmentRequest, StockMovement, StockMovementWithDetails,
        StockTransferRequest, StockWithItem,
    },
    utils::{
        error::{AppError, Result},
        response::{paginated, success},
    },
    AppState,
};

use super::users::PaginationParams;

// ===== LIST STOCK ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/stock",
    tag = "stock",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("branch_id" = Option<Uuid>, Query, description = "Filter by branch"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page"),
    ),
    responses(
        (status = 200, description = "Stock levels", body = Vec<StockWithItem>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_stock(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Query(params): Query<StockQueryParams>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        company_id = %auth_user.company_id,
        branch_id = ?params.branch_id,
        "Listing stock"
    );
    
    // Fetch stock based on filters
    let (stock, total_count) = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            let stock = if let Some(branch_id) = params.branch_id {
                Stock::list_by_branch_pg(
                    pool,
                    branch_id,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            } else {
                Stock::list_by_company_pg(
                    pool,
                    auth_user.company_id,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            };
            
            let total_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM stock WHERE company_id = $1"
            )
            .bind(auth_user.company_id)
            .fetch_one(pool)
            .await?;
            
            (stock, total_count)
        }
        DbPool::Sqlite(pool) => {
            let stock = if let Some(branch_id) = params.branch_id {
                Stock::list_by_branch_sqlite(
                    pool,
                    branch_id,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            } else {
                Stock::list_by_company_sqlite(
                    pool,
                    auth_user.company_id,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            };
            
            let total_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM stock WHERE company_id = ?"
            )
            .bind(auth_user.company_id)
            .fetch_one(pool)
            .await?;
            
            (stock, total_count)
        }
    };
    
    tracing::debug!(
        count = stock.len(),
        total = total_count,
        "Stock retrieved successfully"
    );
    
    Ok(paginated(
        stock,
        params.pagination.page(),
        params.pagination.per_page(),
        total_count,
    ))
}

// ===== STOCK QUERY PARAMETERS =====
#[derive(Debug, Deserialize, ToSchema)]
pub struct StockQueryParams {
    pub branch_id: Option<Uuid>,
    
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

// ===== ADJUST STOCK ENDPOINT =====
#[utoipa::path(
    post,
    path = "/api/v1/stock/adjust",
    tag = "stock",
    security(
        ("bearer_auth" = [])
    ),
    request_body = StockAdjustmentRequest,
    responses(
        (status = 200, description = "Stock adjusted successfully"),
        (status = 400, description = "Validation error"),
        (status = 404, description = "Item or branch not found"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn adjust_stock(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(payload): Json<StockAdjustmentRequest>,
) -> Result<impl axum::response::IntoResponse> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        item_id = %payload.item_id,
        branch_id = %payload.branch_id,
        adjustment = payload.adjustment_quantity,
        "Adjusting stock"
    );
    
    // Verify item belongs to company
    let item = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            crate::models::inventory::InventoryItem::find_by_id_pg(pool, payload.item_id).await?
        }
        DbPool::Sqlite(pool) => {
            crate::models::inventory::InventoryItem::find_by_id_sqlite(pool, payload.item_id).await?
        }
    }
    .ok_or_else(|| AppError::NotFound("Item not found".to_string()))?;
    
    verify_company_access(&auth_user, item.company_id)?;
    
    // Perform stock adjustment
    let stock = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            Stock::adjust_quantity_pg(
                pool,
                payload.item_id,
                payload.branch_id,
                payload.adjustment_quantity,
            ).await?
        }
        DbPool::Sqlite(pool) => {
            Stock::adjust_quantity_sqlite(
                pool,
                payload.item_id,
                payload.branch_id,
                payload.adjustment_quantity,
            ).await?
        }
    };
    
    // Record movement
    let movement = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            StockMovement::create_pg(
                pool,
                auth_user.company_id,
                payload.item_id,
                None,
                Some(payload.branch_id),
                crate::models::stock::MovementType::Adjustment,
                payload.adjustment_quantity.abs(),
                payload.unit_cost,
                None,
                None,
                payload.batch_number,
                payload.serial_numbers,
                Some(payload.reason),
                auth_user.id,
            ).await?
        }
        DbPool::Sqlite(pool) => {
            StockMovement::create_sqlite(
                pool,
                auth_user.company_id,
                payload.item_id,
                None,
                Some(payload.branch_id),
                crate::models::stock::MovementType::Adjustment,
                payload.adjustment_quantity.abs(),
                payload.unit_cost,
                None,
                None,
                payload.batch_number,
                payload.serial_numbers,
                Some(payload.reason),
                auth_user.id,
            ).await?
        }
    };
    
    tracing::info!(
        item_id = %item.id,
        new_quantity = stock.quantity_on_hand,
        "Stock adjusted successfully"
    );
    
    Ok(success(
        "Stock adjusted successfully",
        serde_json::json!({
            "stock": stock,
            "movement": movement,
        }),
    ))
}

// ===== TRANSFER STOCK ENDPOINT =====
#[utoipa::path(
    post,
    path = "/api/v1/stock/transfer",
    tag = "stock",
    security(
        ("bearer_auth" = [])
    ),
    request_body = StockTransferRequest,
    responses(
        (status = 200, description = "Stock transferred successfully"),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Insufficient stock"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn transfer_stock(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(payload): Json<StockTransferRequest>,
) -> Result<impl axum::response::IntoResponse> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        item_id = %payload.item_id,
        from_branch = %payload.from_branch_id,
        to_branch = %payload.to_branch_id,
        quantity = payload.quantity,
        "Transferring stock"
    );
    
    // Prevent transferring to same branch
    if payload.from_branch_id == payload.to_branch_id {
        return Err(AppError::ValidationError(
            "Cannot transfer stock to the same branch".to_string()
        ));
    }
    
    // Verify item belongs to company
    let item = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            crate::models::inventory::InventoryItem::find_by_id_pg(pool, payload.item_id).await?
        }
        DbPool::Sqlite(pool) => {
            crate::models::inventory::InventoryItem::find_by_id_sqlite(pool, payload.item_id).await?
        }
    }
    .ok_or_else(|| AppError::NotFound("Item not found".to_string()))?;
    
    verify_company_access(&auth_user, item.company_id)?;
    
    // Perform transfer
    let (from_stock, to_stock, movement) = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            Stock::transfer_pg(
                pool,
                &payload,
                auth_user.company_id,
                auth_user.id,
            ).await?
        }
        DbPool::Sqlite(pool) => {
            Stock::transfer_sqlite(
                pool,
                &payload,
                auth_user.company_id,
                auth_user.id,
            ).await?
        }
    };
    
    tracing::info!(
        item_id = %item.id,
        "Stock transferred successfully"
    );
    
    Ok(success(
        "Stock transferred successfully",
        serde_json::json!({
            "from_stock": from_stock,
            "to_stock": to_stock,
            "movement": movement,
        }),
    ))
}

// ===== LIST STOCK MOVEMENTS ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/stock/movements",
    tag = "stock",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("item_id" = Option<Uuid>, Query, description = "Filter by item"),
        ("branch_id" = Option<Uuid>, Query, description = "Filter by branch"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page"),
    ),
    responses(
        (status = 200, description = "Stock movements", body = Vec<StockMovementWithDetails>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_movements(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Query(params): Query<MovementQueryParams>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        company_id = %auth_user.company_id,
        "Listing stock movements"
    );
    
    // Fetch movements based on filters
    let (movements, total_count) = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            let movements = if let Some(item_id) = params.item_id {
                StockMovement::list_by_item_pg(
                    pool,
                    item_id,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            } else if let Some(branch_id) = params.branch_id {
                StockMovement::list_by_branch_pg(
                    pool,
                    branch_id,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            } else {
                StockMovement::list_by_company_pg(
                    pool,
                    auth_user.company_id,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            };
            
            let total_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM stock_movements WHERE company_id = $1"
            )
            .bind(auth_user.company_id)
            .fetch_one(pool)
            .await?;
            
            (movements, total_count)
        }
        DbPool::Sqlite(pool) => {
            let movements = if let Some(item_id) = params.item_id {
                StockMovement::list_by_item_sqlite(
                    pool,
                    item_id,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            } else if let Some(branch_id) = params.branch_id {
                StockMovement::list_by_branch_sqlite(
                    pool,
                    branch_id,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            } else {
                StockMovement::list_by_company_sqlite(
                    pool,
                    auth_user.company_id,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            };
            
            let total_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM stock_movements WHERE company_id = ?"
            )
            .bind(auth_user.company_id)
            .fetch_one(pool)
            .await?;
            
            (movements, total_count)
        }
    };
    
    tracing::debug!(
        count = movements.len(),
        total = total_count,
        "Stock movements retrieved"
    );
    
    Ok(paginated(
        movements,
        params.pagination.page(),
        params.pagination.per_page(),
        total_count,
    ))
}

// ===== MOVEMENT QUERY PARAMETERS =====
#[derive(Debug, Deserialize, ToSchema)]
pub struct MovementQueryParams {
    pub item_id: Option<Uuid>,
    pub branch_id: Option<Uuid>,
    
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

// ===== PHYSICAL COUNT ENDPOINT =====
#[utoipa::path(
    post,
    path = "/api/v1/stock/physical-count",
    tag = "stock",
    security(
        ("bearer_auth" = [])
    ),
    request_body = PhysicalCountRequest,
    responses(
        (status = 200, description = "Physical count recorded", body = Stock),
        (status = 400, description = "Validation error"),
        (status = 404, description = "Stock record not found"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn physical_count(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(payload): Json<PhysicalCountRequest>,
) -> Result<Json<Stock>> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        item_id = %payload.item_id,
        branch_id = %payload.branch_id,
        counted_qty = payload.counted_quantity,
        "Recording physical count"
    );
    
    // Verify item belongs to company
    let item = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            crate::models::inventory::InventoryItem::find_by_id_pg(pool, payload.item_id).await?
        }
        DbPool::Sqlite(pool) => {
            crate::models::inventory::InventoryItem::find_by_id_sqlite(pool, payload.item_id).await?
        }
    }
    .ok_or_else(|| AppError::NotFound("Item not found".to_string()))?;
    
    verify_company_access(&auth_user, item.company_id)?;
    
    // Record physical count
    let stock = match state.db.as_ref() {
        DbPool::Postgres(pool) => Stock::record_count_pg(pool, payload).await?,
        DbPool::Sqlite(pool) => Stock::record_count_sqlite(pool, payload).await?,
    };
    
    tracing::info!(
        item_id = %item.id,
        variance = ?stock.variance,
        "Physical count recorded"
    );
    
    Ok(Json(stock))
}

// ===== LOW STOCK ALERTS ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/stock/low-stock-alerts",
    tag = "stock",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Low stock alerts", body = Vec<LowStockAlert>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn low_stock_alerts(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
) -> Result<Json<Vec<LowStockAlert>>> {
    tracing::info!(
        user_id = %auth_user.id,
        company_id = %auth_user.company_id,
        "Fetching low stock alerts"
    );
    
    // Get low stock alerts
    let alerts = match state.db.as_ref() {
        DbPool::Postgres(pool) => Stock::get_low_stock_items_pg(pool, auth_user.company_id).await?,
        DbPool::Sqlite(pool) => Stock::get_low_stock_items_sqlite(pool, auth_user.company_id).await?,
    };
    
    tracing::info!(
        count = alerts.len(),
        "Low stock alerts retrieved"
    );
    
    Ok(Json(alerts))
}

pub fn stock_router() -> axum::Router<Arc<AppState>> {
    use axum::routing::{get, post};

    axum::Router::new()
        .route("/", get(list_stock))
        .route("/adjust", post(adjust_stock))
        .route("/transfer", post(transfer_stock))
        .route("/movements", get(list_movements))
        .route("/physical-count", post(physical_count))
        .route("/low-stock-alerts", get(low_stock_alerts))
}