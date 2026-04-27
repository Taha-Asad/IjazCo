// src/handlers/dashboard.rs
// Dashboard and analytics endpoints
// Provides real-time statistics, charts data, and KPIs

use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Sqlite};
use uuid::Uuid;
use std::sync::Arc;
use sqlx::types::Decimal;
use utoipa::ToSchema;

use crate::{
    middleware::auth::AuthUser,config::DbPool,
    utils::{
        error::Result,
        response::success,
    },
    AppState,
};

// ===== DASHBOARD QUERY PARAMETERS =====
#[derive(Debug, Deserialize, ToSchema)]
pub struct DashboardQueryParams {
    // Date range for statistics
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    
    // Branch filter (optional)
    pub branch_id: Option<uuid::Uuid>,
}

// ===== DASHBOARD STATISTICS RESPONSE =====
#[derive(Debug, Serialize, ToSchema)]
pub struct DashboardStats {
    // Overview metrics
    pub overview: OverviewMetrics,
    
    // Sales metrics
    pub sales: SalesMetrics,
    
    // Inventory metrics
    pub inventory: InventoryMetrics,
    
    // Purchase metrics
    pub purchases: PurchaseMetrics,
    
    // Recent activities
    pub recent_activities: Vec<ActivityItem>,
    
    // Low stock alerts
    pub low_stock_count: i64,
    
    // Pending approvals
    pub pending_approvals: PendingApprovals,
}

// ===== OVERVIEW METRICS =====
#[derive(Debug, Serialize, ToSchema)]
pub struct OverviewMetrics {
    // Total revenue for period
    #[schema(example = 150000.00)]
    pub total_revenue: Decimal,
    
    // Revenue change percentage vs previous period
    #[schema(example = 15.5)]
    pub revenue_change_percent: Decimal,
    
    // Total orders
    pub total_orders: i64,
    
    // Orders change percentage
    #[schema(example = 10.2)]
    pub orders_change_percent: Decimal,
    
    // Total customers
    pub total_customers: i64,
    
    // New customers in period
    pub new_customers: i64,
    
    // Total inventory value
    #[schema(example = 500000.00)]
    pub inventory_value: Decimal,
}

// ===== SALES METRICS =====
#[derive(Debug, Serialize, ToSchema)]
pub struct SalesMetrics {
    // Total sales amount
    #[schema(value_type = f64, example = 150000.00)]
    pub total_sales: Decimal,
    
    // Number of invoices
    pub invoice_count: i64,
    
    // Average order value
    #[schema(value_type = f64, example = 5000.00)]
    pub average_order_value: Decimal,
    
    // Outstanding amount (unpaid)
    #[schema(value_type = f64, example = 25000.00)]
    pub outstanding_amount: Decimal,
    
    // Top selling items
    pub top_items: Vec<TopItem>,
    
    // Sales by status
    pub by_status: StatusBreakdown,
}

// ===== INVENTORY METRICS =====
#[derive(Debug, Serialize, ToSchema)]
pub struct InventoryMetrics {
    // Total items
    pub total_items: i64,
    
    // Total stock quantity
    pub total_quantity: i64,
    
    // Inventory value at cost
    #[schema(value_type = f64, example = 500000.00)]
    pub total_cost_value: Decimal,
    
    // Inventory value at selling price
    #[schema(value_type = f64, example = 750000.00)]
    pub total_selling_value: Decimal,
    
    // Low stock items count
    pub low_stock_items: i64,
    
    // Out of stock items count
    pub out_of_stock_items: i64,
    
    // Categories breakdown
    pub by_category: Vec<CategoryBreakdown>,
}

// ===== PURCHASE METRICS =====
#[derive(Debug, Serialize, ToSchema)]
pub struct PurchaseMetrics {
    // Total purchase amount
    #[schema(value_type = f64, example = 300000.00)]
    pub total_purchases: Decimal,
    
    // Number of purchase orders
    pub po_count: i64,
    
    // Pending POs
    pub pending_pos: i64,
    
    // Average PO value
    #[schema(value_type = f64, example = 15000.00)]
    pub average_po_value: Decimal,
    
    // Top suppliers
    pub top_suppliers: Vec<TopSupplier>,
}

// ===== TOP ITEM =====
#[derive(Debug, Serialize, ToSchema , sqlx::FromRow)]
pub struct TopItem {
    pub item_id: uuid::Uuid,
    pub item_name: String,
    pub sku: String,
    pub quantity_sold: i64,
    #[schema(value_type = f64)]
    pub total_revenue: Decimal,
}

// ===== STATUS BREAKDOWN =====
#[derive(Debug, Serialize, ToSchema)]
pub struct StatusBreakdown {
    pub draft: i64,
    pub pending: i64,
    pub approved: i64,
    pub paid: i64,
    pub cancelled: i64,
}

// ===== CATEGORY BREAKDOWN =====
#[derive(Debug, Serialize, ToSchema, sqlx::FromRow)]
pub struct CategoryBreakdown {
    pub category_id: uuid::Uuid,
    pub category_name: String,
    pub item_count: i64,
    #[schema(value_type = f64)]
    pub total_value: Decimal,
}

// ===== TOP SUPPLIER =====
#[derive(Debug, Serialize, ToSchema, sqlx::FromRow)]
pub struct TopSupplier {
    pub supplier_id: uuid::Uuid,
    pub supplier_name: String,
    pub po_count: i64,
    #[schema(value_type = f64)]
    pub total_amount: Decimal,
}

// ===== ACTIVITY ITEM =====
#[derive(Debug, Serialize, ToSchema)]
pub struct ActivityItem {
    pub id: uuid::Uuid,
    pub activity_type: String,
    pub description: String,
    pub user_name: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

// ===== PENDING APPROVALS =====
#[derive(Debug, Serialize, ToSchema)]
pub struct PendingApprovals {
    pub sales_invoices: i64,
    pub purchase_orders: i64,
}

// ===== GET DASHBOARD STATISTICS ENDPOINT =====
// Get comprehensive dashboard statistics
#[utoipa::path(
    get,
    path = "/api/v1/dashboard/stats",
    tag = "dashboard",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("start_date" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("end_date" = Option<String>, Query, description = "End date (YYYY-MM-DD)"),
        ("branch_id" = Option<uuid::Uuid>, Query, description = "Filter by branch"),
    ),
    responses(
        (status = 200, description = "Dashboard statistics", body = DashboardStats),
        (status = 401, description = "Unauthorized")
    )
)]

pub async fn get_dashboard_stats(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Query(params): Query<DashboardQueryParams>,
) -> Result<Json<DashboardStats>> {
    tracing::info!(
        user_id = %auth_user.id,
        company_id = %auth_user.company_id,
        "Fetching dashboard statistics"
    );
    
    let end_date = params.end_date.unwrap_or_else(|| chrono::Utc::now().date_naive());
    let start_date = params.start_date.unwrap_or_else(|| end_date - chrono::Duration::days(30));
    let prev_start = start_date - chrono::Duration::days(30);
    let prev_end = end_date - chrono::Duration::days(30);
    
    match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            // Fetch Current Period
            let (total_revenue, total_orders): (Option<Decimal>, i64) = sqlx::query_as::<Postgres, (Option<Decimal>, i64)>(
                "SELECT COALESCE(SUM(total_amount), 0), COUNT(*) FROM sales_invoices 
                 WHERE company_id = $1 AND invoice_date BETWEEN $2 AND $3 AND status != 'cancelled'"
            )
            .bind(auth_user.company_id).bind(start_date).bind(end_date)
            .fetch_one(pool).await?;

            // Fetch Previous Period
            let (prev_revenue, prev_orders): (Option<Decimal>, i64) = sqlx::query_as::<Postgres, (Option<Decimal>, i64)>(
                "SELECT COALESCE(SUM(total_amount), 0), COUNT(*) FROM sales_invoices 
                 WHERE company_id = $1 AND invoice_date BETWEEN $2 AND $3 AND status != 'cancelled'"
            )
            .bind(auth_user.company_id).bind(prev_start).bind(prev_end)
            .fetch_one(pool).await?;

            let total_customers = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM customers WHERE company_id = $1 AND is_active = true"
            ).bind(auth_user.company_id).fetch_one(pool).await?;

            let new_customers = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM customers WHERE company_id = $1 AND created_at >= $2 AND created_at <= $3"
            ).bind(auth_user.company_id).bind(start_date).bind(end_date).fetch_one(pool).await?;

            // Inventory value
            let inventory_value: Option<Decimal> = sqlx::query_scalar(
                r#"
                SELECT COALESCE(SUM(s.quantity_on_hand * i.cost_price), 0)
                FROM stock s
                JOIN inventory_items i ON s.item_id = i.id
                WHERE s.company_id = $1
                "#
            )
            .bind(auth_user.company_id)
            .fetch_one(pool)
            .await?;

            let total_revenue_val = total_revenue.unwrap_or(Decimal::ZERO);
            let prev_revenue_val = prev_revenue.unwrap_or(Decimal::ZERO);
            let revenue_change = calculate_percentage_change(total_revenue_val, prev_revenue_val);
            
            let total_orders_val = total_orders;
            let prev_orders_val = prev_orders;
            let orders_change = calculate_percentage_change(Decimal::from(total_orders_val), Decimal::from(prev_orders_val));

            let overview = OverviewMetrics {
                total_revenue: total_revenue_val,
                revenue_change_percent: revenue_change,
                total_orders: total_orders_val,
                orders_change_percent: orders_change,
                total_customers,
                new_customers,
                inventory_value: inventory_value.unwrap_or(Decimal::ZERO),
            };
            
            // ===== SALES METRICS =====
            let (outstanding_amount,): (Option<Decimal>,) = sqlx::query_as(
                r#"
                SELECT COALESCE(SUM(balance_due), 0)
                FROM sales_invoices
                WHERE company_id = $1 AND status IN ('approved', 'pending')
                "#
            )
            .bind(auth_user.company_id)
            .fetch_one(pool)
            .await?;
            
            let average_order_value = if total_orders_val > 0 {
                total_revenue_val / Decimal::from(total_orders_val)
            } else {
                Decimal::ZERO
            };
            
            // Top selling items
            let top_items: Vec<TopItem> = sqlx::query_as(
                r#"
                SELECT 
                    i.id as item_id,
                    i.name as item_name,
                    i.sku,
                    SUM(sii.quantity) as quantity_sold,
                    SUM(sii.line_total) as total_revenue
                FROM sales_invoice_items sii
                JOIN sales_invoices si ON sii.invoice_id = si.id
                JOIN inventory_items i ON sii.item_id = i.id
                WHERE si.company_id = $1
                  AND si.invoice_date BETWEEN $2 AND $3
                  AND si.status != 'cancelled'
                GROUP BY i.id, i.name, i.sku
                ORDER BY quantity_sold DESC
                LIMIT 5
                "#
            )
            .bind(auth_user.company_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(pool)
            .await?;
            
            // Sales by status
            let status_counts: Vec<(String, i64)> = sqlx::query_as(
                r#"
                SELECT status::text, COUNT(*)
                FROM sales_invoices
                WHERE company_id = $1 AND invoice_date BETWEEN $2 AND $3
                GROUP BY status
                "#
            )
            .bind(auth_user.company_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(pool)
            .await?;
            
            let mut status_breakdown = StatusBreakdown {
                draft: 0,
                pending: 0,
                approved: 0,
                paid: 0,
                cancelled: 0,
            };
            
            for (status, count) in status_counts {
                match status.as_str() {
                    "draft" => status_breakdown.draft = count,
                    "pending" => status_breakdown.pending = count,
                    "approved" => status_breakdown.approved = count,
                    "paid" => status_breakdown.paid = count,
                    "cancelled" => status_breakdown.cancelled = count,
                    _ => {}
                }
            }
            
            let sales = SalesMetrics {
                total_sales: total_revenue_val,
                invoice_count: total_orders_val,
                average_order_value,
                outstanding_amount: outstanding_amount.unwrap_or(Decimal::ZERO),
                top_items,
                by_status: status_breakdown,
            };
            
            // ===== INVENTORY METRICS =====
            let total_items: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM inventory_items WHERE company_id = $1 AND is_active = true"
            )
            .bind(auth_user.company_id)
            .fetch_one(pool)
            .await?;
            
            let (total_quantity, total_cost_value, total_selling_value): (Option<i64>, Option<Decimal>, Option<Decimal>) = 
                sqlx::query_as(
                    r#"
                    SELECT 
                        COALESCE(SUM(s.quantity_on_hand), 0),
                        COALESCE(SUM(s.quantity_on_hand * i.cost_price), 0),
                        COALESCE(SUM(s.quantity_on_hand * i.selling_price), 0)
                    FROM stock s
                    JOIN inventory_items i ON s.item_id = i.id
                    WHERE s.company_id = $1
                    "#
                )
                .bind(auth_user.company_id)
                .fetch_one(pool)
                .await?;
            
            let low_stock_items: i64 = sqlx::query_scalar(
                r#"
                SELECT COUNT(*)
                FROM stock s
                JOIN inventory_items i ON s.item_id = i.id
                WHERE s.company_id = $1 
                  AND s.quantity_on_hand < i.reorder_level
                  AND i.is_active = true
                "#
            )
            .bind(auth_user.company_id)
            .fetch_one(pool)
            .await?;
            
            let out_of_stock_items: i64 = sqlx::query_scalar(
                r#"
                SELECT COUNT(*)
                FROM stock s
                WHERE s.company_id = $1 AND s.quantity_on_hand = 0
                "#
            )
            .bind(auth_user.company_id)
            .fetch_one(pool)
            .await?;
            
            // Inventory by category
            let by_category: Vec<CategoryBreakdown> = sqlx::query_as(
                r#"
                SELECT 
                    c.id as category_id,
                    c.name as category_name,
                    COUNT(i.id) as item_count,
                    COALESCE(SUM(s.quantity_on_hand * i.cost_price), 0) as total_value
                FROM categories c
                LEFT JOIN inventory_items i ON i.category_id = c.id AND i.is_active = true
                LEFT JOIN stock s ON s.item_id = i.id
                WHERE c.company_id = $1 AND c.is_active = true
                GROUP BY c.id, c.name
                ORDER BY total_value DESC
                LIMIT 10
                "#
            )
            .bind(auth_user.company_id)
            .fetch_all(pool)
            .await?;
            
            let inventory = InventoryMetrics {
                total_items,
                total_quantity: total_quantity.unwrap_or(0),
                total_cost_value: total_cost_value.unwrap_or(Decimal::ZERO),
                total_selling_value: total_selling_value.unwrap_or(Decimal::ZERO),
                low_stock_items,
                out_of_stock_items,
                by_category,
            };
            
            // ===== PURCHASE METRICS =====
            let (total_purchases, po_count): (Option<Decimal>, i64) = sqlx::query_as(
                r#"
                SELECT 
                    COALESCE(SUM(total_amount), 0),
                    COUNT(*)
                FROM purchase_orders
                WHERE company_id = $1 
                  AND po_date BETWEEN $2 AND $3
                  AND status != 'cancelled'
                "#
            )
            .bind(auth_user.company_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_one(pool)
            .await?;
            
            let pending_pos: i64 = sqlx::query_scalar(
                r#"
                SELECT COUNT(*)
                FROM purchase_orders
                WHERE company_id = $1 AND status IN ('draft', 'submitted', 'confirmed')
                "#
            )
            .bind(auth_user.company_id)
            .fetch_one(pool)
            .await?;
            
            let total_purchases_val = total_purchases.unwrap_or(Decimal::ZERO);
            let po_count_val = po_count;
            
            let average_po_value = if po_count_val > 0 {
                total_purchases_val / Decimal::from(po_count_val)
            } else {
                Decimal::ZERO
            };
            
            // Top suppliers
            let top_suppliers: Vec<TopSupplier> = sqlx::query_as(
                r#"
                SELECT 
                    s.id as supplier_id,
                    s.name as supplier_name,
                    COUNT(po.id) as po_count,
                    COALESCE(SUM(po.total_amount), 0) as total_amount
                FROM suppliers s
                JOIN purchase_orders po ON po.supplier_id = s.id
                WHERE s.company_id = $1
                  AND po.po_date BETWEEN $2 AND $3
                  AND po.status != 'cancelled'
                GROUP BY s.id, s.name
                ORDER BY total_amount DESC
                LIMIT 5
                "#
            )
            .bind(auth_user.company_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(pool)
            .await?;
            
            let purchases = PurchaseMetrics {
                total_purchases: total_purchases_val,
                po_count: po_count_val,
                pending_pos,
                average_po_value,
                top_suppliers,
            };
            
            // ===== PENDING APPROVALS =====
            let pending_invoices: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM sales_invoices WHERE company_id = $1 AND status = 'pending'"
            )
            .bind(auth_user.company_id)
            .fetch_one(pool)
            .await?;
            
            let pending_approvals = PendingApprovals {
                sales_invoices: pending_invoices,
                purchase_orders: pending_pos,
            };
            
            // Build response
            let stats = DashboardStats {
                overview,
                sales,
                inventory,
                purchases,
                recent_activities: vec![],
                low_stock_count: low_stock_items,
                pending_approvals,
            };
            
            tracing::info!(
                total_revenue = %stats.overview.total_revenue,
                total_orders = stats.overview.total_orders,
                "Dashboard statistics retrieved"
            );
            
            Ok(Json(stats))
        }
        
DbPool::Sqlite(pool) => {
    // Fetch Current Period - use f64 for SQLite
    let (total_revenue_f, total_orders): (Option<f64>, i64) = sqlx::query_as::<Sqlite, (Option<f64>, i64)>(
        "SELECT COALESCE(SUM(total_amount), 0.0), COUNT(*) FROM sales_invoices 
         WHERE company_id = ? AND invoice_date BETWEEN ? AND ? AND status != 'cancelled'"
    )
    .bind(auth_user.company_id).bind(start_date).bind(end_date)
    .fetch_one(pool).await?;

    // Fetch Previous Period
    let (prev_revenue_f, prev_orders): (Option<f64>, i64) = sqlx::query_as::<Sqlite, (Option<f64>, i64)>(
        "SELECT COALESCE(SUM(total_amount), 0.0), COUNT(*) FROM sales_invoices 
         WHERE company_id = ? AND invoice_date BETWEEN ? AND ? AND status != 'cancelled'"
    )
    .bind(auth_user.company_id).bind(prev_start).bind(prev_end)
    .fetch_one(pool).await?;

    let total_customers = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM customers WHERE company_id = ? AND is_active = 1"
    ).bind(auth_user.company_id).fetch_one(pool).await?;

    let new_customers = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM customers WHERE company_id = ? AND created_at >= ? AND created_at <= ?"
    ).bind(auth_user.company_id).bind(start_date).bind(end_date).fetch_one(pool).await?;

    // Inventory value - returns f64
    let inventory_value_f: Option<f64> = sqlx::query_scalar(
        r#"
        SELECT COALESCE(SUM(s.quantity_on_hand * i.cost_price), 0.0)
        FROM stock s
        JOIN inventory_items i ON s.item_id = i.id
        WHERE s.company_id = ?
        "#
    )
    .bind(auth_user.company_id)
    .fetch_one(pool)
    .await?;

    let total_revenue_f = total_revenue_f.unwrap_or(0.0);
    let prev_revenue_f = prev_revenue_f.unwrap_or(0.0);
    let total_revenue_val = Decimal::from_f64_retain(total_revenue_f).unwrap_or(Decimal::ZERO);
    let prev_revenue_val = Decimal::from_f64_retain(prev_revenue_f).unwrap_or(Decimal::ZERO);
    let revenue_change = calculate_percentage_change(total_revenue_val, prev_revenue_val);
    
    let total_orders_val = total_orders;
    let prev_orders_val = prev_orders;
    let orders_change = calculate_percentage_change(Decimal::from(total_orders_val), Decimal::from(prev_orders_val));

    let overview = OverviewMetrics {
        total_revenue: total_revenue_val,
        revenue_change_percent: revenue_change,
        total_orders: total_orders_val,
        orders_change_percent: orders_change,
        total_customers,
        new_customers,
        inventory_value: Decimal::from_f64_retain(inventory_value_f.unwrap_or(0.0)).unwrap_or(Decimal::ZERO),
    };
    
    // ===== SALES METRICS =====
    let (outstanding_amount_f,): (Option<f64>,) = sqlx::query_as(
        r#"
        SELECT COALESCE(SUM(balance_due), 0.0)
        FROM sales_invoices
        WHERE company_id = ? AND status IN ('approved', 'pending')
        "#
    )
    .bind(auth_user.company_id)
    .fetch_one(pool)
    .await?;
    
    let outstanding_amount = Decimal::from_f64_retain(outstanding_amount_f.unwrap_or(0.0)).unwrap_or(Decimal::ZERO);
    
    let average_order_value = if total_orders_val > 0 {
        total_revenue_val / Decimal::from(total_orders_val)
    } else {
        Decimal::ZERO
    };
    
    // Top selling items - SQLite specific struct with String/Uuid conversion
    #[derive(sqlx::FromRow)]
    struct TopItemSqlite {
        item_id: String,  // Store as String first
        item_name: String,
        sku: Option<String>,
        quantity_sold: i64,
        total_revenue: f64,
    }
    
    let top_items_sqlite: Vec<TopItemSqlite> = sqlx::query_as(
        r#"
        SELECT 
            i.id as item_id,
            i.name as item_name,
            i.sku,
            COALESCE(SUM(sii.quantity), 0) as quantity_sold,
            COALESCE(SUM(sii.line_total), 0.0) as total_revenue
        FROM sales_invoice_items sii
        JOIN sales_invoices si ON sii.invoice_id = si.id
        JOIN inventory_items i ON sii.item_id = i.id
        WHERE si.company_id = ?
          AND si.invoice_date BETWEEN ? AND ?
          AND si.status != 'cancelled'
        GROUP BY i.id, i.name, i.sku
        ORDER BY quantity_sold DESC
        LIMIT 5
        "#
    )
    .bind(auth_user.company_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await?;
    
    // Convert to the real TopItem type, parsing String to Uuid
    let top_items: Vec<TopItem> = top_items_sqlite.into_iter().map(|item| {
        let item_id = Uuid::parse_str(&item.item_id).unwrap_or_else(|_| {
            // Fallback to generating a nil UUID if parsing fails
            tracing::warn!("Failed to parse UUID from SQLite id: {}", item.item_id);
            Uuid::nil()
        });
        
        TopItem {
            item_id,
            item_name: item.item_name,
            sku: item.sku.unwrap_or_default(),
            quantity_sold: item.quantity_sold,
            total_revenue: Decimal::from_f64_retain(item.total_revenue).unwrap_or(Decimal::ZERO),
        }
    }).collect();
    
    // Sales by status
    let status_counts: Vec<(String, i64)> = sqlx::query_as(
        r#"
        SELECT status, COUNT(*)
        FROM sales_invoices
        WHERE company_id = ? AND invoice_date BETWEEN ? AND ?
        GROUP BY status
        "#
    )
    .bind(auth_user.company_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await?;
    
    let mut status_breakdown = StatusBreakdown {
        draft: 0,
        pending: 0,
        approved: 0,
        paid: 0,
        cancelled: 0,
    };
    
    for (status, count) in status_counts {
        match status.as_str() {
            "draft" => status_breakdown.draft = count,
            "pending" => status_breakdown.pending = count,
            "approved" => status_breakdown.approved = count,
            "paid" => status_breakdown.paid = count,
            "cancelled" => status_breakdown.cancelled = count,
            _ => {}
        }
    }
    
    let sales = SalesMetrics {
        total_sales: total_revenue_val,
        invoice_count: total_orders_val,
        average_order_value,
        outstanding_amount,
        top_items,
        by_status: status_breakdown,
    };
    
    // ===== INVENTORY METRICS =====
    let total_items: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM inventory_items WHERE company_id = ? AND is_active = 1"
    )
    .bind(auth_user.company_id)
    .fetch_one(pool)
    .await?;
    
    let (total_quantity, total_cost_value_f, total_selling_value_f): (Option<i64>, Option<f64>, Option<f64>) = 
        sqlx::query_as(
            r#"
            SELECT 
                COALESCE(SUM(s.quantity_on_hand), 0),
                COALESCE(SUM(s.quantity_on_hand * i.cost_price), 0.0),
                COALESCE(SUM(s.quantity_on_hand * i.selling_price), 0.0)
            FROM stock s
            JOIN inventory_items i ON s.item_id = i.id
            WHERE s.company_id = ?
            "#
        )
        .bind(auth_user.company_id)
        .fetch_one(pool)
        .await?;
    
    let total_cost_value = Decimal::from_f64_retain(total_cost_value_f.unwrap_or(0.0)).unwrap_or(Decimal::ZERO);
    let total_selling_value = Decimal::from_f64_retain(total_selling_value_f.unwrap_or(0.0)).unwrap_or(Decimal::ZERO);
    
    let low_stock_items: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM stock s
        JOIN inventory_items i ON s.item_id = i.id
        WHERE s.company_id = ? 
          AND s.quantity_on_hand < i.reorder_level
          AND i.is_active = 1
        "#
    )
    .bind(auth_user.company_id)
    .fetch_one(pool)
    .await?;
    
    let out_of_stock_items: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM stock s
        WHERE s.company_id = ? AND s.quantity_on_hand = 0
        "#
    )
    .bind(auth_user.company_id)
    .fetch_one(pool)
    .await?;
    
    // Inventory by category - use a SQLite-specific struct
    #[derive(sqlx::FromRow)]
    struct CategoryBreakdownSqlite {
        category_id: String,  // Store as String for UUID
        category_name: String,
        item_count: i64,
        total_value: f64,
    }
    
    let by_category_sqlite: Vec<CategoryBreakdownSqlite> = sqlx::query_as(
        r#"
        SELECT 
            c.id as category_id,
            c.name as category_name,
            COUNT(i.id) as item_count,
            COALESCE(SUM(s.quantity_on_hand * i.cost_price), 0.0) as total_value
        FROM categories c
        LEFT JOIN inventory_items i ON i.category_id = c.id AND i.is_active = 1
        LEFT JOIN stock s ON s.item_id = i.id
        WHERE c.company_id = ? AND c.is_active = 1
        GROUP BY c.id, c.name
        ORDER BY total_value DESC
        LIMIT 10
        "#
    )
    .bind(auth_user.company_id)
    .fetch_all(pool)
    .await?;
    
    // Convert to real CategoryBreakdown type
    let by_category: Vec<CategoryBreakdown> = by_category_sqlite.into_iter().map(|cat| {
        let category_id = Uuid::parse_str(&cat.category_id).unwrap_or_else(|_| {
            tracing::warn!("Failed to parse UUID from SQLite category id: {}", cat.category_id);
            Uuid::nil()
        });
        
        CategoryBreakdown {
            category_id,
            category_name: cat.category_name,
            item_count: cat.item_count,
            total_value: Decimal::from_f64_retain(cat.total_value).unwrap_or(Decimal::ZERO),
        }
    }).collect();
    
    let inventory = InventoryMetrics {
        total_items,
        total_quantity: total_quantity.unwrap_or(0),
        total_cost_value,
        total_selling_value,
        low_stock_items,
        out_of_stock_items,
        by_category,
    };
    
    // ===== PURCHASE METRICS =====
    let (total_purchases_f, po_count): (Option<f64>, i64) = sqlx::query_as(
        r#"
        SELECT 
            COALESCE(SUM(total_amount), 0.0),
            COUNT(*)
        FROM purchase_orders
        WHERE company_id = ? 
          AND po_date BETWEEN ? AND ?
          AND status != 'cancelled'
        "#
    )
    .bind(auth_user.company_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_one(pool)
    .await?;
    
    let pending_pos: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM purchase_orders
        WHERE company_id = ? AND status IN ('draft', 'submitted', 'confirmed')
        "#
    )
    .bind(auth_user.company_id)
    .fetch_one(pool)
    .await?;
    
    let total_purchases_val = Decimal::from_f64_retain(total_purchases_f.unwrap_or(0.0)).unwrap_or(Decimal::ZERO);
    let po_count_val = po_count;
    
    let average_po_value = if po_count_val > 0 {
        total_purchases_val / Decimal::from(po_count_val)
    } else {
        Decimal::ZERO
    };
    
    // Top suppliers - SQLite specific struct
    #[derive(sqlx::FromRow)]
    struct TopSupplierSqlite {
        supplier_id: String,  // Store as String for UUID
        supplier_name: String,
        po_count: i64,
        total_amount: f64,
    }
    
    let top_suppliers_sqlite: Vec<TopSupplierSqlite> = sqlx::query_as(
        r#"
        SELECT 
            s.id as supplier_id,
            s.name as supplier_name,
            COUNT(po.id) as po_count,
            COALESCE(SUM(po.total_amount), 0.0) as total_amount
        FROM suppliers s
        JOIN purchase_orders po ON po.supplier_id = s.id
        WHERE s.company_id = ?
          AND po.po_date BETWEEN ? AND ?
          AND po.status != 'cancelled'
        GROUP BY s.id, s.name
        ORDER BY total_amount DESC
        LIMIT 5
        "#
    )
    .bind(auth_user.company_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await?;
    
    // Convert to real TopSupplier type
    let top_suppliers: Vec<TopSupplier> = top_suppliers_sqlite.into_iter().map(|supplier| {
        let supplier_id = Uuid::parse_str(&supplier.supplier_id).unwrap_or_else(|_| {
            tracing::warn!("Failed to parse UUID from SQLite supplier id: {}", supplier.supplier_id);
            Uuid::nil()
        });
        
        TopSupplier {
            supplier_id,
            supplier_name: supplier.supplier_name,
            po_count: supplier.po_count,
            total_amount: Decimal::from_f64_retain(supplier.total_amount).unwrap_or(Decimal::ZERO),
        }
    }).collect();
    
    let purchases = PurchaseMetrics {
        total_purchases: total_purchases_val,
        po_count: po_count_val,
        pending_pos,
        average_po_value,
        top_suppliers,
    };
    
    // ===== PENDING APPROVALS =====
    let pending_invoices: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sales_invoices WHERE company_id = ? AND status = 'pending'"
    )
    .bind(auth_user.company_id)
    .fetch_one(pool)
    .await?;
    
    let pending_approvals = PendingApprovals {
        sales_invoices: pending_invoices,
        purchase_orders: pending_pos,
    };
    
    // Build response
    let stats = DashboardStats {
        overview,
        sales,
        inventory,
        purchases,
        recent_activities: vec![],
        low_stock_count: low_stock_items,
        pending_approvals,
    };
    
    tracing::info!(
        total_revenue = %stats.overview.total_revenue,
        total_orders = stats.overview.total_orders,
        "Dashboard statistics retrieved"
    );
    
    Ok(Json(stats))
}
   
    }
}
// ===== HELPER: CALCULATE PERCENTAGE CHANGE =====
fn calculate_percentage_change(current: Decimal, previous: Decimal) -> Decimal {
    if previous == Decimal::ZERO {
        if current > Decimal::ZERO {
            Decimal::from(100) // 100% increase
        } else {
            Decimal::ZERO
        }
    } else {
        ((current - previous) / previous) * Decimal::from(100)
    }
}

// ===== SALES SUMMARY ENDPOINT =====
// Get detailed sales summary with trends
#[utoipa::path(
    get,
    path = "/api/v1/dashboard/sales-summary",
    tag = "dashboard",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("start_date" = Option<String>, Query, description = "Start date"),
        ("end_date" = Option<String>, Query, description = "End date"),
    ),
    responses(
        (status = 200, description = "Sales summary with trends"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn sales_summary(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Query(params): Query<DashboardQueryParams>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        "Fetching sales summary"
    );
    
    // We match on the DB pool and return the result of the match directly
    match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            let summary = crate::models::sales::SalesInvoice::get_sales_summary_pg(
                pool,
                auth_user.company_id,
                params.start_date,
                params.end_date,
            )
            .await?;
            
            Ok(success("Sales summary retrieved", summary))
        }
        DbPool::Sqlite(pool) => {
            let summary = crate::models::sales::SalesInvoice::get_sales_summary_sqlite(
                pool,
                auth_user.company_id,
                params.start_date,
                params.end_date,
            )
            .await?;
            
            Ok(success("Sales summary retrieved", summary))
        }
    }
}

// ===== INVENTORY VALUATION ENDPOINT =====
// Get inventory valuation breakdown

// SQLite-specific struct with f64
#[derive(Debug, sqlx::FromRow)]
struct BranchValuationSqlite {
    branch_id: String,  // UUID as string for SQLite
    branch_name: String,
    cost_value: f64,
    selling_value: f64,
    potential_profit: f64,
}

// Keep your original BranchValuation for the response
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct BranchValuation {
    pub branch_id: uuid::Uuid,
    pub branch_name: String,
    #[schema(value_type = f64)]
    pub cost_value: Decimal,
    #[schema(value_type = f64)]
    pub selling_value: Decimal,
    #[schema(value_type = f64)]
    pub potential_profit: Decimal,
}
#[utoipa::path(
    get,
    path = "/api/v1/dashboard/inventory-value",
    tag = "dashboard",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Inventory valuation by branch"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn inventory_valuation(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        "Fetching inventory valuation"
    );
    
    // Branch logic based on the active database driver
    match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            // --- POSTGRES BRANCH ---
            let valuation = sqlx::query_as::<Postgres, BranchValuation>(
                r#"
                SELECT 
                    b.id as branch_id,
                    b.name as branch_name,
                    COALESCE(SUM(s.quantity_on_hand * i.cost_price), 0) as cost_value,
                    COALESCE(SUM(s.quantity_on_hand * i.selling_price), 0) as selling_value,
                    COALESCE(SUM(s.quantity_on_hand * (i.selling_price - i.cost_price)), 0) as potential_profit
                FROM branches b
                LEFT JOIN stock s ON s.branch_id = b.id
                LEFT JOIN inventory_items i ON i.id = s.item_id AND i.is_active = true
                WHERE b.company_id = $1 AND b.is_active = true
                GROUP BY b.id, b.name
                ORDER BY cost_value DESC
                "#
            )
            .bind(auth_user.company_id)
            .fetch_all(pool)
            .await?;
            
            Ok(success("Inventory valuation retrieved", valuation))
        }
DbPool::Sqlite(pool) => {
    // Query using SQLite-specific struct
    let valuation_sqlite: Vec<BranchValuationSqlite> = sqlx::query_as(
        r#"
        SELECT 
            b.id as branch_id,
            b.name as branch_name,
            COALESCE(SUM(s.quantity_on_hand * i.cost_price), 0.0) as cost_value,
            COALESCE(SUM(s.quantity_on_hand * i.selling_price), 0.0) as selling_value,
            COALESCE(SUM(s.quantity_on_hand * (i.selling_price - i.cost_price)), 0.0) as potential_profit
        FROM branches b
        LEFT JOIN stock s ON s.branch_id = b.id
        LEFT JOIN inventory_items i ON i.id = s.item_id AND i.is_active = 1
        WHERE b.company_id = ? AND b.is_active = 1
        GROUP BY b.id, b.name
        ORDER BY cost_value DESC
        "#
    )
    .bind(auth_user.company_id)
    .fetch_all(pool)
    .await?;
    
    // Convert SQLite results to the proper BranchValuation type
    let valuation: Vec<BranchValuation> = valuation_sqlite
        .into_iter()
        .map(|item| BranchValuation {
            branch_id: uuid::Uuid::parse_str(&item.branch_id).unwrap_or_else(|_| {
                tracing::warn!("Failed to parse UUID from SQLite branch id: {}", item.branch_id);
                uuid::Uuid::nil()
            }),
            branch_name: item.branch_name,
            cost_value: Decimal::from_f64_retain(item.cost_value).unwrap_or(Decimal::ZERO),
            selling_value: Decimal::from_f64_retain(item.selling_value).unwrap_or(Decimal::ZERO),
            potential_profit: Decimal::from_f64_retain(item.potential_profit).unwrap_or(Decimal::ZERO),
        })
        .collect();
    
    Ok(success("Inventory valuation retrieved", valuation))
}
    }
}
// ===== BRANCH VALUATION =====


#[derive(Debug, Deserialize, ToSchema)]
pub struct ChartQueryParams {
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub granularity: Option<String>,
}


// SQLite-specific struct with appropriate types
#[derive(Debug, sqlx::FromRow)]
struct ChartDataPointSqlite {
    period: String,
    amount: f64,  // or String depending on your CAST
    count: i64,
}

// Your main struct for API response
#[derive(Debug, Serialize, sqlx::FromRow)]
struct ChartDataPoint {
    period: String,
    amount: Decimal,
    count: i64,
}
// ===== SALES CHART DATA ENDPOINT =====
// Get sales data for charts (daily/monthly trends)
#[utoipa::path(
    get,
    path = "/api/v1/dashboard/sales-chart",
    tag = "dashboard",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("start_date" = Option<String>, Query, description = "Start date"),
        ("end_date" = Option<String>, Query, description = "End date"),
        ("granularity" = Option<String>, Query, description = "day or month"),
    ),
    responses(
        (status = 200, description = "Sales chart data"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn sales_chart_data(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Query(params): Query<ChartQueryParams>,
) -> Result<impl axum::response::IntoResponse> { // Added AppError
    tracing::info!(
        user_id = %auth_user.id,
        granularity = ?params.granularity,
        "Fetching sales chart data"
    );
    
    let end_date = params.end_date.unwrap_or_else(|| chrono::Utc::now().date_naive());
    let start_date = params.start_date.unwrap_or_else(|| end_date - chrono::Duration::days(30));
    let granularity = params.granularity.as_deref().unwrap_or("day");

    match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            let date_trunc = if granularity == "month" { "month" } else { "day" };
            
            let chart_data = sqlx::query_as::<Postgres, ChartDataPoint>(&format!(
                r#"
                SELECT 
                    DATE_TRUNC('{}', invoice_date)::date as period,
                    COALESCE(SUM(total_amount), 0) as amount,
                    COUNT(*) as count
                FROM sales_invoices
                WHERE company_id = $1 AND invoice_date BETWEEN $2 AND $3 AND status != 'cancelled'
                GROUP BY 1 ORDER BY period
                "#,
                date_trunc
            ))
            .bind(auth_user.company_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(pool).await?;
            
            Ok(success("Sales chart data retrieved", chart_data))
        }
DbPool::Sqlite(pool) => {
    let date_format = if granularity == "month" { "%Y-%m-01" } else { "%Y-%m-%d" };
    
    let chart_data_sqlite: Vec<ChartDataPointSqlite> = sqlx::query_as(
        &format!(
            r#"
            SELECT 
                strftime('{}', invoice_date) as period,
                COALESCE(SUM(total_amount), 0) as amount,
                COUNT(*) as count
            FROM sales_invoices
            WHERE company_id = ? AND invoice_date BETWEEN ? AND ? AND status != 'cancelled'
            GROUP BY period ORDER BY period
            "#,
            date_format
        )
    )
    .bind(auth_user.company_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await?;
    
    // Convert to the main struct with Decimal
    let chart_data: Vec<ChartDataPoint> = chart_data_sqlite
        .into_iter()
        .map(|item| ChartDataPoint {
            period: item.period,
            amount: Decimal::from_f64_retain(item.amount).unwrap_or(Decimal::ZERO),
            count: item.count,
        })
        .collect();
    
    Ok(success("Sales chart data retrieved", chart_data))
}
    }
}
pub fn dashboard_router() -> axum::Router<Arc<AppState>> {
    use axum::routing::get;

    axum::Router::new()
        .route("/stats", get(get_dashboard_stats))
        .route("/sales-summary", get(sales_summary))
        .route("/inventory-valuation", get(inventory_valuation))
        .route("/sales-chart", get(sales_chart_data))
}
