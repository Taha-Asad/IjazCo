// src/handlers/reports.rs
// Report generation and export endpoints
// PDF reports, Excel exports, and data analytics

use axum::{
    extract::{Query, State},
    Json,
};
use sqlx::types::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres};
use std::sync::Arc;
use utoipa::ToSchema;
use rust_decimal::prelude::FromPrimitive; // Must import this for from_f64
use crate::{
    AppState, config::DbPool, middleware::auth::AuthUser, utils::{
        error::{Result},
        response::success,
    }
};

// ===== REPORT QUERY PARAMETERS =====
#[derive(Debug, Deserialize, ToSchema)]
pub struct ReportQueryParams {
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub branch_id: Option<uuid::Uuid>,
    pub format: Option<String>, // pdf, excel, csv
}

// ===== SALES REPORT ENDPOINT =====
// Generate comprehensive sales report
#[utoipa::path(
    get,
    path = "/api/v1/reports/sales",
    tag = "reports",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("start_date" = Option<String>, Query, description = "Start date"),
        ("end_date" = Option<String>, Query, description = "End date"),
        ("branch_id" = Option<uuid::Uuid>, Query, description = "Filter by branch"),
        ("format" = Option<String>, Query, description = "Report format (json, pdf, excel)"),
    ),
    responses(
        (status = 200, description = "Sales report data"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn sales_report(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Query(params): Query<ReportQueryParams>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        format = ?params.format,
        "Generating sales report"
    );
    
    let end_date = params.end_date.unwrap_or_else(|| chrono::Utc::now().date_naive());
    let start_date = params.start_date.unwrap_or_else(|| end_date - chrono::Duration::days(30));
    
    // We branch the logic to handle dialect differences and trait requirements
    let sales_data = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            let sales_report_pg: Vec<SalesReportRow> = sqlx::query_as::<Postgres, SalesReportRow>(
                r#"
                SELECT 
                    si.invoice_number,
                    si.invoice_date,
                    c.name as customer_name,
                    b.name as branch_name,
                    si.subtotal,
                    si.tax_amount,
                    si.total_amount,
                    si.paid_amount,
                    si.balance_due,
                    si.status::text as status
                FROM sales_invoices si
                JOIN customers c ON si.customer_id = c.id
                JOIN branches b ON si.branch_id = b.id
                WHERE si.company_id = $1
                  AND si.invoice_date BETWEEN $2 AND $3
                  AND ($4::uuid IS NULL OR si.branch_id = $4)
                ORDER BY si.invoice_date DESC, si.invoice_number
                "#
            )
            .bind(auth_user.company_id)
            .bind(start_date)
            .bind(end_date)
            .bind(params.branch_id)
            .fetch_all(pool)
            .await?;
            
            sales_report_pg // Return the data, not Ok()
        }
        DbPool::Sqlite(pool) => {
            // SQLite logic: Uses '?' and handles NULL check without Postgres casting
            let sales_report_sqlite: Vec<SalesReportRowSqlite> = sqlx::query_as(
                r#"
                SELECT 
                    si.invoice_number,
                    si.invoice_date,
                    c.name as customer_name,
                    b.name as branch_name,
                    si.subtotal,
                    si.tax_amount,
                    si.total_amount,
                    si.paid_amount,
                    si.balance_due,
                    si.status as status
                FROM sales_invoices si
                JOIN customers c ON si.customer_id = c.id
                JOIN branches b ON si.branch_id = b.id
                WHERE si.company_id = ?
                  AND si.invoice_date BETWEEN ? AND ?
                  AND (? IS NULL OR si.branch_id = ?)
                ORDER BY si.invoice_date DESC, si.invoice_number
                "#
            )
            .bind(auth_user.company_id)
            .bind(start_date)
            .bind(end_date)
            .bind(&params.branch_id)
            .bind(&params.branch_id)
            .fetch_all(pool)
            .await?;
            
            // Convert to the main SalesReportRow type with all fields
            let sales_report: Vec<SalesReportRow> = sales_report_sqlite
                .into_iter()
                .map(|item| SalesReportRow {
                    invoice_number: item.invoice_number,
                    invoice_date: item.invoice_date,
                    customer_name: item.customer_name,
                    branch_name: item.branch_name,
                    subtotal: Decimal::from_f64_retain(item.subtotal).unwrap_or(Decimal::ZERO),
                    tax_amount: Decimal::from_f64_retain(item.tax_amount).unwrap_or(Decimal::ZERO),
                    total_amount: Decimal::from_f64_retain(item.total_amount).unwrap_or(Decimal::ZERO),
                    paid_amount: Decimal::from_f64_retain(item.paid_amount).unwrap_or(Decimal::ZERO),
                    balance_due: Decimal::from_f64_retain(item.balance_due).unwrap_or(Decimal::ZERO),
                    status: item.status,
                })
                .collect();
            
            sales_report // Return the data, not Ok()
        }
    };
    
    // Format based on requested format
    match params.format.as_deref() {
        Some("pdf") => {
            tracing::warn!("PDF export not yet implemented");
            Ok(success("PDF generation pending", sales_data))
        },
        Some("excel") | Some("csv") => {
            tracing::warn!("Excel/CSV export not yet implemented");
            Ok(success("Excel/CSV export pending", sales_data))
        },
        _ => {
            Ok(success("Sales report generated", sales_data))
        }
    }
}

// ===== SALES REPORT ROW =====


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct SalesReportRow {

    pub invoice_number: String,
    pub invoice_date: chrono::NaiveDate,
    pub customer_name: String,
    pub branch_name: String,
    pub subtotal: Decimal,
    pub tax_amount: Decimal,
    pub total_amount: Decimal,
    pub paid_amount: Decimal,
    pub balance_due: Decimal,
    pub status: String,
}
#[derive(Debug, sqlx::FromRow)]
pub struct SalesReportRowSqlite {

    pub invoice_number: String,
    pub invoice_date: chrono::NaiveDate,
    pub customer_name: String,
    pub branch_name: String,
    pub subtotal: f64,
    pub tax_amount: f64,
    pub total_amount: f64,
    pub paid_amount: f64,
    pub balance_due: f64,
    pub status: String,
}

// ===== INVENTORY REPORT ENDPOINT =====
// Generate inventory report with stock levels
#[utoipa::path(
    get,
    path = "/api/v1/reports/inventory",
    tag = "reports",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("branch_id" = Option<uuid::Uuid>, Query, description = "Filter by branch"),
        ("category_id" = Option<uuid::Uuid>, Query, description = "Filter by category"),
        ("format" = Option<String>, Query, description = "Report format"),
    ),
    responses(
        (status = 200, description = "Inventory report data"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn inventory_report(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Query(params): Query<InventoryReportParams>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        "Generating inventory report"
    );

    // Helper function for f64 to Decimal conversion
    fn to_decimal(value: f64) -> Decimal {
        Decimal::from_f64_retain(value).unwrap_or(Decimal::ZERO)
    }

    let inventory_data = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            sqlx::query_as::<Postgres, InventoryReportRow>(
                r#"
                SELECT 
                    i.sku, i.name as item_name, c.name as category_name, b.name as branch_name,
                    s.quantity_on_hand, s.quantity_allocated, s.quantity_available,
                    i.cost_price, i.selling_price,
                    (s.quantity_on_hand * i.cost_price) as total_cost_value,
                    (s.quantity_on_hand * i.selling_price) as total_selling_value,
                    i.reorder_level,
                    CASE 
                        WHEN s.quantity_available < i.reorder_level THEN 'Low Stock'
                        WHEN s.quantity_on_hand = 0 THEN 'Out of Stock'
                        ELSE 'Normal'
                    END as stock_status
                FROM inventory_items i
                LEFT JOIN categories c ON i.category_id = c.id
                LEFT JOIN stock s ON s.item_id = i.id
                LEFT JOIN branches b ON s.branch_id = b.id
                WHERE i.company_id = $1 AND i.is_active = true
                  AND ($2::uuid IS NULL OR s.branch_id = $2)
                  AND ($3::uuid IS NULL OR i.category_id = $3)
                ORDER BY i.name, b.name
                "#
            )
            .bind(auth_user.company_id)
            .bind(params.branch_id)
            .bind(params.category_id)
            .fetch_all(pool)
            .await?
        }
        DbPool::Sqlite(pool) => {
            #[derive(sqlx::FromRow)]
            struct InventoryReportRowSqlite {
                sku: Option<String>,
                item_name: String,
                category_name: Option<String>,
                branch_name: Option<String>,
                quantity_on_hand: i32,
                quantity_allocated: i32,
                quantity_available: i32,
                cost_price: f64,
                selling_price: f64,
                total_cost_value: f64,
                total_selling_value: f64,
                reorder_level: i32,
                stock_status: String,
            }
            
            let rows_sqlite: Vec<InventoryReportRowSqlite> = sqlx::query_as(
                r#"
                SELECT 
                    i.sku, i.name as item_name, c.name as category_name, b.name as branch_name,
                    s.quantity_on_hand, s.quantity_allocated, s.quantity_available,
                    i.cost_price, i.selling_price,
                    (s.quantity_on_hand * i.cost_price) as total_cost_value,
                    (s.quantity_on_hand * i.selling_price) as total_selling_value,
                    i.reorder_level,
                    CASE 
                        WHEN s.quantity_available < i.reorder_level THEN 'Low Stock'
                        WHEN s.quantity_on_hand = 0 THEN 'Out of Stock'
                        ELSE 'Normal'
                    END as stock_status
                FROM inventory_items i
                LEFT JOIN categories c ON i.category_id = c.id
                LEFT JOIN stock s ON s.item_id = i.id
                LEFT JOIN branches b ON s.branch_id = b.id
                WHERE i.company_id = ? AND i.is_active = 1
                  AND (? IS NULL OR s.branch_id = ?)
                  AND (? IS NULL OR i.category_id = ?)
                ORDER BY i.name, b.name
                "#
            )
            .bind(auth_user.company_id)
            .bind(params.branch_id)
            .bind(params.branch_id)
            .bind(params.category_id)
            .bind(params.category_id)
            .fetch_all(pool)
            .await?;
            
            rows_sqlite
                .into_iter()
                .map(|row| InventoryReportRow {
                    sku: row.sku.unwrap_or_default(),
                    item_name: row.item_name,
                    category_name: row.category_name,
                    branch_name: row.branch_name,
                    quantity_on_hand: Some(row.quantity_on_hand),
                    quantity_allocated: Some(row.quantity_allocated),
                    quantity_available: Some(row.quantity_available),
                    cost_price: Decimal::from_f64_retain(row.cost_price).unwrap_or(Decimal::ZERO),
                    selling_price: Decimal::from_f64_retain(row.selling_price).unwrap_or(Decimal::ZERO),
total_cost_value: Decimal::from_f64(row.total_cost_value),
total_selling_value: Decimal::from_f64(row.total_selling_value),
                    reorder_level: row.reorder_level,
                    stock_status: row.stock_status,
                })
                .collect()
        }
    };

    Ok(success("Inventory report generated", inventory_data))
}

// ===== INVENTORY REPORT PARAMETERS =====
#[derive(Debug, Deserialize, ToSchema)]
pub struct InventoryReportParams {
    pub branch_id: Option<uuid::Uuid>,
    pub category_id: Option<uuid::Uuid>,
    pub format: Option<String>,
}

// ===== INVENTORY REPORT ROW =====
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct InventoryReportRow {

    pub sku: String,
    pub item_name: String,
    pub category_name: Option<String>,
    pub branch_name: Option<String>,
    pub quantity_on_hand: Option<i32>,
    pub quantity_allocated: Option<i32>,
    pub quantity_available: Option<i32>,
    pub cost_price: Decimal,
    pub selling_price: Decimal,
    pub total_cost_value: Option<Decimal>,
    pub total_selling_value: Option<Decimal>,
    pub reorder_level: i32,
    pub stock_status: String,
}

// ===== EXPORT PDF ENDPOINT =====
// Export any report to PDF format
#[utoipa::path(
    post,
    path = "/api/v1/reports/export/pdf",
    tag = "reports",
    security(
        ("bearer_auth" = [])
    ),
    request_body = ExportPdfRequest,
    responses(
        (status = 200, description = "PDF generated successfully"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn export_pdf(
    State(_state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(payload): Json<ExportPdfRequest>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        report_type = %payload.report_type,
        "Exporting PDF"
    );
    
    // TODO: Implement actual PDF generation
    // Options: printpdf, wkhtmltopdf-rs, or call external service
    
    tracing::warn!("PDF export not yet fully implemented");
    
    Ok(success(
        "PDF export initiated (implementation pending)",
        serde_json::json!({
            "report_type": payload.report_type,
            "status": "pending",
            "message": "PDF generation will be implemented in phase 2"
        })
    ))
}

// ===== EXPORT PDF REQUEST =====
#[derive(Debug, Deserialize, ToSchema)]
pub struct ExportPdfRequest {
    pub report_type: String,
    pub data: serde_json::Value,
}

pub fn reports_router() -> axum::Router<Arc<AppState>> {
    use axum::routing::{get, post};

    axum::Router::new()
        .route("/sales", get(sales_report))
        .route("/inventory", get(inventory_report))
        .route("/export/pdf", post(export_pdf))
}