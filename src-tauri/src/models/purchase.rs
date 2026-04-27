// src/models/purchase.rs
// Purchase order management models
// Handles procurement from suppliers with goods receipt tracking

use chrono::{DateTime, Utc, Datelike};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, SqlitePool, Postgres, Sqlite};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;
use sqlx::types::Decimal;
use rust_decimal::prelude::ToPrimitive;

// ===== PURCHASE ORDER STATUS ENUM =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "purchase_status", rename_all = "lowercase")]
pub enum PurchaseStatus {
    #[serde(rename = "draft")]
    Draft,
    
    #[serde(rename = "submitted")]
    Submitted,
    
    #[serde(rename = "confirmed")]
    Confirmed,
    
    #[serde(rename = "shipped")]
    Shipped,
    
    #[serde(rename = "received")]
    Received,
    
    #[serde(rename = "cancelled")]
    Cancelled,
}

// ===== PURCHASE ORDER MODEL =====
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PurchaseOrder {
    pub id: Uuid,
    pub company_id: Uuid,
    pub branch_id: Uuid,
    pub supplier_id: Uuid,
    
    #[schema(example = "PO-2024-00001")]
    pub po_number: String,
    
    pub po_date: chrono::NaiveDate,
    pub expected_delivery_date: Option<chrono::NaiveDate>,
    pub status: PurchaseStatus,
    
    #[schema(value_type = f64, example = 50000.00)]
    pub subtotal: Decimal,
    
    #[schema(value_type = f64, example = 2500.00)]
    pub discount_amount: Decimal,
    
    #[schema(value_type = f64, example = 4250.00)]
    pub tax_amount: Decimal,
    
    #[schema(value_type = f64, example = 500.00)]
    pub shipping_amount: Decimal,
    
    #[schema(value_type = f64, example = 52250.00)]
    pub total_amount: Decimal,
    
    #[schema(example = "USD")]
    pub currency: String,
    
    #[schema(value_type = f64, example = 1.0)]
    pub exchange_rate: Decimal,
    
    #[schema(example = 30)]
    pub payment_terms: i32,
    
    pub shipping_address: Option<String>,
    pub notes: Option<String>,
    
    #[sqlx(default)]
    pub metadata: serde_json::Value,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Option<Uuid>,
}

// ===== PURCHASE ORDER SQLITE INTERMEDIATE STRUCT =====
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PurchaseOrderSqlite {
    pub id: Uuid,
    pub company_id: Uuid,
    pub branch_id: Uuid,
    pub supplier_id: Uuid,
    pub po_number: String,
    pub po_date: chrono::NaiveDate,
    pub expected_delivery_date: Option<chrono::NaiveDate>,
    pub status: PurchaseStatus,
    pub subtotal: f64,
    pub discount_amount: f64,
    pub tax_amount: f64,
    pub shipping_amount: f64,
    pub total_amount: f64,
    pub currency: String,
    pub exchange_rate: f64,
    pub payment_terms: i32,
    pub shipping_address: Option<String>,
    pub notes: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Option<Uuid>,
}

// ===== PURCHASE ORDER ITEM MODEL =====
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PurchaseOrderItem {
    pub id: Uuid,
    pub po_id: Uuid,
    pub item_id: Uuid,
    pub description: Option<String>,
    
    #[schema(example = 100)]
    pub quantity_ordered: i32,
    
    #[schema(example = 95)]
    pub quantity_received: i32,
    
    #[schema(value_type = f64, example = 2500.00)]
    pub unit_cost: Decimal,
    
    #[schema(value_type = f64, example = 8.5)]
    pub tax_percentage: Decimal,
    
    #[schema(value_type = f64, example = 212.50)]
    pub tax_amount: Decimal,
    
    #[sqlx(default)]
    #[schema(value_type = f64, example = 250212.50)]
    pub line_total: Decimal,
    
    pub created_at: DateTime<Utc>,
}

// ===== PURCHASE ORDER ITEM SQLITE INTERMEDIATE STRUCT =====
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PurchaseOrderItemSqlite {
    pub id: Uuid,
    pub po_id: Uuid,
    pub item_id: Uuid,
    pub description: Option<String>,
    pub quantity_ordered: i32,
    pub quantity_received: i32,
    pub unit_cost: f64,
    pub tax_percentage: f64,
    pub tax_amount: f64,
    pub line_total: f64,
    pub created_at: DateTime<Utc>,
}

// ===== PURCHASE ORDER WITH ITEMS =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PurchaseOrderWithItems {
    #[serde(flatten)]
    pub purchase_order: PurchaseOrder,
    
    pub items: Vec<PurchaseOrderItemWithDetails>,
    
    pub supplier_name: String,
    pub supplier_email: Option<String>,
    pub supplier_phone: Option<String>,
    
    pub branch_name: String,
    pub created_by_username: String,
}

// ===== PURCHASE ORDER ITEM WITH DETAILS =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PurchaseOrderItemWithDetails {
    #[serde(flatten)]
    pub item: PurchaseOrderItem,
    
    pub item_sku: String,
    pub item_name: String,
    pub item_unit: String,
}

// ===== CREATE PURCHASE ORDER REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreatePurchaseOrderRequest {
    pub company_id: Uuid,
    pub branch_id: Uuid,
    pub supplier_id: Uuid,
    
    pub po_date: Option<chrono::NaiveDate>,
    pub expected_delivery_date: Option<chrono::NaiveDate>,
    
    #[validate(length(min = 1))]
    pub items: Vec<CreatePurchaseOrderItemRequest>,
    
    #[validate(custom = "validate_decimal_non_negative")]
    pub discount_amount: Option<Decimal>,
    
    #[validate(custom = "validate_decimal_non_negative")]
    pub shipping_amount: Option<Decimal>,
    
    pub currency: Option<String>,
    
    #[validate(custom = "validate_decimal_non_negative")]
    pub exchange_rate: Option<Decimal>,
    
    pub payment_terms: Option<i32>,
    pub shipping_address: Option<String>,
    pub notes: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

// ===== CREATE PURCHASE ORDER ITEM REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreatePurchaseOrderItemRequest {
    pub item_id: Uuid,
    pub description: Option<String>,
    
    #[validate(range(min = 1))]
    #[schema(example = 100)]
    pub quantity_ordered: i32,
    
    #[validate(custom = "validate_decimal_non_negative")]
    #[schema(example = 2500.00)]
    pub unit_cost: Decimal,
    
    #[validate(custom = "validate_decimal_percentage")]
    #[schema(example = 8.5)]
    pub tax_percentage: Option<Decimal>,
}

// ===== UPDATE PURCHASE ORDER REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdatePurchaseOrderRequest {
    pub expected_delivery_date: Option<chrono::NaiveDate>,
    
    #[validate(custom = "validate_decimal_non_negative")]
    pub discount_amount: Option<Decimal>,
    
    #[validate(custom = "validate_decimal_non_negative")]
    pub shipping_amount: Option<Decimal>,
    
    pub shipping_address: Option<String>,
    pub notes: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

// ===== RECEIVE GOODS REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ReceiveGoodsRequest {
    #[validate(length(min = 1))]
    pub items: Vec<ReceiveGoodsItemRequest>,
    pub notes: Option<String>,
}

// ===== RECEIVE GOODS ITEM REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ReceiveGoodsItemRequest {
    pub po_item_id: Uuid,
    
    #[validate(range(min = 1))]
    #[schema(example = 95)]
    pub quantity_received: i32,
    
    pub serial_numbers: Option<Vec<String>>,
    pub batch_number: Option<String>,
}

// Custom validators for Decimal types
fn validate_decimal_non_negative(value: &Decimal) -> Result<(), validator::ValidationError> {
    if *value >= Decimal::ZERO {
        Ok(())
    } else {
        Err(validator::ValidationError::new("Value must be non-negative"))
    }
}

fn validate_decimal_percentage(value: &Decimal) -> Result<(), validator::ValidationError> {
    if *value >= Decimal::ZERO && *value <= Decimal::new(100, 0) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("Percentage must be between 0 and 100"))
    }
}

// ===== HELPER STRUCT FOR SQL QUERIES =====
#[derive(Debug, FromRow)]
pub struct PurchaseOrderItemWithDetailsRow {
    pub id: Uuid,
    pub po_id: Uuid,
    pub item_id: Uuid,
    pub description: Option<String>,
    pub quantity_ordered: i32,
    pub quantity_received: i32,
    pub unit_cost: Decimal,
    pub tax_percentage: Decimal,
    pub tax_amount: Decimal,
    pub line_total: Decimal,
    pub created_at: DateTime<Utc>,
    pub item_sku: String,
    pub item_name: String,
    pub item_unit: String,
}

#[derive(Debug, FromRow)]
pub struct PurchaseOrderItemWithDetailsRowSqlite {
    pub id: Uuid,
    pub po_id: Uuid,
    pub item_id: Uuid,
    pub description: Option<String>,
    pub quantity_ordered: i32,
    pub quantity_received: i32,
    pub unit_cost: f64,
    pub tax_percentage: f64,
    pub tax_amount: f64,
    pub line_total: f64,
    pub created_at: DateTime<Utc>,
    pub item_sku: String,
    pub item_name: String,
    pub item_unit: String,
}

// ===== CONVERSION IMPLEMENTATIONS =====

impl From<PurchaseOrderSqlite> for PurchaseOrder {
    fn from(s: PurchaseOrderSqlite) -> Self {
        Self {
            id: s.id,
            company_id: s.company_id,
            branch_id: s.branch_id,
            supplier_id: s.supplier_id,
            po_number: s.po_number,
            po_date: s.po_date,
            expected_delivery_date: s.expected_delivery_date,
            status: s.status,
            subtotal: Decimal::from_f64_retain(s.subtotal).unwrap_or_default(),
            discount_amount: Decimal::from_f64_retain(s.discount_amount).unwrap_or_default(),
            tax_amount: Decimal::from_f64_retain(s.tax_amount).unwrap_or_default(),
            shipping_amount: Decimal::from_f64_retain(s.shipping_amount).unwrap_or_default(),
            total_amount: Decimal::from_f64_retain(s.total_amount).unwrap_or_default(),
            currency: s.currency,
            exchange_rate: Decimal::from_f64_retain(s.exchange_rate).unwrap_or_default(),
            payment_terms: s.payment_terms,
            shipping_address: s.shipping_address,
            notes: s.notes,
            metadata: s.metadata,
            created_at: s.created_at,
            updated_at: s.updated_at,
            created_by: s.created_by,
            updated_by: s.updated_by,
        }
    }
}

impl From<PurchaseOrderItemSqlite> for PurchaseOrderItem {
    fn from(s: PurchaseOrderItemSqlite) -> Self {
        Self {
            id: s.id,
            po_id: s.po_id,
            item_id: s.item_id,
            description: s.description,
            quantity_ordered: s.quantity_ordered,
            quantity_received: s.quantity_received,
            unit_cost: Decimal::from_f64_retain(s.unit_cost).unwrap_or_default(),
            tax_percentage: Decimal::from_f64_retain(s.tax_percentage).unwrap_or_default(),
            tax_amount: Decimal::from_f64_retain(s.tax_amount).unwrap_or_default(),
            line_total: Decimal::from_f64_retain(s.line_total).unwrap_or_default(),
            created_at: s.created_at,
        }
    }
}

impl From<PurchaseOrderItemWithDetailsRowSqlite> for PurchaseOrderItemWithDetails {
    fn from(row: PurchaseOrderItemWithDetailsRowSqlite) -> Self {
        Self {
            item: PurchaseOrderItem {
                id: row.id,
                po_id: row.po_id,
                item_id: row.item_id,
                description: row.description,
                quantity_ordered: row.quantity_ordered,
                quantity_received: row.quantity_received,
                unit_cost: Decimal::from_f64_retain(row.unit_cost).unwrap_or_default(),
                tax_percentage: Decimal::from_f64_retain(row.tax_percentage).unwrap_or_default(),
                tax_amount: Decimal::from_f64_retain(row.tax_amount).unwrap_or_default(),
                line_total: Decimal::from_f64_retain(row.line_total).unwrap_or_default(),
                created_at: row.created_at,
            },
            item_sku: row.item_sku,
            item_name: row.item_name,
            item_unit: row.item_unit,
        }
    }
}

// ===== PURCHASE ORDER DATABASE OPERATIONS =====
impl PurchaseOrder {
    // Postgres: uses EXTRACT(YEAR FROM ...)
    async fn generate_po_number_pg(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        company_id: Uuid,
    ) -> Result<String, sqlx::Error> {
        let year = chrono::Utc::now().year();
        
        let last_number: Option<String> = sqlx::query_scalar(
            r#"
            SELECT po_number FROM purchase_orders
            WHERE company_id = $1 
              AND EXTRACT(YEAR FROM po_date) = $2
            ORDER BY created_at DESC
            LIMIT 1
            "#
        )
        .bind(company_id)
        .bind(year as i32)
        .fetch_optional(&mut **tx)
        .await?;
        
        let sequence = if let Some(last) = last_number {
            let parts: Vec<&str> = last.split('-').collect();
            if parts.len() == 3 {
                parts[2].parse::<u32>().unwrap_or(0) + 1
            } else {
                1
            }
        } else {
            1
        };
        
        Ok(format!("PO-{}-{:05}", year, sequence))
    }
    
    // SQLite: uses strftime('%Y', ...)
    async fn generate_po_number_sqlite(
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        company_id: Uuid,
    ) -> Result<String, sqlx::Error> {
        let year = chrono::Utc::now().year();
        
        let last_number: Option<String> = sqlx::query_scalar(
            r#"
            SELECT po_number FROM purchase_orders
            WHERE company_id = ? 
              AND strftime('%Y', po_date) = ?
            ORDER BY created_at DESC
            LIMIT 1
            "#
        )
        .bind(company_id)
        .bind(year.to_string())
        .fetch_optional(&mut **tx)
        .await?;
        
        let sequence = if let Some(last) = last_number {
            let parts: Vec<&str> = last.split('-').collect();
            if parts.len() == 3 {
                parts[2].parse::<u32>().unwrap_or(0) + 1
            } else {
                1
            }
        } else {
            1
        };
        
        Ok(format!("PO-{}-{:05}", year, sequence))
    }
    
    // ===== CREATE PURCHASE ORDER (Postgres) =====
    pub async fn create_pg(
        pool: &PgPool,
        request: CreatePurchaseOrderRequest,
        created_by: Uuid,
    ) -> Result<PurchaseOrderWithItems, sqlx::Error> {
        let mut tx = pool.begin().await?;
        
        let po_number = Self::generate_po_number_pg(&mut tx, request.company_id).await?;
        
        let mut subtotal = Decimal::ZERO;
        let mut total_tax = Decimal::ZERO;
        
        for item in &request.items {
            let line_subtotal = item.unit_cost * Decimal::from(item.quantity_ordered);
            let tax_pct = item.tax_percentage.unwrap_or(Decimal::ZERO);
            let tax_amt = line_subtotal * (tax_pct / Decimal::from(100));
            
            subtotal += line_subtotal;
            total_tax += tax_amt;
        }
        
        let discount_amount = request.discount_amount.unwrap_or(Decimal::ZERO);
        let shipping_amount = request.shipping_amount.unwrap_or(Decimal::ZERO);
        let total_amount = subtotal - discount_amount + total_tax + shipping_amount;
        
        let payment_terms = if let Some(terms) = request.payment_terms {
            terms
        } else {
            sqlx::query_scalar::<Postgres, i32>(
                "SELECT payment_terms FROM suppliers WHERE id = $1"
            )
            .bind(request.supplier_id)
            .fetch_one(&mut *tx)
            .await?
        };
        
        let po = sqlx::query_as::<Postgres, PurchaseOrder>(
            r#"
            INSERT INTO purchase_orders (
                company_id, branch_id, supplier_id, po_number, po_date,
                expected_delivery_date, status, subtotal, discount_amount,
                tax_amount, shipping_amount, total_amount, currency,
                exchange_rate, payment_terms, shipping_address, notes,
                metadata, created_by, updated_by
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13,
                $14, $15, $16, $17, $18, $19, $20
            )
            RETURNING *
            "#
        )
        .bind(request.company_id)
        .bind(request.branch_id)
        .bind(request.supplier_id)
        .bind(&po_number)
        .bind(request.po_date.unwrap_or_else(|| chrono::Utc::now().date_naive()))
        .bind(request.expected_delivery_date)
        .bind(PurchaseStatus::Draft)
        .bind(subtotal)
        .bind(discount_amount)
        .bind(total_tax)
        .bind(shipping_amount)
        .bind(total_amount)
        .bind(request.currency.unwrap_or_else(|| "USD".to_string()))
        .bind(request.exchange_rate.unwrap_or(Decimal::ONE))
        .bind(payment_terms)
        .bind(request.shipping_address)
        .bind(request.notes)
        .bind(request.metadata.unwrap_or(serde_json::json!({})))
        .bind(created_by)
        .bind(created_by)
        .fetch_one(&mut *tx)
        .await?;
        
        let mut items_with_details = Vec::new();
        
        for item_req in request.items {
            let item_details: (String, String, String) = sqlx::query_as(
                "SELECT sku, name, unit_of_measure FROM inventory_items WHERE id = $1"
            )
            .bind(item_req.item_id)
            .fetch_one(&mut *tx)
            .await?;
            
            let tax_pct = item_req.tax_percentage.unwrap_or(Decimal::ZERO);
            let line_subtotal = item_req.unit_cost * Decimal::from(item_req.quantity_ordered);
            let tax_amt = line_subtotal * (tax_pct / Decimal::from(100));
            
            let po_item = sqlx::query_as::<Postgres, PurchaseOrderItem>(
                r#"
                INSERT INTO purchase_order_items (
                    po_id, item_id, description, quantity_ordered,
                    quantity_received, unit_cost, tax_percentage, tax_amount
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING *
                "#
            )
            .bind(po.id)
            .bind(item_req.item_id)
            .bind(item_req.description)
            .bind(item_req.quantity_ordered)
            .bind(0)
            .bind(item_req.unit_cost)
            .bind(tax_pct)
            .bind(tax_amt)
            .fetch_one(&mut *tx)
            .await?;
            
            items_with_details.push(PurchaseOrderItemWithDetails {
                item: po_item,
                item_sku: item_details.0,
                item_name: item_details.1,
                item_unit: item_details.2,
            });
        }
        
        let (supplier_name, supplier_email, supplier_phone): (String, Option<String>, Option<String>) = 
            sqlx::query_as("SELECT name, email, phone FROM suppliers WHERE id = $1")
            .bind(request.supplier_id)
            .fetch_one(&mut *tx)
            .await?;
        
        let branch_name: String = sqlx::query_scalar("SELECT name FROM branches WHERE id = $1")
            .bind(request.branch_id)
            .fetch_one(&mut *tx)
            .await?;
        
        let created_by_username: String = sqlx::query_scalar("SELECT username FROM users WHERE id = $1")
            .bind(created_by)
            .fetch_one(&mut *tx)
            .await?;
        
        tx.commit().await?;
        
        Ok(PurchaseOrderWithItems {
            purchase_order: po,
            items: items_with_details,
            supplier_name,
            supplier_email,
            supplier_phone,
            branch_name,
            created_by_username,
        })
    }
    
    // ===== CREATE PURCHASE ORDER (SQLite) =====
    pub async fn create_sqlite(
        pool: &SqlitePool,
        request: CreatePurchaseOrderRequest,
        created_by: Uuid,
    ) -> Result<PurchaseOrderWithItems, sqlx::Error> {
        let mut tx = pool.begin().await?;
        
        let po_number = Self::generate_po_number_sqlite(&mut tx, request.company_id).await?;
        
        let mut subtotal = Decimal::ZERO;
        let mut total_tax = Decimal::ZERO;
        
        for item in &request.items {
            let line_subtotal = item.unit_cost * Decimal::from(item.quantity_ordered);
            let tax_pct = item.tax_percentage.unwrap_or(Decimal::ZERO);
            let tax_amt = line_subtotal * (tax_pct / Decimal::from(100));
            
            subtotal += line_subtotal;
            total_tax += tax_amt;
        }
        
        let discount_amount = request.discount_amount.unwrap_or(Decimal::ZERO);
        let shipping_amount = request.shipping_amount.unwrap_or(Decimal::ZERO);
        let total_amount = subtotal - discount_amount + total_tax + shipping_amount;
        
        let payment_terms = if let Some(terms) = request.payment_terms {
            terms
        } else {
            sqlx::query_scalar::<Sqlite, i32>(
                "SELECT payment_terms FROM suppliers WHERE id = ?"
            )
            .bind(request.supplier_id)
            .fetch_one(&mut *tx)
            .await?
        };
        
        let po_sqlite = sqlx::query_as::<Sqlite, PurchaseOrderSqlite>(
            r#"
            INSERT INTO purchase_orders (
                company_id, branch_id, supplier_id, po_number, po_date,
                expected_delivery_date, status, subtotal, discount_amount,
                tax_amount, shipping_amount, total_amount, currency,
                exchange_rate, payment_terms, shipping_address, notes,
                metadata, created_by, updated_by
            )
            VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?, ?
            )
            RETURNING *
            "#
        )
        .bind(request.company_id)
        .bind(request.branch_id)
        .bind(request.supplier_id)
        .bind(&po_number)
        .bind(request.po_date.unwrap_or_else(|| chrono::Utc::now().date_naive()))
        .bind(request.expected_delivery_date)
        .bind(PurchaseStatus::Draft)
        .bind(subtotal.to_f64().unwrap_or_default())
        .bind(discount_amount.to_f64().unwrap_or_default())
        .bind(total_tax.to_f64().unwrap_or_default())
        .bind(shipping_amount.to_f64().unwrap_or_default())
        .bind(total_amount.to_f64().unwrap_or_default())
        .bind(request.currency.unwrap_or_else(|| "USD".to_string()))
        .bind(request.exchange_rate.unwrap_or(Decimal::ONE).to_f64().unwrap_or_default())
        .bind(payment_terms)
        .bind(request.shipping_address)
        .bind(request.notes)
        .bind(request.metadata.unwrap_or(serde_json::json!({})))
        .bind(created_by)
        .bind(created_by)
        .fetch_one(&mut *tx)
        .await?;
        
        let po = PurchaseOrder::from(po_sqlite);
        
        let mut items_with_details = Vec::new();
        
        for item_req in request.items {
            let item_details: (String, String, String) = sqlx::query_as(
                "SELECT sku, name, unit_of_measure FROM inventory_items WHERE id = ?"
            )
            .bind(item_req.item_id)
            .fetch_one(&mut *tx)
            .await?;
            
            let tax_pct = item_req.tax_percentage.unwrap_or(Decimal::ZERO);
            
            let po_item_sqlite = sqlx::query_as::<Sqlite, PurchaseOrderItemSqlite>(
                r#"
                INSERT INTO purchase_order_items (
                    po_id, item_id, description, quantity_ordered,
                    quantity_received, unit_cost, tax_percentage, tax_amount
                )
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                RETURNING *
                "#
            )
            .bind(po.id)
            .bind(item_req.item_id)
            .bind(item_req.description)
            .bind(item_req.quantity_ordered)
            .bind(0)
            .bind(item_req.unit_cost.to_f64().unwrap_or_default())
            .bind(tax_pct.to_f64().unwrap_or_default())
            .bind((item_req.unit_cost * Decimal::from(item_req.quantity_ordered) * (tax_pct / Decimal::from(100))).to_f64().unwrap_or_default())
            .fetch_one(&mut *tx)
            .await?;
            
            items_with_details.push(PurchaseOrderItemWithDetails {
                item: PurchaseOrderItem::from(po_item_sqlite),
                item_sku: item_details.0,
                item_name: item_details.1,
                item_unit: item_details.2,
            });
        }
        
        let (supplier_name, supplier_email, supplier_phone): (String, Option<String>, Option<String>) = 
            sqlx::query_as("SELECT name, email, phone FROM suppliers WHERE id = ?")
            .bind(request.supplier_id)
            .fetch_one(&mut *tx)
            .await?;
        
        let branch_name: String = sqlx::query_scalar("SELECT name FROM branches WHERE id = ?")
            .bind(request.branch_id)
            .fetch_one(&mut *tx)
            .await?;
        
        let created_by_username: String = sqlx::query_scalar("SELECT username FROM users WHERE id = ?")
            .bind(created_by)
            .fetch_one(&mut *tx)
            .await?;
        
        tx.commit().await?;
        
        Ok(PurchaseOrderWithItems {
            purchase_order: po,
            items: items_with_details,
            supplier_name,
            supplier_email,
            supplier_phone,
            branch_name,
            created_by_username,
        })
    }
    
    // ===== FIND PURCHASE ORDER BY ID =====
    pub async fn find_by_id_pg(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<PurchaseOrder>, sqlx::Error> {
        let po = sqlx::query_as::<Postgres, PurchaseOrder>(
            "SELECT * FROM purchase_orders WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(po)
    }
    
    pub async fn find_by_id_sqlite(
        pool: &SqlitePool,
        id: Uuid,
    ) -> Result<Option<PurchaseOrder>, sqlx::Error> {
        let po = sqlx::query_as::<Sqlite, PurchaseOrderSqlite>(
            "SELECT * FROM purchase_orders WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(po.map(PurchaseOrder::from))
    }
    
    // ===== GET PURCHASE ORDER WITH ITEMS =====
    pub async fn get_with_items_pg(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<PurchaseOrderWithItems>, sqlx::Error> {
        let po = match Self::find_by_id_pg(pool, id).await? {
            Some(p) => p,
            None => return Ok(None),
        };
        
        let item_rows = sqlx::query_as::<Postgres, PurchaseOrderItemWithDetailsRow>(
            r#"
            SELECT 
                poi.id, poi.po_id, poi.item_id, poi.description,
                poi.quantity_ordered, poi.quantity_received, poi.unit_cost,
                poi.tax_percentage, poi.tax_amount, poi.line_total,
                poi.created_at,
                i.sku as item_sku, i.name as item_name, i.unit_of_measure as item_unit
            FROM purchase_order_items poi
            JOIN inventory_items i ON poi.item_id = i.id
            WHERE poi.po_id = $1
            ORDER BY poi.created_at
            "#
        )
        .bind(id)
        .fetch_all(pool)
        .await?;
        
        let items = item_rows.into_iter().map(|row| {
            PurchaseOrderItemWithDetails {
                item: PurchaseOrderItem {
                    id: row.id, po_id: row.po_id, item_id: row.item_id,
                    description: row.description, quantity_ordered: row.quantity_ordered,
                    quantity_received: row.quantity_received, unit_cost: row.unit_cost,
                    tax_percentage: row.tax_percentage, tax_amount: row.tax_amount,
                    line_total: row.line_total, created_at: row.created_at,
                },
                item_sku: row.item_sku,
                item_name: row.item_name,
                item_unit: row.item_unit,
            }
        }).collect();
        
        let (supplier_name, supplier_email, supplier_phone): (String, Option<String>, Option<String>) = 
            sqlx::query_as("SELECT name, email, phone FROM suppliers WHERE id = $1")
            .bind(po.supplier_id)
            .fetch_one(pool)
            .await?;
        
        let branch_name: String = sqlx::query_scalar("SELECT name FROM branches WHERE id = $1")
            .bind(po.branch_id)
            .fetch_one(pool)
            .await?;
        
        let created_by_username: String = sqlx::query_scalar("SELECT username FROM users WHERE id = $1")
            .bind(po.created_by)
            .fetch_one(pool)
            .await?;
        
        Ok(Some(PurchaseOrderWithItems {
            purchase_order: po,
            items,
            supplier_name,
            supplier_email,
            supplier_phone,
            branch_name,
            created_by_username,
        }))
    }
        // ===== UPDATE PURCHASE ORDER =====
    pub async fn update_pg(
        pool: &PgPool,
        id: Uuid,
        _request: UpdatePurchaseOrderRequest,
        updated_by: Uuid,
    ) -> Result<PurchaseOrder, sqlx::Error> {
        let po = sqlx::query_as::<Postgres, PurchaseOrder>(
            r#"
            UPDATE purchase_orders
            SET updated_by = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING *
            "#
        )
        .bind(updated_by)
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(po)
    }
    
    pub async fn update_sqlite(
        pool: &SqlitePool,
        id: Uuid,
        _request: UpdatePurchaseOrderRequest,
        updated_by: Uuid,
    ) -> Result<PurchaseOrder, sqlx::Error> {
        let po_sqlite = sqlx::query_as::<Sqlite, PurchaseOrderSqlite>(
            r#"
            UPDATE purchase_orders
            SET updated_by = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            RETURNING *
            "#
        )
        .bind(updated_by)
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(PurchaseOrder::from(po_sqlite))
    }
    
    // ===== DELETE PURCHASE ORDER =====
    pub async fn delete_pg(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM purchase_orders WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
    
    pub async fn delete_sqlite(
        pool: &SqlitePool,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM purchase_orders WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
    
    // ===== SUBMIT PURCHASE ORDER =====
    pub async fn submit_pg(
        pool: &PgPool,
        id: Uuid,
        submitted_by: Uuid,
    ) -> Result<PurchaseOrder, sqlx::Error> {
        let po = sqlx::query_as::<Postgres, PurchaseOrder>(
            r#"
            UPDATE purchase_orders
            SET status = $1, updated_by = $2, updated_at = NOW()
            WHERE id = $3 AND status = 'draft'
            RETURNING *
            "#
        )
        .bind(PurchaseStatus::Submitted)
        .bind(submitted_by)
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(po)
    }
    
    pub async fn submit_sqlite(
        pool: &SqlitePool,
        id: Uuid,
        submitted_by: Uuid,
    ) -> Result<PurchaseOrder, sqlx::Error> {
        let po_sqlite = sqlx::query_as::<Sqlite, PurchaseOrderSqlite>(
            r#"
            UPDATE purchase_orders
            SET status = ?, updated_by = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ? AND status = 'draft'
            RETURNING *
            "#
        )
        .bind(PurchaseStatus::Submitted)
        .bind(submitted_by)
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(PurchaseOrder::from(po_sqlite))
    }
    
    // ===== RECEIVE GOODS (simplified) =====
    pub async fn receive_goods_pg(
        pool: &PgPool,
        id: Uuid,
        _request: ReceiveGoodsRequest,
        received_by: Uuid,
    ) -> Result<PurchaseOrder, sqlx::Error> {
        let updated_po = sqlx::query_as::<Postgres, PurchaseOrder>(
            r#"
            UPDATE purchase_orders
            SET status = $1, updated_by = $2, updated_at = NOW()
            WHERE id = $3
            RETURNING *
            "#
        )
        .bind(PurchaseStatus::Received)
        .bind(received_by)
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(updated_po)
    }
    
    pub async fn receive_goods_sqlite(
        pool: &SqlitePool,
        id: Uuid,
        _request: ReceiveGoodsRequest,
        received_by: Uuid,
    ) -> Result<PurchaseOrder, sqlx::Error> {
        let updated_po_sqlite = sqlx::query_as::<Sqlite, PurchaseOrderSqlite>(
            r#"
            UPDATE purchase_orders
            SET status = ?, updated_by = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            RETURNING *
            "#
        )
        .bind(PurchaseStatus::Received)
        .bind(received_by)
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(PurchaseOrder::from(updated_po_sqlite))
    }
    pub async fn get_with_items_sqlite(
        pool: &SqlitePool,
        id: Uuid,
    ) -> Result<Option<PurchaseOrderWithItems>, sqlx::Error> {
        let po = match Self::find_by_id_sqlite(pool, id).await? {
            Some(p) => p,
            None => return Ok(None),
        };
        
        let item_rows = sqlx::query_as::<Sqlite, PurchaseOrderItemWithDetailsRowSqlite>(
            r#"
            SELECT 
                poi.id, poi.po_id, poi.item_id, poi.description,
                poi.quantity_ordered, poi.quantity_received, poi.unit_cost,
                poi.tax_percentage, poi.tax_amount, poi.line_total,
                poi.created_at,
                i.sku as item_sku, i.name as item_name, i.unit_of_measure as item_unit
            FROM purchase_order_items poi
            JOIN inventory_items i ON poi.item_id = i.id
            WHERE poi.po_id = ?
            ORDER BY poi.created_at
            "#
        )
        .bind(id)
        .fetch_all(pool)
        .await?;
        
        let items = item_rows.into_iter().map(PurchaseOrderItemWithDetails::from).collect();
        
        let (supplier_name, supplier_email, supplier_phone): (String, Option<String>, Option<String>) = 
            sqlx::query_as("SELECT name, email, phone FROM suppliers WHERE id = ?")
            .bind(po.supplier_id)
            .fetch_one(pool)
            .await?;
        
        let branch_name: String = sqlx::query_scalar("SELECT name FROM branches WHERE id = ?")
            .bind(po.branch_id)
            .fetch_one(pool)
            .await?;
        
        let created_by_username: String = sqlx::query_scalar("SELECT username FROM users WHERE id = ?")
            .bind(po.created_by)
            .fetch_one(pool)
            .await?;
        
        Ok(Some(PurchaseOrderWithItems {
            purchase_order: po,
            items,
            supplier_name,
            supplier_email,
            supplier_phone,
            branch_name,
            created_by_username,
        }))
    }
    
    // ===== LIST PURCHASE ORDERS BY COMPANY =====
    pub async fn list_by_company_pg(
        pool: &PgPool,
        company_id: Uuid,
        status: Option<PurchaseStatus>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PurchaseOrder>, sqlx::Error> {
        let orders = if let Some(status) = status {
            sqlx::query_as::<Postgres, PurchaseOrder>(
                r#"
                SELECT * FROM purchase_orders
                WHERE company_id = $1 AND status = $2
                ORDER BY po_date DESC, created_at DESC
                LIMIT $3 OFFSET $4
                "#
            )
            .bind(company_id)
            .bind(status)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as::<Postgres, PurchaseOrder>(
                r#"
                SELECT * FROM purchase_orders
                WHERE company_id = $1
                ORDER BY po_date DESC, created_at DESC
                LIMIT $2 OFFSET $3
                "#
            )
            .bind(company_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?
        };
        
        Ok(orders)
    }
    
    pub async fn list_by_company_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
        status: Option<PurchaseStatus>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PurchaseOrder>, sqlx::Error> {
        let orders = if let Some(status) = status {
            sqlx::query_as::<Sqlite, PurchaseOrderSqlite>(
                r#"
                SELECT * FROM purchase_orders
                WHERE company_id = ? AND status = ?
                ORDER BY po_date DESC, created_at DESC
                LIMIT ? OFFSET ?
                "#
            )
            .bind(company_id)
            .bind(status)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as::<Sqlite, PurchaseOrderSqlite>(
                r#"
                SELECT * FROM purchase_orders
                WHERE company_id = ?
                ORDER BY po_date DESC, created_at DESC
                LIMIT ? OFFSET ?
                "#
            )
            .bind(company_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?
        };
        
        Ok(orders.into_iter().map(PurchaseOrder::from).collect())
    }
}

// ===== UNIT TESTS =====
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    
    #[test]
    fn test_purchase_status_serialization() {
        let status = PurchaseStatus::Confirmed;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, r#""confirmed""#);
    }
    
    #[test]
    fn test_create_po_validation() {
        let request = CreatePurchaseOrderRequest {
            company_id: Uuid::new_v4(),
            branch_id: Uuid::new_v4(),
            supplier_id: Uuid::new_v4(),
            po_date: None,
            expected_delivery_date: None,
            items: vec![
                CreatePurchaseOrderItemRequest {
                    item_id: Uuid::new_v4(),
                    description: None,
                    quantity_ordered: 100,
                    unit_cost: Decimal::from(2500),
                    tax_percentage: Some(Decimal::from_str("8.5").unwrap()),
                }
            ],
            discount_amount: None,
            shipping_amount: None,
            currency: None,
            exchange_rate: None,
            payment_terms: None,
            shipping_address: None,
            notes: None,
            metadata: None,
        };
        
        assert!(request.validate().is_ok());
    }
}