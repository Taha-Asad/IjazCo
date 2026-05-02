// src/handlers/purchases.rs
// Purchase order management endpoints
// PO creation, submission, goods receipt, and tracking

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
    config::DbPool,
    middleware::auth::{verify_company_access, AuthUser},
    models::purchase::{
        CreatePurchaseOrderRequest, PurchaseOrder, PurchaseOrderWithItems, PurchaseStatus,
        ReceiveGoodsRequest, UpdatePurchaseOrderRequest, PurchaseOrderItem,
    },
    utils::{
        error::{AppError, Result},
        response::{created, no_content, paginated, success},
    },
    AppState,
};

use super::users::PaginationParams;

// ===== PURCHASE ORDER QUERY PARAMETERS =====
#[derive(Debug, Deserialize, ToSchema)]
pub struct PurchaseOrderQueryParams {
    pub status: Option<PurchaseStatus>,
    pub supplier_id: Option<Uuid>,
    pub branch_id: Option<Uuid>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

// ===== LIST PURCHASE ORDERS ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/purchases/orders",
    tag = "purchases",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("status" = Option<PurchaseStatus>, Query, description = "Filter by status"),
        ("supplier_id" = Option<Uuid>, Query, description = "Filter by supplier"),
        ("branch_id" = Option<Uuid>, Query, description = "Filter by branch"),
        ("start_date" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("end_date" = Option<String>, Query, description = "End date (YYYY-MM-DD)"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page"),
    ),
    responses(
        (status = 200, description = "List of purchase orders", body = Vec<PurchaseOrder>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_orders(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Query(params): Query<PurchaseOrderQueryParams>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        company_id = %auth_user.company_id,
        status = ?params.status,
        "Listing purchase orders"
    );
    
    let (orders, total_count) = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            let orders = PurchaseOrder::list_by_company_pg(
                pool,
                auth_user.company_id,
                params.status,
                params.pagination.limit(),
                params.pagination.offset(),
            ).await?;
            
            let total_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM purchase_orders WHERE company_id = $1"
            )
            .bind(auth_user.company_id)
            .fetch_one(pool)
            .await?;
            
            (orders, total_count)
        }
        DbPool::Sqlite(pool) => {
            let orders = PurchaseOrder::list_by_company_sqlite(
                pool,
                auth_user.company_id,
                params.status,
                params.pagination.limit(),
                params.pagination.offset(),
            ).await?;
            
            let total_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM purchase_orders WHERE company_id = ?"
            )
            .bind(auth_user.company_id)
            .fetch_one(pool)
            .await?;
            
            (orders, total_count)
        }
    };
    
    tracing::debug!(
        count = orders.len(),
        total = total_count,
        "Purchase orders retrieved successfully"
    );
    
    Ok(paginated(
        orders,
        params.pagination.page(),
        params.pagination.per_page(),
        total_count,
    ))
}

// ===== GET PURCHASE ORDER ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/purchases/orders/{id}",
    tag = "purchases",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Purchase Order ID")
    ),
    responses(
        (status = 200, description = "Purchase order found", body = PurchaseOrderWithItems),
        (status = 404, description = "Purchase order not found"),
        (status = 403, description = "Access denied")
    )
)]
pub async fn get_order(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<PurchaseOrderWithItems>> {
    tracing::debug!(
        user_id = %auth_user.id,
        po_id = %id,
        "Fetching purchase order"
    );
    
    let po = match state.db.as_ref() {
        DbPool::Postgres(pool) => PurchaseOrder::get_with_items_pg(pool, id).await?,
        DbPool::Sqlite(pool) => PurchaseOrder::get_with_items_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Purchase order not found".to_string()))?;
    
    verify_company_access(&auth_user, po.purchase_order.company_id)?;
    
    tracing::info!(
        po_id = %id,
        po_number = %po.purchase_order.po_number,
        "Purchase order retrieved successfully"
    );
    
    Ok(Json(po))
}

// ===== CREATE PURCHASE ORDER ENDPOINT =====
#[utoipa::path(
    post,
    path = "/api/v1/purchases/orders",
    tag = "purchases",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreatePurchaseOrderRequest,
    responses(
        (status = 201, description = "Purchase order created successfully", body = PurchaseOrderWithItems),
        (status = 400, description = "Validation error"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn create_order(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(mut payload): Json<CreatePurchaseOrderRequest>,
) -> Result<impl axum::response::IntoResponse> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        supplier_id = %payload.supplier_id,
        items_count = payload.items.len(),
        "Creating purchase order"
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
    
    // Create purchase order
    let po = match state.db.as_ref() {
        DbPool::Postgres(pool) => PurchaseOrder::create_pg(pool, payload, auth_user.id).await?,
        DbPool::Sqlite(pool) => PurchaseOrder::create_sqlite(pool, payload, auth_user.id).await?,
    };
    
    tracing::info!(
        po_id = %po.purchase_order.id,
        po_number = %po.purchase_order.po_number,
        total_amount = %po.purchase_order.total_amount,
        "Purchase order created successfully"
    );
    
    Ok(created("Purchase order created successfully", po))
}

// ===== UPDATE PURCHASE ORDER ENDPOINT =====
#[utoipa::path(
    put,
    path = "/api/v1/purchases/orders/{id}",
    tag = "purchases",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Purchase Order ID")
    ),
    request_body = UpdatePurchaseOrderRequest,
    responses(
        (status = 200, description = "Purchase order updated successfully", body = PurchaseOrder),
        (status = 404, description = "Purchase order not found"),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Purchase order cannot be modified in current status"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn update_order(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePurchaseOrderRequest>,
) -> Result<Json<PurchaseOrder>> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        po_id = %id,
        "Updating purchase order"
    );
    
    let existing_po = match state.db.as_ref() {
        DbPool::Postgres(pool) => PurchaseOrder::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => PurchaseOrder::find_by_id_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Purchase order not found".to_string()))?;
    
    verify_company_access(&auth_user, existing_po.company_id)?;
    
    if existing_po.status != PurchaseStatus::Draft {
        return Err(AppError::InvalidStatus {
            entity: "Purchase Order".to_string(),
            current_status: format!("{:?}", existing_po.status),
            allowed_statuses: vec!["Draft".to_string()],
        });
    }
    
    let updated_po = match state.db.as_ref() {
        DbPool::Postgres(pool) => PurchaseOrder::update_pg(pool, id, payload, auth_user.id).await?,
        DbPool::Sqlite(pool) => PurchaseOrder::update_sqlite(pool, id, payload, auth_user.id).await?,
    };
    
    tracing::info!(
        po_id = %id,
        "Purchase order updated successfully"
    );
    
    Ok(Json(updated_po))
}

// ===== DELETE PURCHASE ORDER ENDPOINT =====
#[utoipa::path(
    delete,
    path = "/api/v1/purchases/orders/{id}",
    tag = "purchases",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Purchase Order ID")
    ),
    responses(
        (status = 204, description = "Purchase order deleted successfully"),
        (status = 404, description = "Purchase order not found"),
        (status = 409, description = "Purchase order cannot be deleted in current status"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn delete_order(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        po_id = %id,
        "Deleting purchase order"
    );
    
    let po = match state.db.as_ref() {
        DbPool::Postgres(pool) => PurchaseOrder::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => PurchaseOrder::find_by_id_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Purchase order not found".to_string()))?;
    
    verify_company_access(&auth_user, po.company_id)?;
    
    match state.db.as_ref() {
        DbPool::Postgres(pool) => PurchaseOrder::delete_pg(pool, id).await?,
        DbPool::Sqlite(pool) => PurchaseOrder::delete_sqlite(pool, id).await?,
    };
    
    tracing::info!(
        po_id = %id,
        po_number = %po.po_number,
        "Purchase order deleted successfully"
    );
    
    Ok(no_content())
}

// ===== SUBMIT PURCHASE ORDER ENDPOINT =====
#[utoipa::path(
    post,
    path = "/api/v1/purchases/orders/{id}/submit",
    tag = "purchases",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Purchase Order ID")
    ),
    responses(
        (status = 200, description = "Purchase order submitted successfully", body = PurchaseOrder),
        (status = 404, description = "Purchase order not found"),
        (status = 409, description = "Invalid status for submission"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn submit_order(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<PurchaseOrder>> {
    tracing::info!(
        user_id = %auth_user.id,
        po_id = %id,
        "Submitting purchase order"
    );
    
    let po = match state.db.as_ref() {
        DbPool::Postgres(pool) => PurchaseOrder::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => PurchaseOrder::find_by_id_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Purchase order not found".to_string()))?;
    
    verify_company_access(&auth_user, po.company_id)?;
    
    let submitted_po = match state.db.as_ref() {
        DbPool::Postgres(pool) => PurchaseOrder::submit_pg(pool, id, auth_user.id).await?,
        DbPool::Sqlite(pool) => PurchaseOrder::submit_sqlite(pool, id, auth_user.id).await?,
    };
    
    tracing::info!(
        po_id = %id,
        po_number = %submitted_po.po_number,
        "Purchase order submitted successfully"
    );
    
    Ok(Json(submitted_po))
}

// ===== RECEIVE GOODS ENDPOINT =====
#[utoipa::path(
    post,
    path = "/api/v1/purchases/orders/{id}/receive",
    tag = "purchases",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Purchase Order ID")
    ),
    request_body = ReceiveGoodsRequest,
    responses(
        (status = 200, description = "Goods received successfully", body = PurchaseOrder),
        (status = 404, description = "Purchase order not found"),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Invalid quantity or status"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn receive_goods(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<ReceiveGoodsRequest>,
) -> Result<Json<PurchaseOrder>> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        po_id = %id,
        items_count = payload.items.len(),
        "Receiving goods"
    );
    
    let po = match state.db.as_ref() {
        DbPool::Postgres(pool) => PurchaseOrder::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => PurchaseOrder::find_by_id_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Purchase order not found".to_string()))?;
    
    verify_company_access(&auth_user, po.company_id)?;
    
    let updated_po = match state.db.as_ref() {
        DbPool::Postgres(pool) => PurchaseOrder::receive_goods_pg(pool, id, payload, auth_user.id).await?,
        DbPool::Sqlite(pool) => PurchaseOrder::receive_goods_sqlite(pool, id, payload, auth_user.id).await?,
    };
    
    tracing::info!(
        po_id = %id,
        new_status = ?updated_po.status,
        "Goods received and stock updated"
    );
    
    Ok(Json(updated_po))
}

// ===== GET PO ITEMS ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/purchases/orders/{id}/items",
    tag = "purchases",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Purchase Order ID")
    ),
    responses(
        (status = 200, description = "Purchase order items", body = Vec<PurchaseOrderItem>),
        (status = 404, description = "Purchase order not found"),
        (status = 403, description = "Access denied")
    )
)]
pub async fn get_order_items(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<PurchaseOrderItem>>> {
    tracing::debug!(
        user_id = %auth_user.id,
        po_id = %id,
        "Fetching purchase order items"
    );
    
    let po = match state.db.as_ref() {
        DbPool::Postgres(pool) => PurchaseOrder::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => PurchaseOrder::find_by_id_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Purchase order not found".to_string()))?;
    
    verify_company_access(&auth_user, po.company_id)?;
    
    let items = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            sqlx::query_as::<sqlx::Postgres, PurchaseOrderItem>(
                "SELECT * FROM purchase_order_items WHERE po_id = $1 ORDER BY created_at"
            )
            .bind(id)
            .fetch_all(pool)
            .await?
        }
        DbPool::Sqlite(pool) => {
            let items_sqlite = sqlx::query_as::<sqlx::Sqlite, crate::models::purchase::PurchaseOrderItemSqlite>(
                "SELECT * FROM purchase_order_items WHERE po_id = ? ORDER BY created_at"
            )
            .bind(id)
            .fetch_all(pool)
            .await?;
            
            items_sqlite.into_iter().map(PurchaseOrderItem::from).collect()
        }
    };
    
    tracing::debug!(
        count = items.len(),
        "Purchase order items retrieved"
    );
    
    Ok(Json(items))
}

pub fn purchases_router() -> axum::Router<Arc<AppState>> {
    use axum::routing::{delete, get, post, put};

    axum::Router::new()
        .route("/orders", get(list_orders).post(create_order))
        .route("/orders/:id", get(get_order).put(update_order).delete(delete_order))
        .route("/orders/:id/submit", post(submit_order))
        .route("/orders/:id/receive", post(receive_goods))
        .route("/orders/:id/items", get(get_order_items))
}