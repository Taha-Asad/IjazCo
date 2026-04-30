// src/handlers/sales.rs
// Sales invoice management endpoints
// Invoice creation, approval, payment recording, and management

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
    models::sales::{
        ApproveInvoiceRequest, CreateSalesInvoiceRequest, InvoiceStatus, RecordPaymentRequest,
        SalesInvoice, SalesInvoiceWithItems, SalesSummary, UpdateSalesInvoiceRequest,
    },
    utils::{
        error::{AppError, Result},
        response::{created, no_content, paginated, success},
    },
    AppState,
};

use super::users::PaginationParams;

// ===== INVOICE QUERY PARAMETERS =====
#[derive(Debug, Deserialize, ToSchema)]
pub struct InvoiceQueryParams {
    pub status: Option<InvoiceStatus>,
    pub customer_id: Option<Uuid>,
    pub branch_id: Option<Uuid>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

// ===== LIST INVOICES ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/sales/invoices",
    tag = "sales",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("status" = Option<InvoiceStatus>, Query, description = "Filter by status"),
        ("customer_id" = Option<Uuid>, Query, description = "Filter by customer"),
        ("branch_id" = Option<Uuid>, Query, description = "Filter by branch"),
        ("start_date" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("end_date" = Option<String>, Query, description = "End date (YYYY-MM-DD)"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page"),
    ),
    responses(
        (status = 200, description = "List of invoices", body = Vec<SalesInvoice>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_invoices(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Query(params): Query<InvoiceQueryParams>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        company_id = %auth_user.company_id,
        status = ?params.status,
        "Listing sales invoices"
    );
    
    // Fetch invoices based on filters
    let (invoices, total_count) = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            let invoices = SalesInvoice::list_by_company_pg(
                pool,
                auth_user.company_id,
                params.status,
                params.pagination.limit(),
                params.pagination.offset(),
            ).await?;
            
            let total_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM sales_invoices WHERE company_id = $1"
            )
            .bind(auth_user.company_id)
            .fetch_one(pool)
            .await?;
            
            (invoices, total_count)
        }
        DbPool::Sqlite(pool) => {
            let invoices = SalesInvoice::list_by_company_sqlite(
                pool,
                auth_user.company_id,
                params.status,
                params.pagination.limit(),
                params.pagination.offset(),
            ).await?;
            
            let total_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM sales_invoices WHERE company_id = ?"
            )
            .bind(auth_user.company_id)
            .fetch_one(pool)
            .await?;
            
            (invoices, total_count)
        }
    };
    
    tracing::debug!(
        count = invoices.len(),
        total = total_count,
        "Invoices retrieved successfully"
    );
    
    Ok(paginated(
        invoices,
        params.pagination.page,
        params.pagination.per_page,
        total_count,
    ))
}

// ===== GET INVOICE ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/sales/invoices/{id}",
    tag = "sales",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Invoice ID")
    ),
    responses(
        (status = 200, description = "Invoice found", body = SalesInvoiceWithItems),
        (status = 404, description = "Invoice not found"),
        (status = 403, description = "Access denied")
    )
)]
pub async fn get_invoice(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<SalesInvoiceWithItems>> {
    tracing::debug!(
        user_id = %auth_user.id,
        invoice_id = %id,
        "Fetching invoice"
    );
    
    // Fetch invoice with items
    let invoice = match state.db.as_ref() {
        DbPool::Postgres(pool) => SalesInvoice::get_with_items_pg(pool, id).await?,
        DbPool::Sqlite(pool) => SalesInvoice::get_with_items_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Invoice not found".to_string()))?;
    
    // Verify invoice belongs to same company
    verify_company_access(&auth_user, invoice.invoice.company_id)?;
    
    tracing::info!(
        invoice_id = %id,
        invoice_number = %invoice.invoice.invoice_number,
        "Invoice retrieved successfully"
    );
    
    Ok(Json(invoice))
}

// ===== CREATE INVOICE ENDPOINT =====
#[utoipa::path(
    post,
    path = "/api/v1/sales/invoices",
    tag = "sales",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateSalesInvoiceRequest,
    responses(
        (status = 201, description = "Invoice created successfully", body = SalesInvoiceWithItems),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Insufficient stock"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn create_invoice(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(mut payload): Json<CreateSalesInvoiceRequest>,
) -> Result<impl axum::response::IntoResponse> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        customer_id = %payload.customer_id,
        items_count = payload.items.len(),
        "Creating sales invoice"
    );
    
    // Force company_id to match authenticated user's company
    payload.company_id = auth_user.company_id;
    
    // Verify customer belongs to same company
    let customer = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            crate::models::customer::Customer::find_by_id_pg(pool, payload.customer_id).await?
        }
        DbPool::Sqlite(pool) => {
            crate::models::customer::Customer::find_by_id_sqlite(pool, payload.customer_id).await?
        }
    }
    .ok_or_else(|| AppError::NotFound("Customer not found".to_string()))?;
    
    verify_company_access(&auth_user, customer.company_id)?;
    
    // Create invoice
    let invoice = match state.db.as_ref() {
        DbPool::Postgres(pool) => SalesInvoice::create_pg(pool, payload, auth_user.id).await?,
        DbPool::Sqlite(pool) => SalesInvoice::create_sqlite(pool, payload, auth_user.id).await?,
    };
    
    tracing::info!(
        invoice_id = %invoice.invoice.id,
        invoice_number = %invoice.invoice.invoice_number,
        total_amount = %invoice.invoice.total_amount,
        "Invoice created successfully"
    );
    
    Ok(created("Invoice created successfully", invoice))
}

// ===== UPDATE INVOICE ENDPOINT =====
#[utoipa::path(
    put,
    path = "/api/v1/sales/invoices/{id}",
    tag = "sales",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Invoice ID")
    ),
    request_body = UpdateSalesInvoiceRequest,
    responses(
        (status = 200, description = "Invoice updated successfully", body = SalesInvoice),
        (status = 404, description = "Invoice not found"),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Invoice cannot be modified in current status"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn update_invoice(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateSalesInvoiceRequest>,
) -> Result<Json<SalesInvoice>> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        invoice_id = %id,
        "Updating invoice"
    );
    
    // Fetch existing invoice
    let existing_invoice = match state.db.as_ref() {
        DbPool::Postgres(pool) => SalesInvoice::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => SalesInvoice::find_by_id_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Invoice not found".to_string()))?;
    
    // Verify invoice belongs to same company
    verify_company_access(&auth_user, existing_invoice.company_id)?;
    
    // Check if invoice can be updated (only draft invoices)
    if existing_invoice.status != InvoiceStatus::Draft {
        return Err(AppError::InvalidStatus {
            entity: "Invoice".to_string(),
            current_status: format!("{:?}", existing_invoice.status),
            allowed_statuses: vec!["Draft".to_string()],
        });
    }
    
    // Update invoice - simplified update for now
    let updated_invoice = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            SalesInvoice::update_pg(pool, id, payload, auth_user.id).await?
        }
        DbPool::Sqlite(pool) => {
            SalesInvoice::update_sqlite(pool, id, payload, auth_user.id).await?
        }
    };
    
    tracing::info!(
        invoice_id = %id,
        "Invoice updated successfully"
    );
    
    Ok(Json(updated_invoice))
}

// ===== DELETE INVOICE ENDPOINT =====
#[utoipa::path(
    delete,
    path = "/api/v1/sales/invoices/{id}",
    tag = "sales",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Invoice ID")
    ),
    responses(
        (status = 204, description = "Invoice deleted successfully"),
        (status = 404, description = "Invoice not found"),
        (status = 409, description = "Invoice cannot be deleted in current status"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn delete_invoice(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        invoice_id = %id,
        "Deleting invoice"
    );
    
    // Fetch invoice to verify company ownership
    let invoice = match state.db.as_ref() {
        DbPool::Postgres(pool) => SalesInvoice::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => SalesInvoice::find_by_id_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Invoice not found".to_string()))?;
    
    verify_company_access(&auth_user, invoice.company_id)?;
    
    // Delete invoice
    match state.db.as_ref() {
        DbPool::Postgres(pool) => SalesInvoice::delete_pg(pool, id).await?,
        DbPool::Sqlite(pool) => SalesInvoice::delete_sqlite(pool, id).await?,
    };
    
    tracing::info!(
        invoice_id = %id,
        invoice_number = %invoice.invoice_number,
        "Invoice deleted successfully"
    );
    
    Ok(no_content())
}

// ===== APPROVE INVOICE ENDPOINT =====
#[utoipa::path(
    post,
    path = "/api/v1/sales/invoices/{id}/approve",
    tag = "sales",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Invoice ID")
    ),
    request_body = ApproveInvoiceRequest,
    responses(
        (status = 200, description = "Invoice approved successfully", body = SalesInvoice),
        (status = 404, description = "Invoice not found"),
        (status = 409, description = "Insufficient stock or invalid status"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn approve_invoice(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(_payload): Json<ApproveInvoiceRequest>,
) -> Result<Json<SalesInvoice>> {
    tracing::info!(
        user_id = %auth_user.id,
        invoice_id = %id,
        "Approving invoice"
    );
    
    // Fetch invoice
    let invoice = match state.db.as_ref() {
        DbPool::Postgres(pool) => SalesInvoice::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => SalesInvoice::find_by_id_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Invoice not found".to_string()))?;
    
    verify_company_access(&auth_user, invoice.company_id)?;
    
    // Approve invoice
    let approved_invoice = match state.db.as_ref() {
        DbPool::Postgres(pool) => SalesInvoice::approve_pg(pool, id, auth_user.id).await?,
        DbPool::Sqlite(pool) => SalesInvoice::approve_sqlite(pool, id, auth_user.id).await?,
    };
    
    tracing::info!(
        invoice_id = %id,
        invoice_number = %approved_invoice.invoice_number,
        "Invoice approved and stock deducted"
    );
    
    Ok(Json(approved_invoice))
}

// ===== RECORD PAYMENT ENDPOINT =====
#[utoipa::path(
    post,
    path = "/api/v1/sales/invoices/{id}/payment",
    tag = "sales",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Invoice ID")
    ),
    request_body = RecordPaymentRequest,
    responses(
        (status = 200, description = "Payment recorded successfully", body = SalesInvoice),
        (status = 404, description = "Invoice not found"),
        (status = 400, description = "Validation error"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn record_payment(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<RecordPaymentRequest>,
) -> Result<Json<SalesInvoice>> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        invoice_id = %id,
        amount = %payload.amount,
        "Recording payment"
    );
    
    // Fetch invoice
    let invoice = match state.db.as_ref() {
        DbPool::Postgres(pool) => SalesInvoice::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => SalesInvoice::find_by_id_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Invoice not found".to_string()))?;
    
    verify_company_access(&auth_user, invoice.company_id)?;
    
    // Validate payment amount doesn't exceed balance
    if payload.amount > invoice.balance_due {
        return Err(AppError::ValidationError(
            format!(
                "Payment amount ({}) exceeds balance due ({})",
                payload.amount, invoice.balance_due
            )
        ));
    }
    
    // Record payment
    let updated_invoice = match state.db.as_ref() {
        DbPool::Postgres(pool) => SalesInvoice::record_payment_pg(pool, id, payload).await?,
        DbPool::Sqlite(pool) => SalesInvoice::record_payment_sqlite(pool, id, payload).await?,
    };
    
    tracing::info!(
        invoice_id = %id,
        new_status = ?updated_invoice.status,
        balance_due = %updated_invoice.balance_due,
        "Payment recorded successfully"
    );
    
    Ok(Json(updated_invoice))
}

// ===== GET INVOICE ITEMS ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/sales/invoices/{id}/items",
    tag = "sales",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Invoice ID")
    ),
    responses(
        (status = 200, description = "Invoice items", body = Vec<SalesInvoiceItem>),
        (status = 404, description = "Invoice not found"),
        (status = 403, description = "Access denied")
    )
)]
pub async fn get_invoice_items(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<crate::models::sales::SalesInvoiceItem>>> {
    tracing::debug!(
        user_id = %auth_user.id,
        invoice_id = %id,
        "Fetching invoice items"
    );
    
    // Fetch invoice to verify access
    let invoice = match state.db.as_ref() {
        DbPool::Postgres(pool) => SalesInvoice::find_by_id_pg(pool, id).await?,
        DbPool::Sqlite(pool) => SalesInvoice::find_by_id_sqlite(pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Invoice not found".to_string()))?;
    
    verify_company_access(&auth_user, invoice.company_id)?;
    
    // Fetch items
    let items = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            sqlx::query_as::<sqlx::Postgres, crate::models::sales::SalesInvoiceItem>(
                "SELECT * FROM sales_invoice_items WHERE invoice_id = $1 ORDER BY created_at"
            )
            .bind(id)
            .fetch_all(pool)
            .await?
        }
        DbPool::Sqlite(pool) => {
            let items_sqlite = sqlx::query_as::<sqlx::Sqlite, crate::models::sales::SalesInvoiceItemSqlite>(
                "SELECT * FROM sales_invoice_items WHERE invoice_id = ? ORDER BY created_at"
            )
            .bind(id)
            .fetch_all(pool)
            .await?;
            
            items_sqlite.into_iter().map(crate::models::sales::SalesInvoiceItem::from).collect()
        }
    };
    
    tracing::debug!(
        count = items.len(),
        "Invoice items retrieved"
    );
    
    Ok(Json(items))
}

// ===== SALES SUMMARY ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/sales/summary",
    tag = "sales",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("start_date" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("end_date" = Option<String>, Query, description = "End date (YYYY-MM-DD)"),
    ),
    responses(
        (status = 200, description = "Sales summary", body = SalesSummary),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn sales_summary(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Query(params): Query<SummaryParams>,
) -> Result<Json<SalesSummary>> {
    tracing::info!(
        user_id = %auth_user.id,
        company_id = %auth_user.company_id,
        "Fetching sales summary"
    );
    
    let summary = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            SalesInvoice::get_sales_summary_pg(
                pool,
                auth_user.company_id,
                params.start_date,
                params.end_date,
            ).await?
        }
        DbPool::Sqlite(pool) => {
            SalesInvoice::get_sales_summary_sqlite(
                pool,
                auth_user.company_id,
                params.start_date,
                params.end_date,
            ).await?
        }
    };
    
    tracing::info!(
        total_sales = %summary.total_sales,
        total_invoices = summary.total_invoices,
        "Sales summary retrieved"
    );
    
    Ok(Json(summary))
}

// ===== SUMMARY QUERY PARAMETERS =====
#[derive(Debug, Deserialize, ToSchema)]
pub struct SummaryParams {
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
}

pub fn sales_router() -> axum::Router<Arc<AppState>> {
    use axum::routing::{delete, get, post, put};

    axum::Router::new()
        .route("/invoices", get(list_invoices).post(create_invoice))
        .route("/summary", get(sales_summary))
        .route("/invoices/:id", get(get_invoice).put(update_invoice).delete(delete_invoice))
        .route("/invoices/:id/approve", post(approve_invoice))
        .route("/invoices/:id/payment", post(record_payment))
        .route("/invoices/:id/items", get(get_invoice_items))
}