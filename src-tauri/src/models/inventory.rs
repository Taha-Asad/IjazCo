// src/models/inventory.rs
// Inventory item models for product catalog management
// Handles inventory items, categories, and product specifications

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Postgres, Sqlite, SqlitePool};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;
use sqlx::types::Decimal;
use rust_decimal::prelude::ToPrimitive;

// ===== INVENTORY ITEM MODEL =====
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct InventoryItem {
    pub id: Uuid,
    pub company_id: Uuid,
    pub category_id: Option<Uuid>,
    
    #[schema(example = "SKU-MICRO-001")]
    pub sku: String,
    
    #[schema(example = "7891234567890")]
    pub barcode: Option<String>,
    
    #[schema(example = "Professional Compound Microscope 1000X")]
    pub name: String,
    
    pub description: Option<String>,
    
    #[schema(example = "Zeiss")]
    pub brand: Option<String>,
    
    #[schema(example = "Axioscope 5")]
    pub model_number: Option<String>,
    
    pub serial_number: Option<String>,
    
    #[schema(example = "PCS")]
    pub unit_of_measure: String,
    
    pub is_serialized: bool,
    pub is_batch_tracked: bool,
    
    #[schema(value_type = f64, example = 2500.00)]
    pub cost_price: Decimal,
    
    #[schema(value_type = f64, example = 4999.00)]
    pub selling_price: Decimal,
    
    #[schema(value_type = f64, example = 5499.00)]
    pub msrp: Option<Decimal>,
    
    #[schema(value_type = f64, example = 8.5)]
    pub tax_rate: Decimal,
    
    #[schema(value_type = f64, example = 12.5)]
    pub weight: Option<Decimal>,
    
    #[schema(example = "KG")]
    pub weight_unit: Option<String>,
    
    #[sqlx(default)]
    #[schema(value_type = Object, example = json!({"length": 30, "width": 20, "height": 40, "unit": "cm"}))]
    pub dimensions: Option<serde_json::Value>,
    
    #[schema(example = 10)]
    pub reorder_level: i32,
    
    #[schema(example = 50)]
    pub reorder_quantity: i32,
    
    pub max_stock_level: Option<i32>,
    
    #[schema(example = 30)]
    pub lead_time_days: i32,
    
    pub warranty_period: Option<i32>,
    
    #[sqlx(default)]
    #[schema(value_type = Vec<String>, example = json!(["https://cdn.example.com/image1.jpg", "https://cdn.example.com/image2.jpg"]))]
    pub images: serde_json::Value,
    
    #[sqlx(default)]
    #[schema(value_type = Object, example = json!({"magnification": "40x to 1000x", "illumination": "LED", "power": "110-240V AC"}))]
    pub specifications: serde_json::Value,
    
    #[sqlx(default)]
    pub tags: Vec<String>,
    
    pub is_active: bool,
    pub is_discontinued: bool,
    pub discontinued_at: Option<DateTime<Utc>>,
    
    #[sqlx(default)]
    pub metadata: serde_json::Value,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// ===== INVENTORY ITEM SQLITE INTERMEDIATE STRUCT =====
// Uses f64 for Decimal fields and handles type differences for SQLite
// UUID fields are String since SQLite stores them as TEXT
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct InventoryItemSqlite {
    pub id: String,
    pub company_id: String,
    pub category_id: Option<String>,
    pub sku: String,
    pub barcode: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub brand: Option<String>,
    pub model_number: Option<String>,
    pub serial_number: Option<String>,
    pub unit_of_measure: String,
    pub is_serialized: bool,
    pub is_batch_tracked: bool,
    
    // Decimal fields stored as f64 in SQLite
    pub cost_price: f64,
    pub selling_price: f64,
    pub msrp: Option<f64>,
    pub tax_rate: f64,
    pub weight: Option<f64>,
    
    pub weight_unit: Option<String>,
    
    // JSON fields remain as serde_json::Value (SQLite stores as TEXT)
    pub dimensions: Option<serde_json::Value>,
    
    pub reorder_level: i32,
    pub reorder_quantity: i32,
    pub max_stock_level: Option<i32>,
    pub lead_time_days: i32,
    pub warranty_period: Option<i32>,
    
    // JSON fields
    pub images: serde_json::Value,
    pub specifications: serde_json::Value,
    
    // Tags stored as JSON in SQLite
    pub tags: sqlx::types::Json<Vec<String>>,
    
    pub is_active: bool,
    pub is_discontinued: bool,
    pub discontinued_at: Option<DateTime<Utc>>,
    
    pub metadata: serde_json::Value,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

// ===== CREATE INVENTORY ITEM REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateItemRequest {
    pub company_id: Uuid,
    pub category_id: Option<Uuid>,
    
    #[validate(length(min = 1, max = 100))]
    #[schema(example = "SKU-MICRO-001")]
    pub sku: String,
    
    #[validate(length(max = 100))]
    pub barcode: Option<String>,
    
    #[validate(length(min = 1, max = 255))]
    #[schema(example = "Professional Microscope")]
    pub name: String,
    
    pub description: Option<String>,
    #[validate(length(max = 100))]
    pub brand: Option<String>,
    #[validate(length(max = 100))]
    pub model_number: Option<String>,
    pub serial_number: Option<String>,
    
    #[validate(length(min = 1, max = 50))]
    #[schema(example = "PCS")]
    pub unit_of_measure: String,
    
    pub is_serialized: bool,
    pub is_batch_tracked: bool,
    
    #[validate(custom = "validate_decimal_non_negative")]
    #[schema(example = 2500.00)]
    pub cost_price: Decimal,
    
    #[validate(custom = "validate_decimal_non_negative")]
    #[schema(example = 4999.00)]
    pub selling_price: Decimal,
    pub msrp: Option<Decimal>,
    
    #[validate(custom = "validate_decimal_percentage")]
    #[schema(example = 8.5)]
    pub tax_rate: Decimal,
    
    pub weight: Option<Decimal>,
    pub weight_unit: Option<String>,
    pub dimensions: Option<serde_json::Value>,
    
    #[validate(range(min = 0))]
    #[schema(example = 10)]
    pub reorder_level: i32,
    
    #[validate(range(min = 1))]
    #[schema(example = 50)]
    pub reorder_quantity: i32,
    
    pub max_stock_level: Option<i32>,
    #[validate(range(min = 0))]
    #[schema(example = 30)]
    pub lead_time_days: i32,
    pub warranty_period: Option<i32>,
    pub images: Option<serde_json::Value>,
    pub specifications: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<serde_json::Value>,
}

// ===== UPDATE INVENTORY ITEM REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateItemRequest {
    pub category_id: Option<Uuid>,
    pub barcode: Option<String>,
    
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub brand: Option<String>,
    pub model_number: Option<String>,
    
    #[validate(custom = "validate_decimal_non_negative")]
    pub cost_price: Option<Decimal>,
    
    #[validate(custom = "validate_decimal_non_negative")]
    pub selling_price: Option<Decimal>,
    pub msrp: Option<Decimal>,
    
    #[validate(custom = "validate_decimal_percentage")]
    pub tax_rate: Option<Decimal>,
    
    pub weight: Option<Decimal>,
    pub weight_unit: Option<String>,
    pub dimensions: Option<serde_json::Value>,
    pub reorder_level: Option<i32>,
    pub reorder_quantity: Option<i32>,
    pub max_stock_level: Option<i32>,
    pub lead_time_days: Option<i32>,
    pub warranty_period: Option<i32>,
    pub images: Option<serde_json::Value>,
    pub specifications: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,
    pub is_active: Option<bool>,
    pub is_discontinued: Option<bool>,
    pub metadata: Option<serde_json::Value>,
}

// ===== INVENTORY ITEM WITH STOCK =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InventoryItemWithStock {
    #[serde(flatten)]
    pub item: InventoryItem,
    
    #[schema(example = 150)]
    pub total_quantity_on_hand: i64,
    
    #[schema(example = 20)]
    pub total_quantity_allocated: i64,
    
    #[schema(example = 130)]
    pub total_quantity_available: i64,
    
    #[schema(value_type = f64, example = 375000.00)]
    pub total_cost_value: Decimal,
    
    #[schema(value_type = f64, example = 749850.00)]
    pub total_selling_value: Decimal,
}

// ===== Postgres Row Struct for Stock Queries =====
#[derive(Debug, sqlx::FromRow)]
pub struct InventoryItemWithStockRow {
    pub id: Uuid,
    pub company_id: Uuid,
    pub category_id: Option<Uuid>,
    pub sku: String,
    pub barcode: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub brand: Option<String>,
    pub model_number: Option<String>,
    pub serial_number: Option<String>,
    pub unit_of_measure: String,
    pub is_serialized: bool,
    pub is_batch_tracked: bool,
    pub cost_price: Decimal,
    pub selling_price: Decimal,
    pub msrp: Option<Decimal>,
    pub tax_rate: Decimal,
    pub weight: Option<Decimal>,
    pub weight_unit: Option<String>,
    pub dimensions: Option<serde_json::Value>,
    pub reorder_level: i32,
    pub reorder_quantity: i32,
    pub max_stock_level: Option<i32>,
    pub lead_time_days: i32,
    pub warranty_period: Option<i32>,
    pub images: serde_json::Value,
    pub specifications: serde_json::Value,
    pub tags: Vec<String>,
    pub is_active: bool,
    pub is_discontinued: bool,
    pub discontinued_at: Option<DateTime<Utc>>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    // Stock fields
    pub total_on_hand: i64,
    pub total_allocated: i64,
    pub total_available: i64,
}

// ===== SQLite Row Struct for Stock Queries =====
#[derive(Debug, sqlx::FromRow)]
pub struct InventoryItemWithStockRowSqlite {
    pub id: String,
    pub company_id: String,
    pub category_id: Option<String>,
    pub sku: String,
    pub barcode: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub brand: Option<String>,
    pub model_number: Option<String>,
    pub serial_number: Option<String>,
    pub unit_of_measure: String,
    pub is_serialized: bool,
    pub is_batch_tracked: bool,
    // SQLite uses f64
    pub cost_price: f64,
    pub selling_price: f64,
    pub msrp: Option<f64>,
    pub tax_rate: f64,
    pub weight: Option<f64>,
    pub weight_unit: Option<String>,
    pub dimensions: Option<serde_json::Value>,
    pub reorder_level: i32,
    pub reorder_quantity: i32,
    pub max_stock_level: Option<i32>,
    pub lead_time_days: i32,
    pub warranty_period: Option<i32>,
    pub images: serde_json::Value,
    pub specifications: serde_json::Value,
    // SQLite stores Vec as JSON
    pub tags: sqlx::types::Json<Vec<String>>,
    pub is_active: bool,
    pub is_discontinued: bool,
    pub discontinued_at: Option<DateTime<Utc>>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    // Stock fields
    pub total_on_hand: i64,
    pub total_allocated: i64,
    pub total_available: i64,
}

// ===== CATEGORY MODEL =====
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Category {
    pub id: Uuid,
    pub company_id: Uuid,
    pub parent_id: Option<Uuid>,
    
    #[schema(example = "CAT-MICRO")]
    pub code: String,
    
    #[schema(example = "Microscopes")]
    pub name: String,
    
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub sort_order: i32,
    pub is_active: bool,
    
    #[sqlx(default)]
    pub metadata: serde_json::Value,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// ===== CREATE CATEGORY REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateCategoryRequest {
    pub company_id: Uuid,
    pub parent_id: Option<Uuid>,
    
    #[validate(length(min = 1, max = 50))]
    #[schema(example = "CAT-MICRO")]
    pub code: String,
    
    #[validate(length(min = 1, max = 255))]
    #[schema(example = "Microscopes")]
    pub name: String,
    
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub sort_order: Option<i32>,
    pub metadata: Option<serde_json::Value>,
}

// ===== UPDATE CATEGORY REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateCategoryRequest {
    pub parent_id: Option<Uuid>,
    
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
    pub metadata: Option<serde_json::Value>,
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

// ===== CONVERSION IMPLEMENTATIONS =====

impl From<InventoryItemSqlite> for InventoryItem {
    fn from(s: InventoryItemSqlite) -> Self {
        Self {
            id: Uuid::parse_str(&s.id).unwrap_or_else(|_| {
                tracing::warn!("Failed to parse UUID from SQLite id: {}", s.id);
                Uuid::nil()
            }),
            company_id: Uuid::parse_str(&s.company_id).unwrap_or_else(|_| {
                tracing::warn!("Failed to parse UUID from SQLite company_id: {}", s.company_id);
                Uuid::nil()
            }),
            category_id: s.category_id.map(|v| Uuid::parse_str(&v).unwrap_or_else(|_| {
                tracing::warn!("Failed to parse UUID from SQLite category_id: {}", v);
                Uuid::nil()
            })),
            sku: s.sku,
            barcode: s.barcode,
            name: s.name,
            description: s.description,
            brand: s.brand,
            model_number: s.model_number,
            serial_number: s.serial_number,
            unit_of_measure: s.unit_of_measure,
            is_serialized: s.is_serialized,
            is_batch_tracked: s.is_batch_tracked,
            
            // Convert f64 back to Decimal for all Decimal fields
            cost_price: Decimal::from_f64_retain(s.cost_price).unwrap_or_default(),
            selling_price: Decimal::from_f64_retain(s.selling_price).unwrap_or_default(),
            msrp: s.msrp.map(|v| Decimal::from_f64_retain(v).unwrap_or_default()),
            tax_rate: Decimal::from_f64_retain(s.tax_rate).unwrap_or_default(),
            weight: s.weight.map(|v| Decimal::from_f64_retain(v).unwrap_or_default()),
            
            weight_unit: s.weight_unit,
            dimensions: s.dimensions,
            reorder_level: s.reorder_level,
            reorder_quantity: s.reorder_quantity,
            max_stock_level: s.max_stock_level,
            lead_time_days: s.lead_time_days,
            warranty_period: s.warranty_period,
            images: s.images,
            specifications: s.specifications,
            
            // Extract Vec<String> from Json wrapper
            tags: s.tags.0,
            
            is_active: s.is_active,
            is_discontinued: s.is_discontinued,
            discontinued_at: s.discontinued_at,
            metadata: s.metadata,
            created_at: s.created_at,
            updated_at: s.updated_at,
            created_by: s.created_by.map(|v| Uuid::parse_str(&v).unwrap_or_else(|_| {
                tracing::warn!("Failed to parse UUID from SQLite created_by: {}", v);
                Uuid::nil()
            })),
            updated_by: s.updated_by.map(|v| Uuid::parse_str(&v).unwrap_or_else(|_| {
                tracing::warn!("Failed to parse UUID from SQLite updated_by: {}", v);
                Uuid::nil()
            })),
        }
    }
}

impl From<InventoryItemWithStockRow> for InventoryItemWithStock {
    fn from(row: InventoryItemWithStockRow) -> Self {
        let total_on_hand = row.total_on_hand as i64;
        let total_cost_value = row.cost_price * Decimal::from(total_on_hand);
        let total_selling_value = row.selling_price * Decimal::from(total_on_hand);
        
        Self {
            item: InventoryItem {
                id: row.id,
                company_id: row.company_id,
                category_id: row.category_id,
                sku: row.sku,
                barcode: row.barcode,
                name: row.name,
                description: row.description,
                brand: row.brand,
                model_number: row.model_number,
                serial_number: row.serial_number,
                unit_of_measure: row.unit_of_measure,
                is_serialized: row.is_serialized,
                is_batch_tracked: row.is_batch_tracked,
                cost_price: row.cost_price,
                selling_price: row.selling_price,
                msrp: row.msrp,
                tax_rate: row.tax_rate,
                weight: row.weight,
                weight_unit: row.weight_unit,
                dimensions: row.dimensions,
                reorder_level: row.reorder_level,
                reorder_quantity: row.reorder_quantity,
                max_stock_level: row.max_stock_level,
                lead_time_days: row.lead_time_days,
                warranty_period: row.warranty_period,
                images: row.images,
                specifications: row.specifications,
                tags: row.tags,
                is_active: row.is_active,
                is_discontinued: row.is_discontinued,
                discontinued_at: row.discontinued_at,
                metadata: row.metadata,
                created_at: row.created_at,
                updated_at: row.updated_at,
                created_by: row.created_by,
                updated_by: row.updated_by,
            },
            total_quantity_on_hand: total_on_hand,
            total_quantity_allocated: row.total_allocated,
            total_quantity_available: row.total_available,
            total_cost_value,
            total_selling_value,
        }
    }
}

impl From<InventoryItemWithStockRowSqlite> for InventoryItemWithStock {
    fn from(row: InventoryItemWithStockRowSqlite) -> Self {
        let total_on_hand = row.total_on_hand;
        let cost_price = Decimal::from_f64_retain(row.cost_price).unwrap_or_default();
        let selling_price = Decimal::from_f64_retain(row.selling_price).unwrap_or_default();
        let total_cost_value = cost_price * Decimal::from(total_on_hand);
        let total_selling_value = selling_price * Decimal::from(total_on_hand);
        
        Self {
            item: InventoryItem {
                id: Uuid::parse_str(&row.id).unwrap_or_else(|_| {
                    tracing::warn!("Failed to parse UUID from SQLite id: {}", row.id);
                    Uuid::nil()
                }),
                company_id: Uuid::parse_str(&row.company_id).unwrap_or_else(|_| {
                    tracing::warn!("Failed to parse UUID from SQLite company_id: {}", row.company_id);
                    Uuid::nil()
                }),
                category_id: row.category_id.map(|v| Uuid::parse_str(&v).unwrap_or_else(|_| {
                    tracing::warn!("Failed to parse UUID from SQLite category_id: {}", v);
                    Uuid::nil()
                })),
                sku: row.sku,
                barcode: row.barcode,
                name: row.name,
                description: row.description,
                brand: row.brand,
                model_number: row.model_number,
                serial_number: row.serial_number,
                unit_of_measure: row.unit_of_measure,
                is_serialized: row.is_serialized,
                is_batch_tracked: row.is_batch_tracked,
                // Convert f64 to Decimal
                cost_price,
                selling_price,
                msrp: row.msrp.map(|v| Decimal::from_f64_retain(v).unwrap_or_default()),
                tax_rate: Decimal::from_f64_retain(row.tax_rate).unwrap_or_default(),
                weight: row.weight.map(|v| Decimal::from_f64_retain(v).unwrap_or_default()),
                weight_unit: row.weight_unit,
                dimensions: row.dimensions,
                reorder_level: row.reorder_level,
                reorder_quantity: row.reorder_quantity,
                max_stock_level: row.max_stock_level,
                lead_time_days: row.lead_time_days,
                warranty_period: row.warranty_period,
                images: row.images,
                specifications: row.specifications,
                // Extract Vec from Json wrapper
                tags: row.tags.0,
                is_active: row.is_active,
                is_discontinued: row.is_discontinued,
                discontinued_at: row.discontinued_at,
                metadata: row.metadata,
                created_at: row.created_at,
                updated_at: row.updated_at,
                created_by: row.created_by.map(|v| Uuid::parse_str(&v).unwrap_or_else(|_| {
                    tracing::warn!("Failed to parse UUID from SQLite created_by: {}", v);
                    Uuid::nil()
                })),
                updated_by: row.updated_by.map(|v| Uuid::parse_str(&v).unwrap_or_else(|_| {
                    tracing::warn!("Failed to parse UUID from SQLite updated_by: {}", v);
                    Uuid::nil()
                })),
            },
            total_quantity_on_hand: total_on_hand,
            total_quantity_allocated: row.total_allocated,
            total_quantity_available: row.total_available,
            total_cost_value,
            total_selling_value,
        }
    }
}

// ===== INVENTORY ITEM DATABASE OPERATIONS =====
impl InventoryItem {
    // ===== CREATE NEW ITEM (Postgres) =====
    pub async fn create_pg(
        pool: &PgPool,
        request: CreateItemRequest,
        created_by: Uuid,
    ) -> Result<InventoryItem, sqlx::Error> {
        let item = sqlx::query_as::<Postgres, InventoryItem>(
            r#"
            INSERT INTO inventory_items (
                company_id, category_id, sku, barcode, name, description,
                brand, model_number, serial_number, unit_of_measure,
                is_serialized, is_batch_tracked, cost_price, selling_price, msrp,
                tax_rate, weight, weight_unit, dimensions, reorder_level,
                reorder_quantity, max_stock_level, lead_time_days, warranty_period,
                images, specifications, tags, is_active, is_discontinued,
                metadata, created_by, updated_by
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
                $21, $22, $23, $24, $25, $26, $27, $28, $29, $30,
                $31, $32
            )
            RETURNING *
            "#
        )
        .bind(request.company_id)
        .bind(request.category_id)
        .bind(request.sku)
        .bind(request.barcode)
        .bind(request.name)
        .bind(request.description)
        .bind(request.brand)
        .bind(request.model_number)
        .bind(request.serial_number)
        .bind(request.unit_of_measure)
        .bind(request.is_serialized)
        .bind(request.is_batch_tracked)
        // Postgres handles Decimal natively
        .bind(request.cost_price)
        .bind(request.selling_price)
        .bind(request.msrp)
        .bind(request.tax_rate)
        .bind(request.weight)
        .bind(request.weight_unit)
        .bind(request.dimensions.unwrap_or(serde_json::json!({})))
        .bind(request.reorder_level)
        .bind(request.reorder_quantity)
        .bind(request.max_stock_level)
        .bind(request.lead_time_days)
        .bind(request.warranty_period)
        .bind(request.images.unwrap_or(serde_json::json!([])))
        .bind(request.specifications.unwrap_or(serde_json::json!({})))
        // Postgres handles Vec<String> natively
        .bind(request.tags.unwrap_or_default())
        .bind(true)
        .bind(false)
        .bind(request.metadata.unwrap_or(serde_json::json!({})))
        .bind(created_by)
        .bind(created_by)
        .fetch_one(pool)
        .await?;
        
        Ok(item)
    }

    // ===== CREATE NEW ITEM (SQLite) =====
    pub async fn create_sqlite(
        pool: &SqlitePool,
        request: CreateItemRequest,
        created_by: Uuid,
    ) -> Result<InventoryItem, sqlx::Error> {
        let item_sqlite = sqlx::query_as::<Sqlite, InventoryItemSqlite>(
            r#"
            INSERT INTO inventory_items (
                company_id, category_id, sku, barcode, name, description,
                brand, model_number, serial_number, unit_of_measure,
                is_serialized, is_batch_tracked, cost_price, selling_price, msrp,
                tax_rate, weight, weight_unit, dimensions, reorder_level,
                reorder_quantity, max_stock_level, lead_time_days, warranty_period,
                images, specifications, tags, is_active, is_discontinued,
                metadata, created_by, updated_by
            )
            VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?
            )
            RETURNING *
            "#
        )
        .bind(request.company_id)
        .bind(request.category_id)
        .bind(request.sku)
        .bind(request.barcode)
        .bind(request.name)
        .bind(request.description)
        .bind(request.brand)
        .bind(request.model_number)
        .bind(request.serial_number)
        .bind(request.unit_of_measure)
        .bind(request.is_serialized)
        .bind(request.is_batch_tracked)
        // SQLite: Convert Decimal to f64
        .bind(request.cost_price.to_f64().unwrap_or_default())
        .bind(request.selling_price.to_f64().unwrap_or_default())
        .bind(request.msrp.map(|v| v.to_f64().unwrap_or_default()))
        .bind(request.tax_rate.to_f64().unwrap_or_default())
        .bind(request.weight.map(|v| v.to_f64().unwrap_or_default()))
        .bind(request.weight_unit)
        .bind(request.dimensions.unwrap_or(serde_json::json!({})))
        .bind(request.reorder_level)
        .bind(request.reorder_quantity)
        .bind(request.max_stock_level)
        .bind(request.lead_time_days)
        .bind(request.warranty_period)
        .bind(request.images.unwrap_or(serde_json::json!([])))
        .bind(request.specifications.unwrap_or(serde_json::json!({})))
        // SQLite: Convert Vec<String> to JSON
        .bind(serde_json::json!(request.tags.unwrap_or_default()))
        .bind(true)
        .bind(false)
        .bind(request.metadata.unwrap_or(serde_json::json!({})))
        .bind(created_by)
        .bind(created_by)
        .fetch_one(pool)
        .await?;
        
        // Convert SQLite intermediate struct back to InventoryItem
        Ok(InventoryItem::from(item_sqlite))
    }

    // ===== FIND ITEM BY ID =====
    pub async fn find_by_id_pg(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<InventoryItem>, sqlx::Error> {
        let item = sqlx::query_as::<Postgres, InventoryItem>(
            "SELECT * FROM inventory_items WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(item)
    }
    
    pub async fn find_by_id_sqlite(
        pool: &SqlitePool,
        id: Uuid,
    ) -> Result<Option<InventoryItem>, sqlx::Error> {
        let item = sqlx::query_as::<Sqlite, InventoryItemSqlite>(
            "SELECT * FROM inventory_items WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(item.map(InventoryItem::from))
    }
    
    // ===== FIND ITEM BY SKU =====
    pub async fn find_by_sku_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
        sku: &str,
    ) -> Result<Option<InventoryItem>, sqlx::Error> {
        let item = sqlx::query_as::<Sqlite, InventoryItemSqlite>(
            "SELECT * FROM inventory_items WHERE company_id = ? AND sku = ?"
        )
        .bind(company_id)
        .bind(sku)
        .fetch_optional(pool)
        .await?;
        
        // Convert Option<InventoryItemSqlite> to Option<InventoryItem>
        Ok(item.map(InventoryItem::from))
    }

    pub async fn find_by_sku_pg(
        pool: &PgPool,
        company_id: Uuid,
        sku: &str,
    ) -> Result<Option<InventoryItem>, sqlx::Error> {
        let item = sqlx::query_as::<Postgres, InventoryItem>(
            "SELECT * FROM inventory_items WHERE company_id = $1 AND sku = $2"
        )
        .bind(company_id)
        .bind(sku)
        .fetch_optional(pool)
        .await?;
        
        Ok(item)
    }

    // ===== LIST ITEMS BY COMPANY =====
    pub async fn list_by_company_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<InventoryItem>, sqlx::Error> {
        let items = sqlx::query_as::<Sqlite, InventoryItemSqlite>(
            "SELECT * FROM inventory_items WHERE company_id = ? AND is_active = true ORDER BY name LIMIT ? OFFSET ?"
        )
        .bind(company_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(items.into_iter().map(InventoryItem::from).collect())
    }
    
    pub async fn list_by_company_pg(
        pool: &PgPool,
        company_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<InventoryItem>, sqlx::Error> {
        let items = sqlx::query_as::<Postgres, InventoryItem>(
            "SELECT * FROM inventory_items WHERE company_id = $1 AND is_active = true ORDER BY name LIMIT $2 OFFSET $3"
        )
        .bind(company_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(items)
    }
    
    // ===== SEARCH ITEMS =====
    pub async fn search_pg(
        pool: &PgPool,
        company_id: Uuid,
        search_term: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<InventoryItem>, sqlx::Error> {
        let pattern = format!("%{}%", search_term);
        let items = sqlx::query_as::<Postgres, InventoryItem>(
            r#"
            SELECT * FROM inventory_items
            WHERE company_id = $1 
              AND is_active = true
              AND (
                name ILIKE $2
                OR sku ILIKE $2
                OR barcode ILIKE $2
                OR brand ILIKE $2
                OR $3 = ANY(tags)
              )
            ORDER BY name
            LIMIT $4 OFFSET $5
            "#
        )
        .bind(company_id)
        .bind(&pattern)
        .bind(search_term)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(items)
    }
    
    pub async fn search_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
        search_term: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<InventoryItem>, sqlx::Error> {
        let pattern = format!("%{}%", search_term);
        let items = sqlx::query_as::<Sqlite, InventoryItemSqlite>(
            r#"
            SELECT * FROM inventory_items
            WHERE company_id = ? 
              AND is_active = true
              AND (
                name LIKE ?
                OR sku LIKE ?
                OR barcode LIKE ?
                OR brand LIKE ?
                OR tags LIKE ?
              )
            ORDER BY name
            LIMIT ? OFFSET ?
            "#
        )
        .bind(company_id)
        .bind(&pattern)  // name LIKE ?
        .bind(&pattern)  // sku LIKE ?
        .bind(&pattern)  // barcode LIKE ?
        .bind(&pattern)  // brand LIKE ?
        .bind(&pattern)  // tags LIKE ? (SQLite stores tags as JSON text)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(items.into_iter().map(InventoryItem::from).collect())
    }
    
    // ===== LIST ITEMS BY CATEGORY =====
    pub async fn list_by_category_pg(
        pool: &PgPool,
        category_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<InventoryItem>, sqlx::Error> {
        let items = sqlx::query_as::<Postgres, InventoryItem>(
            "SELECT * FROM inventory_items WHERE category_id = $1 AND is_active = true ORDER BY name LIMIT $2 OFFSET $3"
        )
        .bind(category_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(items)
    }

    pub async fn list_by_category_sqlite(
        pool: &SqlitePool,
        category_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<InventoryItem>, sqlx::Error> {
        let items = sqlx::query_as::<Sqlite, InventoryItemSqlite>(
            "SELECT * FROM inventory_items WHERE category_id = ? AND is_active = true ORDER BY name LIMIT ? OFFSET ?"
        )
        .bind(category_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(items.into_iter().map(InventoryItem::from).collect())
    }
    
    // ===== UPDATE ITEM =====
    pub async fn update_pg(
        pool: &PgPool,
        id: Uuid,
        request: UpdateItemRequest,
        updated_by: Uuid,
    ) -> Result<InventoryItem, sqlx::Error> {
        let mut builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("UPDATE inventory_items SET ");
        
        builder.push("updated_by = ").push_bind(updated_by);
        builder.push(", updated_at = NOW()");
        
        if let Some(category_id) = request.category_id {
            builder.push(", category_id = ").push_bind(category_id);
        }
        if let Some(name) = &request.name {
            builder.push(", name = ").push_bind(name);
        }
        if let Some(description) = &request.description {
            builder.push(", description = ").push_bind(description);
        }
        if let Some(cost_price) = request.cost_price {
            builder.push(", cost_price = ").push_bind(cost_price);
        }
        if let Some(selling_price) = request.selling_price {
            builder.push(", selling_price = ").push_bind(selling_price);
        }
        if let Some(tax_rate) = request.tax_rate {
            builder.push(", tax_rate = ").push_bind(tax_rate);
        }
        if let Some(reorder_level) = request.reorder_level {
            builder.push(", reorder_level = ").push_bind(reorder_level);
        }
        if let Some(reorder_quantity) = request.reorder_quantity {
            builder.push(", reorder_quantity = ").push_bind(reorder_quantity);
        }
        if let Some(is_active) = request.is_active {
            builder.push(", is_active = ").push_bind(is_active);
        }
        if let Some(is_discontinued) = request.is_discontinued {
            builder.push(", is_discontinued = ").push_bind(is_discontinued);
        }
        
        builder.push(" WHERE id = ").push_bind(id);
        builder.push(" RETURNING *");
        
        let item = builder.build_query_as::<InventoryItem>().fetch_one(pool).await?;
        Ok(item)
    }

    pub async fn update_sqlite(
        pool: &SqlitePool,
        id: Uuid,
        request: UpdateItemRequest,
        updated_by: Uuid,
    ) -> Result<InventoryItem, sqlx::Error> {
        let mut builder = sqlx::QueryBuilder::<sqlx::Sqlite>::new("UPDATE inventory_items SET ");
        
        builder.push("updated_by = ").push_bind(updated_by);
        builder.push(", updated_at = CURRENT_TIMESTAMP");
        
        if let Some(category_id) = request.category_id {
            builder.push(", category_id = ").push_bind(category_id);
        }
        if let Some(name) = &request.name {
            builder.push(", name = ").push_bind(name);
        }
        if let Some(description) = &request.description {
            builder.push(", description = ").push_bind(description);
        }
        // SQLite: Convert Decimal to f64
        if let Some(cost_price) = request.cost_price {
            builder.push(", cost_price = ").push_bind(cost_price.to_f64().unwrap_or_default());
        }
        if let Some(selling_price) = request.selling_price {
            builder.push(", selling_price = ").push_bind(selling_price.to_f64().unwrap_or_default());
        }
        if let Some(tax_rate) = request.tax_rate {
            builder.push(", tax_rate = ").push_bind(tax_rate.to_f64().unwrap_or_default());
        }
        if let Some(reorder_level) = request.reorder_level {
            builder.push(", reorder_level = ").push_bind(reorder_level);
        }
        if let Some(reorder_quantity) = request.reorder_quantity {
            builder.push(", reorder_quantity = ").push_bind(reorder_quantity);
        }
        // SQLite: Convert bool to 0/1
        if let Some(is_active) = request.is_active {
            builder.push(", is_active = ").push_bind(if is_active { 1 } else { 0 });
        }
        if let Some(is_discontinued) = request.is_discontinued {
            builder.push(", is_discontinued = ").push_bind(if is_discontinued { 1 } else { 0 });
        }
        
        builder.push(" WHERE id = ").push_bind(id);
        builder.push(" RETURNING *");
        
        // Query into SQLite intermediate struct, then convert
        let item_sqlite = builder.build_query_as::<InventoryItemSqlite>().fetch_one(pool).await?;
        Ok(InventoryItem::from(item_sqlite))
    }
    
    // ===== DELETE ITEM =====
    pub async fn delete_pg(
        pool: &PgPool,
        id: Uuid,
        deleted_by: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE inventory_items SET is_active = false, updated_by = $1, updated_at = NOW() WHERE id = $2"
        )
        .bind(deleted_by)
        .bind(id)
        .execute(pool)
        .await?;
        
        Ok(())
    }

    pub async fn delete_sqlite(
        pool: &SqlitePool,
        id: Uuid,
        deleted_by: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE inventory_items SET is_active = false, updated_by = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
        )
        .bind(deleted_by)
        .bind(id)
        .execute(pool)
        .await?;
        
        Ok(())
    }
    
    // ===== GET ITEM WITH STOCK SUMMARY =====
    pub async fn get_with_stock_pg(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<InventoryItemWithStock>, sqlx::Error> {
        let result = sqlx::query_as::<Postgres, InventoryItemWithStockRow>(
            r#"
            SELECT 
                i.*,
                COALESCE(SUM(s.quantity_on_hand), 0) as total_on_hand,
                COALESCE(SUM(s.quantity_allocated), 0) as total_allocated,
                COALESCE(SUM(COALESCE(s.quantity_on_hand, 0) - COALESCE(s.quantity_allocated, 0)), 0) as total_available
            FROM inventory_items i
            LEFT JOIN stock s ON i.id = s.item_id
            WHERE i.id = $1
            GROUP BY i.id
            "#
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(result.map(InventoryItemWithStock::from))
    }

    pub async fn get_with_stock_sqlite(
        pool: &SqlitePool,
        id: Uuid,
    ) -> Result<Option<InventoryItemWithStock>, sqlx::Error> {
        let result = sqlx::query_as::<Sqlite, InventoryItemWithStockRowSqlite>(
            r#"
            SELECT 
                i.*,
                COALESCE(SUM(s.quantity_on_hand), 0) as total_on_hand,
                COALESCE(SUM(s.quantity_allocated), 0) as total_allocated,
                COALESCE(SUM(COALESCE(s.quantity_on_hand, 0) - COALESCE(s.quantity_allocated, 0)), 0) as total_available
            FROM inventory_items i
            LEFT JOIN stock s ON i.id = s.item_id
            WHERE i.id = ?
            GROUP BY i.id
            "#
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(result.map(InventoryItemWithStock::from))
    }
    
    // ===== GET LOW STOCK ITEMS =====
    pub async fn get_low_stock_items_pg(
        pool: &PgPool,
        company_id: Uuid,
    ) -> Result<Vec<InventoryItemWithStock>, sqlx::Error> {
        let items = sqlx::query_as::<Postgres, InventoryItemWithStockRow>(
            r#"
            SELECT 
                i.*,
                COALESCE(SUM(s.quantity_on_hand), 0) as total_on_hand,
                COALESCE(SUM(s.quantity_allocated), 0) as total_allocated,
                COALESCE(SUM(COALESCE(s.quantity_on_hand, 0) - COALESCE(s.quantity_allocated, 0)), 0) as total_available
            FROM inventory_items i
            LEFT JOIN stock s ON i.id = s.item_id
            WHERE i.company_id = $1 AND i.is_active = true
            GROUP BY i.id
            HAVING COALESCE(SUM(COALESCE(s.quantity_on_hand, 0) - COALESCE(s.quantity_allocated, 0)), 0) < i.reorder_level
            ORDER BY i.name
            "#
        )
        .bind(company_id)
        .fetch_all(pool)
        .await?;
        
        Ok(items.into_iter().map(InventoryItemWithStock::from).collect())
    }
    
    pub async fn get_low_stock_items_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
    ) -> Result<Vec<InventoryItemWithStock>, sqlx::Error> {
        let items = sqlx::query_as::<Sqlite, InventoryItemWithStockRowSqlite>(
            r#"
            SELECT 
                i.*,
                COALESCE(SUM(s.quantity_on_hand), 0) as total_on_hand,
                COALESCE(SUM(s.quantity_allocated), 0) as total_allocated,
                COALESCE(SUM(COALESCE(s.quantity_on_hand, 0) - COALESCE(s.quantity_allocated, 0)), 0) as total_available
            FROM inventory_items i
            LEFT JOIN stock s ON i.id = s.item_id
            WHERE i.company_id = ? AND i.is_active = true
            GROUP BY i.id
            HAVING COALESCE(SUM(COALESCE(s.quantity_on_hand, 0) - COALESCE(s.quantity_allocated, 0)), 0) < i.reorder_level
            ORDER BY i.name
            "#
        )
        .bind(company_id)
        .fetch_all(pool)
        .await?;
        
        Ok(items.into_iter().map(InventoryItemWithStock::from).collect())
    }
}
// ===== CATEGORY DATABASE OPERATIONS =====
// impl Category {
//     // ===== CREATE CATEGORY =====
//     pub async fn create(
//         pool: &PgPool,
//         request: CreateCategoryRequest,
//         created_by: Uuid,
//     ) -> Result<Category, sqlx::Error> {
//         let category = sqlx::query_as::<_, Category>(
//             r#"
//             INSERT INTO categories (
//                 company_id, parent_id, code, name, description,
//                 image_url, sort_order, is_active, metadata,
//                 created_by, updated_by
//             )
//             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
//             RETURNING *
//             "#
//         )
//         .bind(request.company_id)
//         .bind(request.parent_id)
//         .bind(request.code)
//         .bind(request.name)
//         .bind(request.description)
//         .bind(request.image_url)
//         .bind(request.sort_order.unwrap_or(0))
//         .bind(true)
//         .bind(request.metadata.unwrap_or(serde_json::json!({})))
//         .bind(created_by)
//         .bind(created_by)
//         .fetch_one(pool)
//         .await?;
        
//         Ok(category)
//     }
    
//     // ===== FIND CATEGORY BY ID =====
//     pub async fn find_by_id(
//         pool: &PgPool,
//         id: Uuid,
//     ) -> Result<Option<Category>, sqlx::Error> {
//         let category = sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = $1")
//             .bind(id)
//             .fetch_optional(pool)
//             .await?;
        
//         Ok(category)
//     }
    
//     // ===== LIST CATEGORIES BY COMPANY =====
//     pub async fn list_by_company(
//         pool: &PgPool,
//         company_id: Uuid,
//     ) -> Result<Vec<Category>, sqlx::Error> {
//         let categories = sqlx::query_as::<_, Category>(
//             "SELECT * FROM categories WHERE company_id = $1 AND is_active = true ORDER BY sort_order, name"
//         )
//         .bind(company_id)
//         .fetch_all(pool)
//         .await?;
        
//         Ok(categories)
//     }
    
//     // ===== GET TOP-LEVEL CATEGORIES =====
//     pub async fn get_top_level(
//         pool: &PgPool,
//         company_id: Uuid,
//     ) -> Result<Vec<Category>, sqlx::Error> {
//         let categories = sqlx::query_as::<_, Category>(
//             "SELECT * FROM categories WHERE company_id = $1 AND parent_id IS NULL AND is_active = true ORDER BY sort_order, name"
//         )
//         .bind(company_id)
//         .fetch_all(pool)
//         .await?;
        
//         Ok(categories)
//     }
    
//     // ===== GET SUBCATEGORIES =====
//     pub async fn get_subcategories(
//         pool: &PgPool,
//         parent_id: Uuid,
//     ) -> Result<Vec<Category>, sqlx::Error> {
//         let categories = sqlx::query_as::<_, Category>(
//             "SELECT * FROM categories WHERE parent_id = $1 AND is_active = true ORDER BY sort_order, name"
//         )
//         .bind(parent_id)
//         .fetch_all(pool)
//         .await?;
        
//         Ok(categories)
//     }
    
//     // ===== UPDATE CATEGORY =====
//     pub async fn update(
//         pool: &PgPool,
//         id: Uuid,
//         request: UpdateCategoryRequest,
//         updated_by: Uuid,
//     ) -> Result<Category, sqlx::Error> {
//         let category = sqlx::query_as::<_, Category>(
//             "UPDATE categories SET updated_by = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
//         )
//         .bind(updated_by)
//         .bind(id)
//         .fetch_one(pool)
//         .await?;
        
//         Ok(category)
//     }
    
//     // ===== DELETE CATEGORY =====
//     pub async fn delete(
//         pool: &PgPool,
//         id: Uuid,
//     ) -> Result<(), sqlx::Error> {
//         sqlx::query("DELETE FROM categories WHERE id = $1").bind(id).execute(pool).await?;
//         Ok(())
//     }
// }

// ===== UNIT TESTS =====
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use validator::Validate;
    
    #[test]
    fn test_create_item_request_validation() {
        let request = CreateItemRequest {
            company_id: Uuid::new_v4(),
            category_id: None,
            sku: "SKU-001".to_string(),
            barcode: None,
            name: "Test Item".to_string(),
            description: None,
            brand: None,
            model_number: None,
            serial_number: None,
            unit_of_measure: "PCS".to_string(),
            is_serialized: false,
            is_batch_tracked: false,
            cost_price: Decimal::from(100),
            selling_price: Decimal::from(200),
            msrp: None,
            tax_rate: Decimal::from_str("8.5").unwrap(),
            weight: None,
            weight_unit: None,
            dimensions: None,
            reorder_level: 10,
            reorder_quantity: 50,
            max_stock_level: None,
            lead_time_days: 7,
            warranty_period: None,
            images: None,
            specifications: None,
            tags: None,
            metadata: None,
        };
        
        assert!(request.validate().is_ok());
    }
}