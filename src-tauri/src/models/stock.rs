// src/models/stock.rs
// Stock management models for inventory tracking
// Handles stock levels, movements, transfers, and adjustments

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, SqlitePool, Postgres, Sqlite};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;
use sqlx::types::Decimal;
use rust_decimal::prelude::ToPrimitive;

// ===== STOCK MOVEMENT TYPE ENUM =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "stock_movement_type", rename_all = "lowercase")]
pub enum MovementType {
    #[serde(rename = "purchase")]
    Purchase,
    
    #[serde(rename = "sale")]
    Sale,
    
    #[serde(rename = "transfer")]
    Transfer,
    
    #[serde(rename = "adjustment")]
    Adjustment,
    
    #[serde(rename = "return")]
    Return,
    
    #[serde(rename = "damage")]
    Damage,
    
    #[serde(rename = "loss")]
    Loss,
}

// ===== STOCK MODEL =====
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Stock {
    pub id: Uuid,
    pub company_id: Uuid,
    pub item_id: Uuid,
    pub branch_id: Uuid,
    #[schema(example = 150)]
    pub quantity_on_hand: i32,
    
    #[schema(example = 20)]
    pub quantity_allocated: i32,
    
    #[sqlx(default)]
    #[schema(example = 130)]
    pub quantity_available: i32,
    
    #[schema(example = 50)]
    pub quantity_in_transit: i32,
    
    #[schema(example = "A-01-001")]
    pub bin_location: Option<String>,
    
    pub last_counted_at: Option<DateTime<Utc>>,
    pub last_count_qty: Option<i32>,
    pub variance: Option<i32>,
    pub updated_at: DateTime<Utc>,
}

// Stock doesn't have Decimal or Vec fields, so no intermediate struct needed for basic queries

// ===== STOCK WITH ITEM DETAILS =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StockWithItem {
    #[serde(flatten)]
    pub stock: Stock,
    
    #[schema(example = "SKU-MICRO-001")]
    pub item_sku: String,
    
    #[schema(example = "Professional Microscope")]
    pub item_name: String,
    
    #[schema(example = "PCS")]
    pub item_unit: String,
    
    #[schema(value_type = f64, example = 2500.00)]
    pub item_cost_price: Decimal,
    
    #[schema(value_type = f64, example = 4999.00)]
    pub item_selling_price: Decimal,
    
    #[schema(example = 10)]
    pub item_reorder_level: i32,
    
    #[schema(example = "San Francisco Main Warehouse")]
    pub branch_name: String,
    
    #[schema(value_type = f64, example = 375000.00)]
    pub total_cost_value: Decimal,
    
    #[schema(value_type = f64, example = 749850.00)]
    pub total_selling_value: Decimal,
}

// ===== STOCK MOVEMENT MODEL =====
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct StockMovement {
    pub id: Uuid,
    pub company_id: Uuid,
    pub item_id: Uuid,
    pub from_branch_id: Option<Uuid>,
    pub to_branch_id: Option<Uuid>,
    pub movement_type: MovementType,
    
    #[schema(example = 10)]
    pub quantity: i32,
    
    #[sqlx(default)]
    #[schema(value_type = f64, example = 2500.00)]
    pub unit_cost: Option<Decimal>,
    
    #[schema(example = "sales_invoice")]
    pub reference_type: Option<String>,
    
    pub reference_id: Option<Uuid>,
    pub batch_number: Option<String>,
    
    #[sqlx(default)]
    pub serial_numbers: Vec<String>,
    
    pub notes: Option<String>,
    pub movement_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}

// ===== STOCK MOVEMENT SQLITE INTERMEDIATE STRUCT =====
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct StockMovementSqlite {
    pub id: Uuid,
    pub company_id: Uuid,
    pub item_id: Uuid,
    pub from_branch_id: Option<Uuid>,
    pub to_branch_id: Option<Uuid>,
    pub movement_type: MovementType,
    pub quantity: i32,
    pub unit_cost: Option<f64>,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub batch_number: Option<String>,
    pub serial_numbers: sqlx::types::Json<Vec<String>>,
    pub notes: Option<String>,
    pub movement_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}

// ===== STOCK MOVEMENT WITH DETAILS =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StockMovementWithDetails {
    #[serde(flatten)]
    pub movement: StockMovement,
    
    #[schema(example = "SKU-MICRO-001")]
    pub item_sku: String,
    
    #[schema(example = "Professional Microscope")]
    pub item_name: String,
    
    pub from_branch_name: Option<String>,
    pub to_branch_name: Option<String>,
    pub created_by_username: String,
    
    #[schema(value_type = f64, example = 25000.00)]
    pub total_value: Option<Decimal>,
}

// ===== CREATE/UPDATE STOCK REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpsertStockRequest {
    pub company_id: Uuid,
    pub item_id: Uuid,
    pub branch_id: Uuid,
    
    #[validate(range(min = 0))]
    #[schema(example = 150)]
    pub quantity_on_hand: i32,
    
    #[validate(range(min = 0))]
    #[schema(example = 20)]
    pub quantity_allocated: Option<i32>,
    
    #[validate(range(min = 0))]
    #[schema(example = 50)]
    pub quantity_in_transit: Option<i32>,
    
    #[schema(example = "A-01-001")]
    pub bin_location: Option<String>,
}

// ===== STOCK ADJUSTMENT REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct StockAdjustmentRequest {
    pub item_id: Uuid,
    pub branch_id: Uuid,
    
    #[schema(example = "-5")]
    pub adjustment_quantity: i32,
    
    #[validate(length(min = 1, max = 500))]
    #[schema(example = "Physical count correction")]
    pub reason: String,
    
    pub unit_cost: Option<Decimal>,
    pub batch_number: Option<String>,
    pub serial_numbers: Option<Vec<String>>,
}

// ===== STOCK TRANSFER REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct StockTransferRequest {
    pub item_id: Uuid,
    pub from_branch_id: Uuid,
    pub to_branch_id: Uuid,
    
    #[validate(range(min = 1))]
    #[schema(example = 25)]
    pub quantity: i32,
    
    #[schema(example = "Restocking LA warehouse")]
    pub notes: Option<String>,
    
    pub unit_cost: Option<Decimal>,
    pub batch_number: Option<String>,
    pub serial_numbers: Option<Vec<String>>,
}

// ===== PHYSICAL COUNT REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct PhysicalCountRequest {
    pub item_id: Uuid,
    pub branch_id: Uuid,
    
    #[validate(range(min = 0))]
    #[schema(example = 148)]
    pub counted_quantity: i32,
    
    pub notes: Option<String>,
    
    #[schema(example = true)]
    pub auto_adjust: bool,
}

// ===== LOW STOCK ALERT =====
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct LowStockAlert {
    pub item_id: Uuid,
    
    #[schema(example = "SKU-MICRO-001")]
    pub item_sku: String,
    
    #[schema(example = "Professional Microscope")]
    pub item_name: String,
    
    pub branch_id: Uuid,
    
    #[schema(example = "San Francisco Main Warehouse")]
    pub branch_name: String,
    
    #[schema(example = 8)]
    pub current_quantity: i32,
    
    #[schema(example = 10)]
    pub reorder_level: i32,
    
    #[schema(example = 2)]
    pub shortage: i32,
    
    #[schema(example = 50)]
    pub reorder_quantity: i32,
    
    #[sqlx(default)]
    pub days_of_stock: Option<i32>,
}

// ===== HELPER STRUCTS FOR SQL QUERIES =====
#[derive(Debug, FromRow)]
pub struct StockWithItemRow {
    pub id: Uuid,
    pub company_id: Uuid,
    pub item_id: Uuid,
    pub branch_id: Uuid,
    pub quantity_on_hand: i32,
    pub quantity_allocated: i32,
    pub quantity_available: i32,
    pub quantity_in_transit: i32,
    pub bin_location: Option<String>,
    pub last_counted_at: Option<DateTime<Utc>>,
    pub last_count_qty: Option<i32>,
    pub variance: Option<i32>,
    pub updated_at: DateTime<Utc>,
    pub item_sku: String,
    pub item_name: String,
    pub item_unit: String,
    pub item_cost_price: Decimal,
    pub item_selling_price: Decimal,
    pub item_reorder_level: i32,
    pub branch_name: String,
    pub total_cost_value: Decimal,
    pub total_selling_value: Decimal,
}

#[derive(Debug, FromRow)]
pub struct StockWithItemRowSqlite {
    pub id: Uuid,
    pub company_id: Uuid,
    pub item_id: Uuid,
    pub branch_id: Uuid,
    pub quantity_on_hand: i32,
    pub quantity_allocated: i32,
    pub quantity_available: i32,
    pub quantity_in_transit: i32,
    pub bin_location: Option<String>,
    pub last_counted_at: Option<DateTime<Utc>>,
    pub last_count_qty: Option<i32>,
    pub variance: Option<i32>,
    pub updated_at: DateTime<Utc>,
    pub item_sku: String,
    pub item_name: String,
    pub item_unit: String,
    pub item_cost_price: f64,
    pub item_selling_price: f64,
    pub item_reorder_level: i32,
    pub branch_name: String,
    pub total_cost_value: f64,
    pub total_selling_value: f64,
}

#[derive(Debug, FromRow)]
pub struct StockMovementWithDetailsRow {
    pub id: Uuid,
    pub company_id: Uuid,
    pub item_id: Uuid,
    pub from_branch_id: Option<Uuid>,
    pub to_branch_id: Option<Uuid>,
    pub movement_type: MovementType,
    pub quantity: i32,
    pub unit_cost: Option<Decimal>,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub batch_number: Option<String>,
    pub serial_numbers: Vec<String>,
    pub notes: Option<String>,
    pub movement_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub item_sku: String,
    pub item_name: String,
    pub from_branch_name: Option<String>,
    pub to_branch_name: Option<String>,
    pub created_by_username: String,
    pub total_value: Option<Decimal>,
}

#[derive(Debug, FromRow)]
pub struct StockMovementWithDetailsRowSqlite {
    pub id: Uuid,
    pub company_id: Uuid,
    pub item_id: Uuid,
    pub from_branch_id: Option<Uuid>,
    pub to_branch_id: Option<Uuid>,
    pub movement_type: MovementType,
    pub quantity: i32,
    pub unit_cost: Option<f64>,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub batch_number: Option<String>,
    pub serial_numbers: sqlx::types::Json<Vec<String>>,
    pub notes: Option<String>,
    pub movement_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub item_sku: String,
    pub item_name: String,
    pub from_branch_name: Option<String>,
    pub to_branch_name: Option<String>,
    pub created_by_username: String,
    pub total_value: Option<f64>,
}

// ===== CONVERSION IMPLEMENTATIONS =====

impl From<StockMovementSqlite> for StockMovement {
    fn from(s: StockMovementSqlite) -> Self {
        Self {
            id: s.id,
            company_id: s.company_id,
            item_id: s.item_id,
            from_branch_id: s.from_branch_id,
            to_branch_id: s.to_branch_id,
            movement_type: s.movement_type,
            quantity: s.quantity,
            unit_cost: s.unit_cost.map(|v| Decimal::from_f64_retain(v).unwrap_or_default()),
            reference_type: s.reference_type,
            reference_id: s.reference_id,
            batch_number: s.batch_number,
            serial_numbers: s.serial_numbers.0,
            notes: s.notes,
            movement_date: s.movement_date,
            created_at: s.created_at,
            created_by: s.created_by,
        }
    }
}

impl From<StockWithItemRowSqlite> for StockWithItem {
    fn from(row: StockWithItemRowSqlite) -> Self {
        Self {
            stock: Stock {
                id: row.id,
                company_id: row.company_id,
                item_id: row.item_id,
                branch_id: row.branch_id,
                quantity_on_hand: row.quantity_on_hand,
                quantity_allocated: row.quantity_allocated,
                quantity_available: row.quantity_available,
                quantity_in_transit: row.quantity_in_transit,
                bin_location: row.bin_location,
                last_counted_at: row.last_counted_at,
                last_count_qty: row.last_count_qty,
                variance: row.variance,
                updated_at: row.updated_at,
            },
            item_sku: row.item_sku,
            item_name: row.item_name,
            item_unit: row.item_unit,
            item_cost_price: Decimal::from_f64_retain(row.item_cost_price).unwrap_or_default(),
            item_selling_price: Decimal::from_f64_retain(row.item_selling_price).unwrap_or_default(),
            item_reorder_level: row.item_reorder_level,
            branch_name: row.branch_name,
            total_cost_value: Decimal::from_f64_retain(row.total_cost_value).unwrap_or_default(),
            total_selling_value: Decimal::from_f64_retain(row.total_selling_value).unwrap_or_default(),
        }
    }
}

impl From<StockMovementWithDetailsRowSqlite> for StockMovementWithDetails {
    fn from(row: StockMovementWithDetailsRowSqlite) -> Self {
        Self {
            movement: StockMovement {
                id: row.id,
                company_id: row.company_id,
                item_id: row.item_id,
                from_branch_id: row.from_branch_id,
                to_branch_id: row.to_branch_id,
                movement_type: row.movement_type,
                quantity: row.quantity,
                unit_cost: row.unit_cost.map(|v| Decimal::from_f64_retain(v).unwrap_or_default()),
                reference_type: row.reference_type,
                reference_id: row.reference_id,
                batch_number: row.batch_number,
                serial_numbers: row.serial_numbers.0,
                notes: row.notes,
                movement_date: row.movement_date,
                created_at: row.created_at,
                created_by: row.created_by,
            },
            item_sku: row.item_sku,
            item_name: row.item_name,
            from_branch_name: row.from_branch_name,
            to_branch_name: row.to_branch_name,
            created_by_username: row.created_by_username,
            total_value: row.total_value.map(|v| Decimal::from_f64_retain(v).unwrap_or_default()),
        }
    }
}

// ===== STOCK DATABASE OPERATIONS =====
impl Stock {

        // ===== ADJUST QUANTITY =====
    pub async fn adjust_quantity_pg(
        pool: &PgPool,
        item_id: Uuid,
        branch_id: Uuid,
        adjustment: i32,
    ) -> Result<Stock, sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE stock
            SET quantity_on_hand = quantity_on_hand + $1,
                updated_at = NOW()
            WHERE item_id = $2 AND branch_id = $3
            "#
        )
        .bind(adjustment)
        .bind(item_id)
        .bind(branch_id)
        .execute(pool)
        .await?;
        
        Self::find_by_item_and_branch_pg(pool, item_id, branch_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }
    
    pub async fn adjust_quantity_sqlite(
        pool: &SqlitePool,
        item_id: Uuid,
        branch_id: Uuid,
        adjustment: i32,
    ) -> Result<Stock, sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE stock
            SET quantity_on_hand = quantity_on_hand + ?,
                updated_at = CURRENT_TIMESTAMP
            WHERE item_id = ? AND branch_id = ?
            "#
        )
        .bind(adjustment)
        .bind(item_id)
        .bind(branch_id)
        .execute(pool)
        .await?;
        
        Self::find_by_item_and_branch_sqlite(pool, item_id, branch_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    // ===== RECORD PHYSICAL COUNT =====
    pub async fn record_count_pg(
        pool: &PgPool,
        request: PhysicalCountRequest,
    ) -> Result<Stock, sqlx::Error> {
        let current_stock = Self::find_by_item_and_branch_pg(
            pool,
            request.item_id,
            request.branch_id,
        )
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;
        
        let variance = request.counted_quantity - current_stock.quantity_on_hand;
        
        let stock = sqlx::query_as::<Postgres, Stock>(
            r#"
            UPDATE stock
            SET last_counted_at = NOW(),
                last_count_qty = $1,
                variance = $2,
                quantity_on_hand = CASE WHEN $3 THEN $1 ELSE quantity_on_hand END,
                updated_at = NOW()
            WHERE item_id = $4 AND branch_id = $5
            RETURNING *
            "#
        )
        .bind(request.counted_quantity)
        .bind(variance)
        .bind(request.auto_adjust)
        .bind(request.item_id)
        .bind(request.branch_id)
        .fetch_one(pool)
        .await?;
        
        Ok(stock)
    }
    
    pub async fn record_count_sqlite(
        pool: &SqlitePool,
        request: PhysicalCountRequest,
    ) -> Result<Stock, sqlx::Error> {
        let current_stock = Self::find_by_item_and_branch_sqlite(
            pool,
            request.item_id,
            request.branch_id,
        )
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;
        
        let variance = request.counted_quantity - current_stock.quantity_on_hand;
        
        let stock = sqlx::query_as::<Sqlite, Stock>(
            r#"
            UPDATE stock
            SET last_counted_at = CURRENT_TIMESTAMP,
                last_count_qty = ?,
                variance = ?,
                quantity_on_hand = CASE WHEN ? THEN ? ELSE quantity_on_hand END,
                updated_at = CURRENT_TIMESTAMP
            WHERE item_id = ? AND branch_id = ?
            RETURNING *
            "#
        )
        .bind(request.counted_quantity)
        .bind(variance)
        .bind(request.auto_adjust)
        .bind(request.counted_quantity)
        .bind(request.item_id)
        .bind(request.branch_id)
        .fetch_one(pool)
        .await?;
        
        Ok(stock)
    }

    // ===== TRANSFER STOCK (Postgres) =====
    pub async fn transfer_pg(
        pool: &PgPool,
        request: &StockTransferRequest,
        company_id: Uuid,
        user_id: Uuid,
    ) -> Result<(Stock, Stock, StockMovement), sqlx::Error> {
        let mut tx = pool.begin().await?;
        
        let from_stock = sqlx::query_as::<Postgres, Stock>(
            r#"
            UPDATE stock
            SET quantity_on_hand = quantity_on_hand - $1,
                updated_at = NOW()
            WHERE item_id = $2 AND branch_id = $3
              AND quantity_on_hand >= $1
            RETURNING *
            "#
        )
        .bind(request.quantity)
        .bind(request.item_id)
        .bind(request.from_branch_id)
        .fetch_one(&mut *tx)
        .await?;
        
        let to_stock = sqlx::query_as::<Postgres, Stock>(
            r#"
            INSERT INTO stock (
                company_id, item_id, branch_id, quantity_on_hand,
                quantity_allocated, quantity_in_transit
            )
            VALUES ($1, $2, $3, $4, 0, 0)
            ON CONFLICT (item_id, branch_id)
            DO UPDATE SET
                quantity_on_hand = stock.quantity_on_hand + EXCLUDED.quantity_on_hand,
                updated_at = NOW()
            RETURNING *
            "#
        )
        .bind(company_id)
        .bind(request.item_id)
        .bind(request.to_branch_id)
        .bind(request.quantity)
        .fetch_one(&mut *tx)
        .await?;
        
        let movement = StockMovement::create_with_tx_pg(
            &mut *tx,
            company_id,
            request.item_id,
            Some(request.from_branch_id),
            Some(request.to_branch_id),
            MovementType::Transfer,
            request.quantity,
            request.unit_cost,
            None,
            None,
            request.batch_number.clone(),
            request.serial_numbers.clone(),
            request.notes.clone(),
            user_id,
        )
        .await?;
        
        tx.commit().await?;
        
        Ok((from_stock, to_stock, movement))
    }
    
    // ===== TRANSFER STOCK (SQLite) =====
    pub async fn transfer_sqlite(
        pool: &SqlitePool,
        request: &StockTransferRequest,
        company_id: Uuid,
        user_id: Uuid,
    ) -> Result<(Stock, Stock, StockMovement), sqlx::Error> {
        let mut tx = pool.begin().await?;
        
        let from_stock = sqlx::query_as::<Sqlite, Stock>(
            r#"
            UPDATE stock
            SET quantity_on_hand = quantity_on_hand - ?,
                updated_at = CURRENT_TIMESTAMP
            WHERE item_id = ? AND branch_id = ?
              AND quantity_on_hand >= ?
            RETURNING *
            "#
        )
        .bind(request.quantity)
        .bind(request.item_id)
        .bind(request.from_branch_id)
        .bind(request.quantity)
        .fetch_one(&mut *tx)
        .await?;
        
        // Check if stock exists at destination branch
        let existing = sqlx::query_as::<Sqlite, Stock>(
            "SELECT * FROM stock WHERE item_id = ? AND branch_id = ?"
        )
        .bind(request.item_id)
        .bind(request.to_branch_id)
        .fetch_optional(&mut *tx)
        .await?;
        
        let to_stock = if let Some(es) = existing {
            sqlx::query_as::<Sqlite, Stock>(
                r#"
                UPDATE stock
                SET quantity_on_hand = quantity_on_hand + ?,
                    updated_at = CURRENT_TIMESTAMP
                WHERE id = ?
                RETURNING *
                "#
            )
            .bind(request.quantity)
            .bind(es.id)
            .fetch_one(&mut *tx)
            .await?
        } else {
            sqlx::query_as::<Sqlite, Stock>(
                r#"
                INSERT INTO stock (
                    company_id, item_id, branch_id, quantity_on_hand,
                    quantity_allocated, quantity_in_transit
                )
                VALUES (?, ?, ?, ?, 0, 0)
                RETURNING *
                "#
            )
            .bind(company_id)
            .bind(request.item_id)
            .bind(request.to_branch_id)
            .bind(request.quantity)
            .fetch_one(&mut *tx)
            .await?
        };
        
        let movement = StockMovement::create_with_tx_sqlite(
            &mut *tx,
            company_id,
            request.item_id,
            Some(request.from_branch_id),
            Some(request.to_branch_id),
            MovementType::Transfer,
            request.quantity,
            request.unit_cost,
            None,
            None,
            request.batch_number.clone(),
            request.serial_numbers.clone(),
            request.notes.clone(),
            user_id,
        )
        .await?;
        
        tx.commit().await?;
        
        Ok((from_stock, to_stock, movement))
    }

    // ===== LIST STOCK BY COMPANY (Postgres) =====
    pub async fn list_by_company_pg(
        pool: &PgPool,
        company_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<StockWithItem>, sqlx::Error> {
        let item_rows = sqlx::query_as::<Postgres, StockWithItemRow>(
            r#"
            SELECT 
                s.id, s.company_id, s.item_id, s.branch_id,
                s.quantity_on_hand, s.quantity_allocated,
                s.quantity_available, s.quantity_in_transit,
                s.bin_location, s.last_counted_at, s.last_count_qty,
                s.variance, s.updated_at,
                i.sku as item_sku, i.name as item_name, i.unit_of_measure as item_unit,
                i.cost_price as item_cost_price, i.selling_price as item_selling_price,
                i.reorder_level as item_reorder_level,
                b.name as branch_name,
                (s.quantity_on_hand * i.cost_price) as total_cost_value,
                (s.quantity_on_hand * i.selling_price) as total_selling_value
            FROM stock s
            JOIN inventory_items i ON s.item_id = i.id
            JOIN branches b ON s.branch_id = b.id
            WHERE s.company_id = $1 AND i.is_active = true
            ORDER BY i.name, b.name
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(company_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        let stock_list = item_rows.into_iter().map(|row| {
            StockWithItem {
                stock: Stock {
                    id: row.id, company_id: row.company_id, item_id: row.item_id,
                    branch_id: row.branch_id, quantity_on_hand: row.quantity_on_hand,
                    quantity_allocated: row.quantity_allocated, quantity_available: row.quantity_available,
                    quantity_in_transit: row.quantity_in_transit, bin_location: row.bin_location,
                    last_counted_at: row.last_counted_at, last_count_qty: row.last_count_qty,
                    variance: row.variance, updated_at: row.updated_at,
                },
                item_sku: row.item_sku, item_name: row.item_name, item_unit: row.item_unit,
                item_cost_price: row.item_cost_price, item_selling_price: row.item_selling_price,
                item_reorder_level: row.item_reorder_level, branch_name: row.branch_name,
                total_cost_value: row.total_cost_value, total_selling_value: row.total_selling_value,
            }
        }).collect();
        
        Ok(stock_list)
    }
    
    // ===== LIST STOCK BY COMPANY (SQLite) =====
    pub async fn list_by_company_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<StockWithItem>, sqlx::Error> {
        let item_rows = sqlx::query_as::<Sqlite, StockWithItemRowSqlite>(
            r#"
            SELECT 
                s.id, s.company_id, s.item_id, s.branch_id,
                s.quantity_on_hand, s.quantity_allocated,
                s.quantity_available, s.quantity_in_transit,
                s.bin_location, s.last_counted_at, s.last_count_qty,
                s.variance, s.updated_at,
                i.sku as item_sku, i.name as item_name, i.unit_of_measure as item_unit,
                i.cost_price as item_cost_price, i.selling_price as item_selling_price,
                i.reorder_level as item_reorder_level,
                b.name as branch_name,
                (s.quantity_on_hand * i.cost_price) as total_cost_value,
                (s.quantity_on_hand * i.selling_price) as total_selling_value
            FROM stock s
            JOIN inventory_items i ON s.item_id = i.id
            JOIN branches b ON s.branch_id = b.id
            WHERE s.company_id = ? AND i.is_active = true
            ORDER BY i.name, b.name
            LIMIT ? OFFSET ?
            "#
        )
        .bind(company_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(item_rows.into_iter().map(StockWithItem::from).collect())
    }
    // Postgres: uses ON CONFLICT
    pub async fn upsert_pg(
        pool: &PgPool,
        request: UpsertStockRequest,
    ) -> Result<Stock, sqlx::Error> {
        let stock = sqlx::query_as::<Postgres, Stock>(
            r#"
            INSERT INTO stock (
                company_id, item_id, branch_id, quantity_on_hand,
                quantity_allocated, quantity_in_transit, bin_location
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (item_id, branch_id)
            DO UPDATE SET
                quantity_on_hand = EXCLUDED.quantity_on_hand,
                quantity_allocated = COALESCE(EXCLUDED.quantity_allocated, stock.quantity_allocated),
                quantity_in_transit = COALESCE(EXCLUDED.quantity_in_transit, stock.quantity_in_transit),
                bin_location = COALESCE(EXCLUDED.bin_location, stock.bin_location),
                updated_at = NOW()
            RETURNING *
            "#
        )
        .bind(request.company_id)
        .bind(request.item_id)
        .bind(request.branch_id)
        .bind(request.quantity_on_hand)
        .bind(request.quantity_allocated.unwrap_or(0))
        .bind(request.quantity_in_transit.unwrap_or(0))
        .bind(request.bin_location)
        .fetch_one(pool)
        .await?;
        
        Ok(stock)
    }
    
    // SQLite: uses INSERT OR REPLACE with manual conflict handling
    pub async fn upsert_sqlite(
        pool: &SqlitePool,
        request: UpsertStockRequest,
    ) -> Result<Stock, sqlx::Error> {
        // SQLite approach: try update first, then insert if not exists
        let existing = sqlx::query_as::<Sqlite, Stock>(
            "SELECT * FROM stock WHERE item_id = ? AND branch_id = ?"
        )
        .bind(request.item_id)
        .bind(request.branch_id)
        .fetch_optional(pool)
        .await?;
        
        if let Some(existing_stock) = existing {
            let stock = sqlx::query_as::<Sqlite, Stock>(
                r#"
                UPDATE stock
                SET quantity_on_hand = ?,
                    quantity_allocated = ?,
                    quantity_in_transit = ?,
                    bin_location = ?,
                    updated_at = CURRENT_TIMESTAMP
                WHERE id = ?
                RETURNING *
                "#
            )
            .bind(request.quantity_on_hand)
            .bind(request.quantity_allocated.unwrap_or(existing_stock.quantity_allocated))
            .bind(request.quantity_in_transit.unwrap_or(existing_stock.quantity_in_transit))
            .bind(request.bin_location.or(existing_stock.bin_location))
            .bind(existing_stock.id)
            .fetch_one(pool)
            .await?;
            
            Ok(stock)
        } else {
            let stock = sqlx::query_as::<Sqlite, Stock>(
                r#"
                INSERT INTO stock (
                    company_id, item_id, branch_id, quantity_on_hand,
                    quantity_allocated, quantity_in_transit, bin_location
                )
                VALUES (?, ?, ?, ?, ?, ?, ?)
                RETURNING *
                "#
            )
            .bind(request.company_id)
            .bind(request.item_id)
            .bind(request.branch_id)
            .bind(request.quantity_on_hand)
            .bind(request.quantity_allocated.unwrap_or(0))
            .bind(request.quantity_in_transit.unwrap_or(0))
            .bind(request.bin_location)
            .fetch_one(pool)
            .await?;
            
            Ok(stock)
        }
    }
    
    pub async fn find_by_item_and_branch_pg(
        pool: &PgPool,
        item_id: Uuid,
        branch_id: Uuid,
    ) -> Result<Option<Stock>, sqlx::Error> {
        let stock = sqlx::query_as::<Postgres, Stock>(
            "SELECT * FROM stock WHERE item_id = $1 AND branch_id = $2"
        )
        .bind(item_id)
        .bind(branch_id)
        .fetch_optional(pool)
        .await?;
        
        Ok(stock)
    }
    
    pub async fn find_by_item_and_branch_sqlite(
        pool: &SqlitePool,
        item_id: Uuid,
        branch_id: Uuid,
    ) -> Result<Option<Stock>, sqlx::Error> {
        let stock = sqlx::query_as::<Sqlite, Stock>(
            "SELECT * FROM stock WHERE item_id = ? AND branch_id = ?"
        )
        .bind(item_id)
        .bind(branch_id)
        .fetch_optional(pool)
        .await?;
        
        Ok(stock)
    }
    
    pub async fn list_by_item_pg(
        pool: &PgPool,
        item_id: Uuid,
    ) -> Result<Vec<Stock>, sqlx::Error> {
        let stock_list = sqlx::query_as::<Postgres, Stock>(
            "SELECT * FROM stock WHERE item_id = $1 ORDER BY branch_id"
        )
        .bind(item_id)
        .fetch_all(pool)
        .await?;
        
        Ok(stock_list)
    }
    
    pub async fn list_by_item_sqlite(
        pool: &SqlitePool,
        item_id: Uuid,
    ) -> Result<Vec<Stock>, sqlx::Error> {
        let stock_list = sqlx::query_as::<Sqlite, Stock>(
            "SELECT * FROM stock WHERE item_id = ? ORDER BY branch_id"
        )
        .bind(item_id)
        .fetch_all(pool)
        .await?;
        
        Ok(stock_list)
    }
    
    pub async fn list_by_branch_pg(
        pool: &PgPool,
        branch_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<StockWithItem>, sqlx::Error> {
        let item_rows = sqlx::query_as::<Postgres, StockWithItemRow>(
            r#"
            SELECT 
                s.id, s.company_id, s.item_id, s.branch_id,
                s.quantity_on_hand, s.quantity_allocated,
                s.quantity_available, s.quantity_in_transit,
                s.bin_location, s.last_counted_at, s.last_count_qty,
                s.variance, s.updated_at,
                i.sku as item_sku, i.name as item_name, i.unit_of_measure as item_unit,
                i.cost_price as item_cost_price, i.selling_price as item_selling_price,
                i.reorder_level as item_reorder_level,
                b.name as branch_name,
                (s.quantity_on_hand * i.cost_price) as total_cost_value,
                (s.quantity_on_hand * i.selling_price) as total_selling_value
            FROM stock s
            JOIN inventory_items i ON s.item_id = i.id
            JOIN branches b ON s.branch_id = b.id
            WHERE s.branch_id = $1
            ORDER BY i.name
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(branch_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        let stock_list = item_rows.into_iter().map(|row| {
            StockWithItem {
                stock: Stock {
                    id: row.id, company_id: row.company_id, item_id: row.item_id,
                    branch_id: row.branch_id, quantity_on_hand: row.quantity_on_hand,
                    quantity_allocated: row.quantity_allocated, quantity_available: row.quantity_available,
                    quantity_in_transit: row.quantity_in_transit, bin_location: row.bin_location,
                    last_counted_at: row.last_counted_at, last_count_qty: row.last_count_qty,
                    variance: row.variance, updated_at: row.updated_at,
                },
                item_sku: row.item_sku, item_name: row.item_name, item_unit: row.item_unit,
                item_cost_price: row.item_cost_price, item_selling_price: row.item_selling_price,
                item_reorder_level: row.item_reorder_level, branch_name: row.branch_name,
                total_cost_value: row.total_cost_value, total_selling_value: row.total_selling_value,
            }
        }).collect();
        
        Ok(stock_list)
    }
    
    pub async fn list_by_branch_sqlite(
        pool: &SqlitePool,
        branch_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<StockWithItem>, sqlx::Error> {
        let item_rows = sqlx::query_as::<Sqlite, StockWithItemRowSqlite>(
            r#"
            SELECT 
                s.id, s.company_id, s.item_id, s.branch_id,
                s.quantity_on_hand, s.quantity_allocated,
                s.quantity_available, s.quantity_in_transit,
                s.bin_location, s.last_counted_at, s.last_count_qty,
                s.variance, s.updated_at,
                i.sku as item_sku, i.name as item_name, i.unit_of_measure as item_unit,
                i.cost_price as item_cost_price, i.selling_price as item_selling_price,
                i.reorder_level as item_reorder_level,
                b.name as branch_name,
                (s.quantity_on_hand * i.cost_price) as total_cost_value,
                (s.quantity_on_hand * i.selling_price) as total_selling_value
            FROM stock s
            JOIN inventory_items i ON s.item_id = i.id
            JOIN branches b ON s.branch_id = b.id
            WHERE s.branch_id = ?
            ORDER BY i.name
            LIMIT ? OFFSET ?
            "#
        )
        .bind(branch_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(item_rows.into_iter().map(StockWithItem::from).collect())
    }
    
    // Postgres: uses GREATEST function
    pub async fn deallocate_pg(
        pool: &PgPool,
        item_id: Uuid,
        branch_id: Uuid,
        quantity: i32,
    ) -> Result<Stock, sqlx::Error> {
        let stock = sqlx::query_as::<Postgres, Stock>(
            r#"
            UPDATE stock
            SET quantity_allocated = GREATEST(quantity_allocated - $1, 0),
                updated_at = NOW()
            WHERE item_id = $2 AND branch_id = $3
            RETURNING *
            "#
        )
        .bind(quantity)
        .bind(item_id)
        .bind(branch_id)
        .fetch_one(pool)
        .await?;
        
        Ok(stock)
    }
    
    // SQLite: uses MAX function or CASE
    pub async fn deallocate_sqlite(
        pool: &SqlitePool,
        item_id: Uuid,
        branch_id: Uuid,
        quantity: i32,
    ) -> Result<Stock, sqlx::Error> {
        let stock = sqlx::query_as::<Sqlite, Stock>(
            r#"
            UPDATE stock
            SET quantity_allocated = MAX(quantity_allocated - ?, 0),
                updated_at = CURRENT_TIMESTAMP
            WHERE item_id = ? AND branch_id = ?
            RETURNING *
            "#
        )
        .bind(quantity)
        .bind(item_id)
        .bind(branch_id)
        .fetch_one(pool)
        .await?;
        
        Ok(stock)
    }
    
    pub async fn get_low_stock_items_pg(
        pool: &PgPool,
        company_id: Uuid,
    ) -> Result<Vec<LowStockAlert>, sqlx::Error> {
        let alerts = sqlx::query_as::<Postgres, LowStockAlert>(
            r#"
            SELECT 
                i.id as item_id,
                i.sku as item_sku,
                i.name as item_name,
                b.id as branch_id,
                b.name as branch_name,
                s.quantity_available as current_quantity,
                i.reorder_level,
                (i.reorder_level - s.quantity_available) as shortage,
                i.reorder_quantity,
                NULL::INTEGER as days_of_stock
            FROM stock s
            JOIN inventory_items i ON s.item_id = i.id
            JOIN branches b ON s.branch_id = b.id
            WHERE s.company_id = $1
              AND i.is_active = true
              AND s.quantity_available < i.reorder_level
            ORDER BY shortage DESC, i.name
            "#
        )
        .bind(company_id)
        .fetch_all(pool)
        .await?;
        
        Ok(alerts)
    }
    
    pub async fn get_low_stock_items_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
    ) -> Result<Vec<LowStockAlert>, sqlx::Error> {
        let alerts = sqlx::query_as::<Sqlite, LowStockAlert>(
            r#"
            SELECT 
                i.id as item_id,
                i.sku as item_sku,
                i.name as item_name,
                b.id as branch_id,
                b.name as branch_name,
                s.quantity_available as current_quantity,
                i.reorder_level,
                (i.reorder_level - s.quantity_available) as shortage,
                i.reorder_quantity,
                NULL as days_of_stock
            FROM stock s
            JOIN inventory_items i ON s.item_id = i.id
            JOIN branches b ON s.branch_id = b.id
            WHERE s.company_id = ?
              AND i.is_active = true
              AND s.quantity_available < i.reorder_level
            ORDER BY shortage DESC, i.name
            "#
        )
        .bind(company_id)
        .fetch_all(pool)
        .await?;
        
        Ok(alerts)
    }
    
    pub async fn get_valuation_by_branch_pg(
        pool: &PgPool,
        branch_id: Uuid,
    ) -> Result<(Decimal, Decimal), sqlx::Error> {
        let result = sqlx::query_as::<Postgres, (Option<Decimal>, Option<Decimal>)>(
            r#"
            SELECT 
                COALESCE(SUM(s.quantity_on_hand * i.cost_price), 0),
                COALESCE(SUM(s.quantity_on_hand * i.selling_price), 0)
            FROM stock s
            JOIN inventory_items i ON s.item_id = i.id
            WHERE s.branch_id = $1
            "#
        )
        .bind(branch_id)
        .fetch_one(pool)
        .await?;
        
        Ok((
            result.0.unwrap_or(Decimal::ZERO),
            result.1.unwrap_or(Decimal::ZERO),
        ))
    }
    
    pub async fn get_valuation_by_branch_sqlite(
        pool: &SqlitePool,
        branch_id: Uuid,
    ) -> Result<(Decimal, Decimal), sqlx::Error> {
        let result: (Option<f64>, Option<f64>) = sqlx::query_as(
            r#"
            SELECT 
                COALESCE(SUM(s.quantity_on_hand * i.cost_price), 0),
                COALESCE(SUM(s.quantity_on_hand * i.selling_price), 0)
            FROM stock s
            JOIN inventory_items i ON s.item_id = i.id
            WHERE s.branch_id = ?
            "#
        )
        .bind(branch_id)
        .fetch_one(pool)
        .await?;
        
        Ok((
            Decimal::from_f64_retain(result.0.unwrap_or(0.0)).unwrap_or_default(),
            Decimal::from_f64_retain(result.1.unwrap_or(0.0)).unwrap_or_default(),
        ))
    }
}

// ===== STOCK MOVEMENT DATABASE OPERATIONS =====
impl StockMovement {
        // Transaction versions for create
    pub async fn create_with_tx_pg(
        tx: &mut sqlx::PgConnection,
        company_id: Uuid,
        item_id: Uuid,
        from_branch_id: Option<Uuid>,
        to_branch_id: Option<Uuid>,
        movement_type: MovementType,
        quantity: i32,
        unit_cost: Option<Decimal>,
        reference_type: Option<String>,
        reference_id: Option<Uuid>,
        batch_number: Option<String>,
        serial_numbers: Option<Vec<String>>,
        notes: Option<String>,
        created_by: Uuid,
    ) -> Result<StockMovement, sqlx::Error> {
        let movement = sqlx::query_as::<Postgres, StockMovement>(
            r#"
            INSERT INTO stock_movements (
                company_id, item_id, from_branch_id, to_branch_id,
                movement_type, quantity, unit_cost, reference_type,
                reference_id, batch_number, serial_numbers, notes,
                movement_date, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, NOW(), $13)
            RETURNING *
            "#
        )
        .bind(company_id)
        .bind(item_id)
        .bind(from_branch_id)
        .bind(to_branch_id)
        .bind(movement_type)
        .bind(quantity)
        .bind(unit_cost)
        .bind(reference_type)
        .bind(reference_id)
        .bind(batch_number)
        .bind(serial_numbers.unwrap_or_default())
        .bind(notes)
        .bind(created_by)
        .fetch_one(tx)
        .await?;
        
        Ok(movement)
    }
    
    pub async fn create_with_tx_sqlite(
        tx: &mut sqlx::SqliteConnection,
        company_id: Uuid,
        item_id: Uuid,
        from_branch_id: Option<Uuid>,
        to_branch_id: Option<Uuid>,
        movement_type: MovementType,
        quantity: i32,
        unit_cost: Option<Decimal>,
        reference_type: Option<String>,
        reference_id: Option<Uuid>,
        batch_number: Option<String>,
        serial_numbers: Option<Vec<String>>,
        notes: Option<String>,
        created_by: Uuid,
    ) -> Result<StockMovement, sqlx::Error> {
        let movement_sqlite = sqlx::query_as::<Sqlite, StockMovementSqlite>(
            r#"
            INSERT INTO stock_movements (
                company_id, item_id, from_branch_id, to_branch_id,
                movement_type, quantity, unit_cost, reference_type,
                reference_id, batch_number, serial_numbers, notes,
                movement_date, created_by
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, ?)
            RETURNING *
            "#
        )
        .bind(company_id)
        .bind(item_id)
        .bind(from_branch_id)
        .bind(to_branch_id)
        .bind(movement_type)
        .bind(quantity)
        .bind(unit_cost.map(|v| v.to_f64().unwrap_or_default()))
        .bind(reference_type)
        .bind(reference_id)
        .bind(batch_number)
        .bind(serde_json::json!(serial_numbers.unwrap_or_default()))
        .bind(notes)
        .bind(created_by)
        .fetch_one(tx)
        .await?;
        
        Ok(StockMovement::from(movement_sqlite))
    }

    // ===== LIST BY BRANCH (Postgres) =====
    pub async fn list_by_branch_pg(
        pool: &PgPool,
        branch_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<StockMovementWithDetails>, sqlx::Error> {
        let rows = sqlx::query_as::<Postgres, StockMovementWithDetailsRow>(
            r#"
            SELECT 
                sm.id, sm.company_id, sm.item_id,
                sm.from_branch_id, sm.to_branch_id,
                sm.movement_type, sm.quantity, sm.unit_cost,
                sm.reference_type, sm.reference_id,
                sm.batch_number, sm.serial_numbers,
                sm.notes, sm.movement_date, sm.created_at, sm.created_by,
                i.sku as item_sku, i.name as item_name,
                b1.name as from_branch_name, b2.name as to_branch_name,
                u.username as created_by_username,
                (sm.quantity * sm.unit_cost) as total_value
            FROM stock_movements sm
            JOIN inventory_items i ON sm.item_id = i.id
            LEFT JOIN branches b1 ON sm.from_branch_id = b1.id
            LEFT JOIN branches b2 ON sm.to_branch_id = b2.id
            JOIN users u ON sm.created_by = u.id
            WHERE sm.from_branch_id = $1 OR sm.to_branch_id = $1
            ORDER BY sm.movement_date DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(branch_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        let movements = rows.into_iter().map(|row| {
            StockMovementWithDetails {
                movement: StockMovement {
                    id: row.id, company_id: row.company_id, item_id: row.item_id,
                    from_branch_id: row.from_branch_id, to_branch_id: row.to_branch_id,
                    movement_type: row.movement_type, quantity: row.quantity,
                    unit_cost: row.unit_cost, reference_type: row.reference_type,
                    reference_id: row.reference_id, batch_number: row.batch_number,
                    serial_numbers: row.serial_numbers, notes: row.notes,
                    movement_date: row.movement_date, created_at: row.created_at,
                    created_by: row.created_by,
                },
                item_sku: row.item_sku, item_name: row.item_name,
                from_branch_name: row.from_branch_name, to_branch_name: row.to_branch_name,
                created_by_username: row.created_by_username, total_value: row.total_value,
            }
        }).collect();
        
        Ok(movements)
    }
    
    // ===== LIST BY BRANCH (SQLite) =====
    pub async fn list_by_branch_sqlite(
        pool: &SqlitePool,
        branch_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<StockMovementWithDetails>, sqlx::Error> {
        let rows = sqlx::query_as::<Sqlite, StockMovementWithDetailsRowSqlite>(
            r#"
            SELECT 
                sm.id, sm.company_id, sm.item_id,
                sm.from_branch_id, sm.to_branch_id,
                sm.movement_type, sm.quantity, sm.unit_cost,
                sm.reference_type, sm.reference_id,
                sm.batch_number, sm.serial_numbers,
                sm.notes, sm.movement_date, sm.created_at, sm.created_by,
                i.sku as item_sku, i.name as item_name,
                b1.name as from_branch_name, b2.name as to_branch_name,
                u.username as created_by_username,
                (sm.quantity * sm.unit_cost) as total_value
            FROM stock_movements sm
            JOIN inventory_items i ON sm.item_id = i.id
            LEFT JOIN branches b1 ON sm.from_branch_id = b1.id
            LEFT JOIN branches b2 ON sm.to_branch_id = b2.id
            JOIN users u ON sm.created_by = u.id
            WHERE sm.from_branch_id = ? OR sm.to_branch_id = ?
            ORDER BY sm.movement_date DESC
            LIMIT ? OFFSET ?
            "#
        )
        .bind(branch_id)
        .bind(branch_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(rows.into_iter().map(StockMovementWithDetails::from).collect())
    }
    pub async fn create_pg(
        pool: &PgPool,
        company_id: Uuid,
        item_id: Uuid,
        from_branch_id: Option<Uuid>,
        to_branch_id: Option<Uuid>,
        movement_type: MovementType,
        quantity: i32,
        unit_cost: Option<Decimal>,
        reference_type: Option<String>,
        reference_id: Option<Uuid>,
        batch_number: Option<String>,
        serial_numbers: Option<Vec<String>>,
        notes: Option<String>,
        created_by: Uuid,
    ) -> Result<StockMovement, sqlx::Error> {
        let movement = sqlx::query_as::<Postgres, StockMovement>(
            r#"
            INSERT INTO stock_movements (
                company_id, item_id, from_branch_id, to_branch_id,
                movement_type, quantity, unit_cost, reference_type,
                reference_id, batch_number, serial_numbers, notes,
                movement_date, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, NOW(), $13)
            RETURNING *
            "#
        )
        .bind(company_id)
        .bind(item_id)
        .bind(from_branch_id)
        .bind(to_branch_id)
        .bind(movement_type)
        .bind(quantity)
        .bind(unit_cost)
        .bind(reference_type)
        .bind(reference_id)
        .bind(batch_number)
        .bind(serial_numbers.unwrap_or_default())
        .bind(notes)
        .bind(created_by)
        .fetch_one(pool)
        .await?;
        
        Ok(movement)
    }
    
    pub async fn create_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
        item_id: Uuid,
        from_branch_id: Option<Uuid>,
        to_branch_id: Option<Uuid>,
        movement_type: MovementType,
        quantity: i32,
        unit_cost: Option<Decimal>,
        reference_type: Option<String>,
        reference_id: Option<Uuid>,
        batch_number: Option<String>,
        serial_numbers: Option<Vec<String>>,
        notes: Option<String>,
        created_by: Uuid,
    ) -> Result<StockMovement, sqlx::Error> {
        let movement_sqlite = sqlx::query_as::<Sqlite, StockMovementSqlite>(
            r#"
            INSERT INTO stock_movements (
                company_id, item_id, from_branch_id, to_branch_id,
                movement_type, quantity, unit_cost, reference_type,
                reference_id, batch_number, serial_numbers, notes,
                movement_date, created_by
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, ?)
            RETURNING *
            "#
        )
        .bind(company_id)
        .bind(item_id)
        .bind(from_branch_id)
        .bind(to_branch_id)
        .bind(movement_type)
        .bind(quantity)
        .bind(unit_cost.map(|v| v.to_f64().unwrap_or_default()))
        .bind(reference_type)
        .bind(reference_id)
        .bind(batch_number)
        .bind(serde_json::json!(serial_numbers.unwrap_or_default()))
        .bind(notes)
        .bind(created_by)
        .fetch_one(pool)
        .await?;
        
        Ok(StockMovement::from(movement_sqlite))
    }
    
    pub async fn list_by_item_pg(
        pool: &PgPool,
        item_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<StockMovementWithDetails>, sqlx::Error> {
        let rows = sqlx::query_as::<Postgres, StockMovementWithDetailsRow>(
            r#"
            SELECT 
                sm.id, sm.company_id, sm.item_id,
                sm.from_branch_id, sm.to_branch_id,
                sm.movement_type, sm.quantity, sm.unit_cost,
                sm.reference_type, sm.reference_id,
                sm.batch_number, sm.serial_numbers,
                sm.notes, sm.movement_date, sm.created_at, sm.created_by,
                i.sku as item_sku, i.name as item_name,
                b1.name as from_branch_name, b2.name as to_branch_name,
                u.username as created_by_username,
                (sm.quantity * sm.unit_cost) as total_value
            FROM stock_movements sm
            JOIN inventory_items i ON sm.item_id = i.id
            LEFT JOIN branches b1 ON sm.from_branch_id = b1.id
            LEFT JOIN branches b2 ON sm.to_branch_id = b2.id
            JOIN users u ON sm.created_by = u.id
            WHERE sm.item_id = $1
            ORDER BY sm.movement_date DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(item_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        let movements = rows.into_iter().map(|row| {
            StockMovementWithDetails {
                movement: StockMovement {
                    id: row.id, company_id: row.company_id, item_id: row.item_id,
                    from_branch_id: row.from_branch_id, to_branch_id: row.to_branch_id,
                    movement_type: row.movement_type, quantity: row.quantity,
                    unit_cost: row.unit_cost, reference_type: row.reference_type,
                    reference_id: row.reference_id, batch_number: row.batch_number,
                    serial_numbers: row.serial_numbers, notes: row.notes,
                    movement_date: row.movement_date, created_at: row.created_at,
                    created_by: row.created_by,
                },
                item_sku: row.item_sku, item_name: row.item_name,
                from_branch_name: row.from_branch_name, to_branch_name: row.to_branch_name,
                created_by_username: row.created_by_username, total_value: row.total_value,
            }
        }).collect();
        
        Ok(movements)
    }
    
    pub async fn list_by_item_sqlite(
        pool: &SqlitePool,
        item_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<StockMovementWithDetails>, sqlx::Error> {
        let rows = sqlx::query_as::<Sqlite, StockMovementWithDetailsRowSqlite>(
            r#"
            SELECT 
                sm.id, sm.company_id, sm.item_id,
                sm.from_branch_id, sm.to_branch_id,
                sm.movement_type, sm.quantity, sm.unit_cost,
                sm.reference_type, sm.reference_id,
                sm.batch_number, sm.serial_numbers,
                sm.notes, sm.movement_date, sm.created_at, sm.created_by,
                i.sku as item_sku, i.name as item_name,
                b1.name as from_branch_name, b2.name as to_branch_name,
                u.username as created_by_username,
                (sm.quantity * sm.unit_cost) as total_value
            FROM stock_movements sm
            JOIN inventory_items i ON sm.item_id = i.id
            LEFT JOIN branches b1 ON sm.from_branch_id = b1.id
            LEFT JOIN branches b2 ON sm.to_branch_id = b2.id
            JOIN users u ON sm.created_by = u.id
            WHERE sm.item_id = ?
            ORDER BY sm.movement_date DESC
            LIMIT ? OFFSET ?
            "#
        )
        .bind(item_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(rows.into_iter().map(StockMovementWithDetails::from).collect())
    }
    
    pub async fn list_by_company_pg(
        pool: &PgPool,
        company_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<StockMovementWithDetails>, sqlx::Error> {
        let rows = sqlx::query_as::<Postgres, StockMovementWithDetailsRow>(
            r#"
            SELECT 
                sm.id, sm.company_id, sm.item_id,
                sm.from_branch_id, sm.to_branch_id,
                sm.movement_type, sm.quantity, sm.unit_cost,
                sm.reference_type, sm.reference_id,
                sm.batch_number, sm.serial_numbers,
                sm.notes, sm.movement_date, sm.created_at, sm.created_by,
                i.sku as item_sku, i.name as item_name,
                b1.name as from_branch_name, b2.name as to_branch_name,
                u.username as created_by_username,
                (sm.quantity * sm.unit_cost) as total_value
            FROM stock_movements sm
            JOIN inventory_items i ON sm.item_id = i.id
            LEFT JOIN branches b1 ON sm.from_branch_id = b1.id
            LEFT JOIN branches b2 ON sm.to_branch_id = b2.id
            JOIN users u ON sm.created_by = u.id
            WHERE sm.company_id = $1
            ORDER BY sm.movement_date DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(company_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        let movements = rows.into_iter().map(|row| {
            StockMovementWithDetails {
                movement: StockMovement {
                    id: row.id, company_id: row.company_id, item_id: row.item_id,
                    from_branch_id: row.from_branch_id, to_branch_id: row.to_branch_id,
                    movement_type: row.movement_type, quantity: row.quantity,
                    unit_cost: row.unit_cost, reference_type: row.reference_type,
                    reference_id: row.reference_id, batch_number: row.batch_number,
                    serial_numbers: row.serial_numbers, notes: row.notes,
                    movement_date: row.movement_date, created_at: row.created_at,
                    created_by: row.created_by,
                },
                item_sku: row.item_sku, item_name: row.item_name,
                from_branch_name: row.from_branch_name, to_branch_name: row.to_branch_name,
                created_by_username: row.created_by_username, total_value: row.total_value,
            }
        }).collect();
        
        Ok(movements)
    }
    
    pub async fn list_by_company_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<StockMovementWithDetails>, sqlx::Error> {
        let rows = sqlx::query_as::<Sqlite, StockMovementWithDetailsRowSqlite>(
            r#"
            SELECT 
                sm.id, sm.company_id, sm.item_id,
                sm.from_branch_id, sm.to_branch_id,
                sm.movement_type, sm.quantity, sm.unit_cost,
                sm.reference_type, sm.reference_id,
                sm.batch_number, sm.serial_numbers,
                sm.notes, sm.movement_date, sm.created_at, sm.created_by,
                i.sku as item_sku, i.name as item_name,
                b1.name as from_branch_name, b2.name as to_branch_name,
                u.username as created_by_username,
                (sm.quantity * sm.unit_cost) as total_value
            FROM stock_movements sm
            JOIN inventory_items i ON sm.item_id = i.id
            LEFT JOIN branches b1 ON sm.from_branch_id = b1.id
            LEFT JOIN branches b2 ON sm.to_branch_id = b2.id
            JOIN users u ON sm.created_by = u.id
            WHERE sm.company_id = ?
            ORDER BY sm.movement_date DESC
            LIMIT ? OFFSET ?
            "#
        )
        .bind(company_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(rows.into_iter().map(StockMovementWithDetails::from).collect())
    }
    
}

// ===== UNIT TESTS =====
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_movement_type_serialization() {
        let movement = MovementType::Purchase;
        let json = serde_json::to_string(&movement).unwrap();
        assert_eq!(json, r#""purchase""#);
    }
    
    #[test]
    fn test_stock_adjustment_validation() {
        let request = StockAdjustmentRequest {
            item_id: Uuid::new_v4(),
            branch_id: Uuid::new_v4(),
            adjustment_quantity: -5,
            reason: "Physical count correction".to_string(),
            unit_cost: None,
            batch_number: None,
            serial_numbers: None,
        };
        
        assert!(request.validate().is_ok());
    }
    
    #[test]
    fn test_stock_transfer_validation() {
        let request = StockTransferRequest {
            item_id: Uuid::new_v4(),
            from_branch_id: Uuid::new_v4(),
            to_branch_id: Uuid::new_v4(),
            quantity: 25,
            notes: Some("Restocking".to_string()),
            unit_cost: None,
            batch_number: None,
            serial_numbers: None,
        };
        
        assert!(request.validate().is_ok());
    }
}