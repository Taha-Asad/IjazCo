// src/models/inventory.rs
// Inventory item management models
// Handles inventory items, categories, stock levels, and inventory transactions

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, SqlitePool, Postgres, Sqlite};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;
use rust_decimal::prelude::{ToPrimitive, FromPrimitive};

// ===== INVENTORY ITEM MODEL =====
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct InventoryItem {
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
}

// ===== INVENTORY ITEM SQLITE INTERMEDIATE STRUCT =====
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct InventoryItemSqlite {
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
    pub cost_price: f64,
    pub selling_price: f64,
    pub msrp: Option<f64>,
    pub tax_rate: f64,
    pub weight: Option<f64>,
    pub weight_unit: Option<String>,
    pub dimensions: serde_json::Value,
    pub reorder_level: i32,
    pub reorder_quantity: i32,
    pub max_stock_level: Option<i32>,
    pub lead_time_days: i32,
    pub warranty_period: Option<i32>,
    pub images: serde_json::Value,
    pub specifications: serde_json::Value,
    pub tags: serde_json::Value,
    pub is_active: bool,
    pub is_discontinued: bool,
    pub discontinued_at: Option<DateTime<Utc>>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

impl From<InventoryItemSqlite> for InventoryItem {
    fn from(s: InventoryItemSqlite) -> Self {
        Self {
            id: s.id,
            company_id: s.company_id,
            category_id: s.category_id,
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
            cost_price: Decimal::from_f64(s.cost_price).unwrap_or_default(),
            selling_price: Decimal::from_f64(s.selling_price).unwrap_or_default(),
            msrp: s.msrp.and_then(Decimal::from_f64),
            tax_rate: Decimal::from_f64(s.tax_rate).unwrap_or_default(),
            weight: s.weight.and_then(Decimal::from_f64),
            weight_unit: s.weight_unit,
            dimensions: if s.dimensions.is_null() { Some(serde_json::json!({})) } else { Some(s.dimensions) },
            reorder_level: s.reorder_level,
            reorder_quantity: s.reorder_quantity,
            max_stock_level: s.max_stock_level,
            lead_time_days: s.lead_time_days,
            warranty_period: s.warranty_period,
            images: s.images,
            specifications: s.specifications,
            tags: {
                if let serde_json::Value::Array(arr) = &s.tags {
                    arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()
                } else {
                    vec![]
                }
            },
            is_active: s.is_active,
            is_discontinued: s.is_discontinued,
            discontinued_at: s.discontinued_at,
            metadata: s.metadata,
            created_at: s.created_at,
            updated_at: s.updated_at,
            created_by: s.created_by,
            updated_by: s.updated_by,
        }
    }
}

// ===== CREATE ITEM REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateItemRequest {
    pub category_id: Option<Uuid>,
    
    #[validate(length(min = 1, max = 100))]
    pub sku: String,
    
    #[validate(length(max = 100))]
    pub barcode: Option<String>,
    
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    
    pub description: Option<String>,
    
    #[validate(length(max = 100))]
    pub brand: Option<String>,
    
    pub model_number: Option<String>,
    pub serial_number: Option<String>,
    
    #[validate(length(min = 1, max = 50))]
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
    
    pub images: Option<serde_json::Value>,
    pub specifications: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<serde_json::Value>,
}

// ===== UPDATE ITEM REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateItemRequest {
    pub category_id: Option<Uuid>,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub brand: Option<String>,
    pub model_number: Option<String>,
    pub serial_number: Option<String>,
    pub unit_of_measure: Option<String>,
    pub is_serialized: Option<bool>,
    pub is_batch_tracked: Option<bool>,
    pub cost_price: Option<Decimal>,
    pub selling_price: Option<Decimal>,
    pub msrp: Option<Decimal>,
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
    pub total_on_hand: i64,
    pub total_allocated: i64,
    pub total_available: i64,
}

// ===== INVENTORY ITEM WITH STOCK SQLITE ROW =====
#[derive(Debug, sqlx::FromRow)]
pub struct InventoryItemWithStockRowSqlite {
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
    pub cost_price: f64,
    pub selling_price: f64,
    pub msrp: Option<f64>,
    pub tax_rate: f64,
    pub weight: Option<f64>,
    pub weight_unit: Option<String>,
    pub dimensions: serde_json::Value,
    pub reorder_level: i32,
    pub reorder_quantity: i32,
    pub max_stock_level: Option<i32>,
    pub lead_time_days: i32,
    pub warranty_period: Option<i32>,
    pub images: serde_json::Value,
    pub specifications: serde_json::Value,
    pub tags: serde_json::Value,
    pub is_active: bool,
    pub is_discontinued: bool,
    pub discontinued_at: Option<DateTime<Utc>>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub total_on_hand: i64,
    pub total_allocated: i64,
    pub total_available: i64,
}

impl From<InventoryItemWithStockRowSqlite> for InventoryItemWithStock {
    fn from(row: InventoryItemWithStockRowSqlite) -> Self {
        InventoryItemWithStock {
            item: InventoryItem::from(InventoryItemSqlite {
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
            }),
            total_on_hand: row.total_on_hand,
            total_allocated: row.total_allocated,
            total_available: row.total_available,
        }
    }
}

// ===== CATEGORY MODEL =====
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Category {
    pub id: Uuid,
    pub company_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub is_active: bool,
    pub sort_order: i32,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// ===== CREATE CATEGORY REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateCategoryRequest {
    pub parent_id: Option<Uuid>,
    
    #[validate(length(min = 1, max = 50))]
    pub code: String,
    
    #[validate(length(min = 1, max = 100))]
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
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub is_active: Option<bool>,
    pub sort_order: Option<i32>,
    pub metadata: Option<serde_json::Value>,
}

// ===== INVENTORY ITEM DATABASE OPERATIONS =====
impl InventoryItem {
    // ===== CREATE NEW ITEM (Postgres) =====
    pub async fn create_pg(
        pool: &PgPool,
        request: CreateItemRequest,
        company_id: Uuid,
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
                $1, $2, $3, $4, $5, $6,
                $7, $8, $9, $10,
                $11, $12, $13, $14, $15,
                $16, $17, $18, $19, $20,
                $21, $22, $23, $24,
                $25, $26, $27, $28, $29,
                $30, $31, $32
            )
            RETURNING *
            "#
        )
        .bind(company_id)
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
        company_id: Uuid,
        created_by: Uuid,
    ) -> Result<InventoryItem, sqlx::Error> {
        let id = Uuid::new_v4();
        let item_sqlite = sqlx::query_as::<Sqlite, InventoryItemSqlite>(
            r#"
            INSERT INTO inventory_items (
                id, company_id, category_id, sku, barcode, name, description,
                brand, model_number, serial_number, unit_of_measure,
                is_serialized, is_batch_tracked, cost_price, selling_price, msrp,
                tax_rate, weight, weight_unit, dimensions, reorder_level,
                reorder_quantity, max_stock_level, lead_time_days, warranty_period,
                images, specifications, tags, is_active, is_discontinued,
                metadata, created_by, updated_by
            )
            VALUES (
                ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?,
                ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?,
                ?, ?, ?, ?,
                ?, ?, ?, ?,
                ?, ?, ?, ?
            )
            RETURNING *
            "#
        )
        .bind(id)
        .bind(company_id)
        .bind(request.category_id)
        .bind(&request.sku)
        .bind(&request.barcode)
        .bind(&request.name)
        .bind(&request.description)
        .bind(&request.brand)
        .bind(&request.model_number)
        .bind(&request.serial_number)
        .bind(&request.unit_of_measure)
        .bind(request.is_serialized)
        .bind(request.is_batch_tracked)
        .bind(request.cost_price.to_f64().unwrap_or_default())
        .bind(request.selling_price.to_f64().unwrap_or_default())
        .bind(request.msrp.map(|d| d.to_f64().unwrap_or_default()))
        .bind(request.tax_rate.to_f64().unwrap_or_default())
        .bind(request.weight.map(|d| d.to_f64().unwrap_or_default()))
        .bind(&request.weight_unit)
        .bind(request.dimensions.unwrap_or(serde_json::json!({})))
        .bind(request.reorder_level)
        .bind(request.reorder_quantity)
        .bind(request.max_stock_level)
        .bind(request.lead_time_days)
        .bind(request.warranty_period)
        .bind(serde_json::json!(request.images.unwrap_or(serde_json::json!([]))))
        .bind(serde_json::json!(request.specifications.unwrap_or(serde_json::json!({}))))
        .bind(serde_json::json!(request.tags.unwrap_or_default()))
        .bind(true)
        .bind(false)
        .bind(request.metadata.unwrap_or(serde_json::json!({})))
        .bind(created_by)
        .bind(created_by)
        .fetch_one(pool)
        .await?;
        
        Ok(InventoryItem::from(item_sqlite))
    }
    
    // ===== FIND ITEM BY ID (Postgres) =====
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
    
    // ===== FIND ITEM BY ID (SQLite) =====
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
    
    // ===== FIND ITEM BY SKU (Postgres) =====
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
    
    // ===== FIND ITEM BY SKU (SQLite) =====
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
        
        Ok(item.map(InventoryItem::from))
    }
    
    // ===== LIST ITEMS BY COMPANY (Postgres) =====
    pub async fn list_by_company_pg(
        pool: &PgPool,
        company_id: Uuid,
        active_only: bool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<InventoryItem>, sqlx::Error> {
        let items = sqlx::query_as::<Postgres, InventoryItem>(
            r#"
            SELECT * FROM inventory_items 
            WHERE company_id = $1 AND ($2 = false OR is_active = true)
            ORDER BY name
            LIMIT $3 OFFSET $4
            "#
        )
        .bind(company_id)
        .bind(active_only)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(items)
    }
    
    // ===== LIST ITEMS BY COMPANY (SQLite) =====
    pub async fn list_by_company_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
        active_only: bool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<InventoryItem>, sqlx::Error> {
        let items = if active_only {
            sqlx::query_as::<Sqlite, InventoryItemSqlite>(
                r#"
                SELECT * FROM inventory_items 
                WHERE company_id = ? AND is_active = 1
                ORDER BY name
                LIMIT ? OFFSET ?
                "#
            )
            .bind(company_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as::<Sqlite, InventoryItemSqlite>(
                r#"
                SELECT * FROM inventory_items 
                WHERE company_id = ?
                ORDER BY name
                LIMIT ? OFFSET ?
                "#
            )
            .bind(company_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?
        };
        
        Ok(items.into_iter().map(InventoryItem::from).collect())
    }
    
    // ===== SEARCH ITEMS (Postgres) =====
    pub async fn search_pg(
        pool: &PgPool,
        company_id: Uuid,
        search_term: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<InventoryItem>, sqlx::Error> {
        let search_pattern = format!("%{}%", search_term);
        let items = sqlx::query_as::<Postgres, InventoryItem>(
            r#"
            SELECT * FROM inventory_items 
            WHERE company_id = $1 
            AND (sku ILIKE $2 OR name ILIKE $2 OR barcode ILIKE $2)
            ORDER BY name
            LIMIT $3 OFFSET $4
            "#
        )
        .bind(company_id)
        .bind(search_pattern)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(items)
    }
    
    // ===== SEARCH ITEMS (SQLite) =====
    pub async fn search_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
        search_term: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<InventoryItem>, sqlx::Error> {
        let search_pattern = format!("%{}%", search_term);
        let items = sqlx::query_as::<Sqlite, InventoryItemSqlite>(
            r#"
            SELECT * FROM inventory_items 
            WHERE company_id = ? 
            AND (sku LIKE ? OR name LIKE ? OR barcode LIKE ?)
            ORDER BY name
            LIMIT ? OFFSET ?
            "#
        )
        .bind(company_id)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(items.into_iter().map(InventoryItem::from).collect())
    }
    
    // ===== LIST BY CATEGORY (Postgres) =====
    pub async fn list_by_category_pg(
        pool: &PgPool,
        company_id: Uuid,
        category_id: Uuid,
        active_only: bool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<InventoryItem>, sqlx::Error> {
        let items = sqlx::query_as::<Postgres, InventoryItem>(
            r#"
            SELECT * FROM inventory_items 
            WHERE company_id = $1 AND category_id = $2 
            AND ($3 = false OR is_active = true)
            ORDER BY name
            LIMIT $4 OFFSET $5
            "#
        )
        .bind(company_id)
        .bind(category_id)
        .bind(active_only)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(items)
    }
    
    // ===== LIST BY CATEGORY (SQLite) =====
    pub async fn list_by_category_sqlite(
        pool: &SqlitePool,
        category_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<InventoryItem>, sqlx::Error> {
        let items = sqlx::query_as::<Sqlite, InventoryItemSqlite>(
            r#"
            SELECT * FROM inventory_items 
            WHERE category_id = ? AND is_active = 1
            ORDER BY name
            LIMIT ? OFFSET ?
            "#
        )
        .bind(category_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(items.into_iter().map(InventoryItem::from).collect())
    }
    
    // ===== UPDATE ITEM (Postgres) =====
    pub async fn update_pg(
        pool: &PgPool,
        id: Uuid,
        request: UpdateItemRequest,
        updated_by: Uuid,
    ) -> Result<InventoryItem, sqlx::Error> {
        let mut builder = sqlx::QueryBuilder::<Postgres>::new("UPDATE inventory_items SET ");
        builder.push("updated_by = ").push_bind(updated_by);
        builder.push(", updated_at = NOW()");
        
        if request.category_id.is_some() { builder.push(", category_id = ").push_bind(request.category_id); }
        if request.sku.is_some() { builder.push(", sku = ").push_bind(request.sku); }
        if request.barcode.is_some() { builder.push(", barcode = ").push_bind(request.barcode); }
        if request.name.is_some() { builder.push(", name = ").push_bind(request.name); }
        if request.description.is_some() { builder.push(", description = ").push_bind(request.description); }
        if request.brand.is_some() { builder.push(", brand = ").push_bind(request.brand); }
        if request.model_number.is_some() { builder.push(", model_number = ").push_bind(request.model_number); }
        if request.serial_number.is_some() { builder.push(", serial_number = ").push_bind(request.serial_number); }
        if request.unit_of_measure.is_some() { builder.push(", unit_of_measure = ").push_bind(request.unit_of_measure); }
        if request.is_serialized.is_some() { builder.push(", is_serialized = ").push_bind(request.is_serialized); }
        if request.is_batch_tracked.is_some() { builder.push(", is_batch_tracked = ").push_bind(request.is_batch_tracked); }
        if request.cost_price.is_some() { builder.push(", cost_price = ").push_bind(request.cost_price); }
        if request.selling_price.is_some() { builder.push(", selling_price = ").push_bind(request.selling_price); }
        if request.msrp.is_some() { builder.push(", msrp = ").push_bind(request.msrp); }
        if request.tax_rate.is_some() { builder.push(", tax_rate = ").push_bind(request.tax_rate); }
        if request.weight.is_some() { builder.push(", weight = ").push_bind(request.weight); }
        if request.weight_unit.is_some() { builder.push(", weight_unit = ").push_bind(request.weight_unit); }
        if request.dimensions.is_some() { builder.push(", dimensions = ").push_bind(request.dimensions); }
        if request.reorder_level.is_some() { builder.push(", reorder_level = ").push_bind(request.reorder_level); }
        if request.reorder_quantity.is_some() { builder.push(", reorder_quantity = ").push_bind(request.reorder_quantity); }
        if request.max_stock_level.is_some() { builder.push(", max_stock_level = ").push_bind(request.max_stock_level); }
        if request.lead_time_days.is_some() { builder.push(", lead_time_days = ").push_bind(request.lead_time_days); }
        if request.warranty_period.is_some() { builder.push(", warranty_period = ").push_bind(request.warranty_period); }
        if request.images.is_some() { builder.push(", images = ").push_bind(request.images); }
        if request.specifications.is_some() { builder.push(", specifications = ").push_bind(request.specifications); }
        if request.tags.is_some() { builder.push(", tags = ").push_bind(request.tags); }
        if request.is_active.is_some() { builder.push(", is_active = ").push_bind(request.is_active); }
        if request.is_discontinued.is_some() { builder.push(", is_discontinued = ").push_bind(request.is_discontinued); }
        if request.metadata.is_some() { builder.push(", metadata = ").push_bind(request.metadata); }
        
        builder.push(" WHERE id = ").push_bind(id);
        builder.push(" RETURNING *");
        builder.build_query_as::<InventoryItem>().fetch_one(pool).await
    }
    
    // ===== UPDATE ITEM (SQLite) =====
    pub async fn update_sqlite(
        pool: &SqlitePool,
        id: Uuid,
        request: UpdateItemRequest,
        updated_by: Uuid,
    ) -> Result<InventoryItem, sqlx::Error> {
        let mut builder = sqlx::QueryBuilder::<Sqlite>::new("UPDATE inventory_items SET ");
        builder.push("updated_by = ").push_bind(updated_by);
        builder.push(", updated_at = CURRENT_TIMESTAMP");
        
        if request.category_id.is_some() { builder.push(", category_id = ").push_bind(request.category_id); }
        if request.sku.is_some() { builder.push(", sku = ").push_bind(request.sku); }
        if request.barcode.is_some() { builder.push(", barcode = ").push_bind(request.barcode); }
        if request.name.is_some() { builder.push(", name = ").push_bind(request.name); }
        if request.description.is_some() { builder.push(", description = ").push_bind(request.description); }
        if request.brand.is_some() { builder.push(", brand = ").push_bind(request.brand); }
        if request.model_number.is_some() { builder.push(", model_number = ").push_bind(request.model_number); }
        if request.serial_number.is_some() { builder.push(", serial_number = ").push_bind(request.serial_number); }
        if request.unit_of_measure.is_some() { builder.push(", unit_of_measure = ").push_bind(request.unit_of_measure); }
        if request.is_serialized.is_some() { builder.push(", is_serialized = ").push_bind(request.is_serialized); }
        if request.is_batch_tracked.is_some() { builder.push(", is_batch_tracked = ").push_bind(request.is_batch_tracked); }
        if request.cost_price.is_some() { builder.push(", cost_price = ").push_bind(request.cost_price.map(|d| d.to_f64().unwrap_or_default())); }
        if request.selling_price.is_some() { builder.push(", selling_price = ").push_bind(request.selling_price.map(|d| d.to_f64().unwrap_or_default())); }
        if request.msrp.is_some() { builder.push(", msrp = ").push_bind(request.msrp.map(|d| d.to_f64().unwrap_or_default())); }
        if request.tax_rate.is_some() { builder.push(", tax_rate = ").push_bind(request.tax_rate.map(|d| d.to_f64().unwrap_or_default())); }
        if request.weight.is_some() { builder.push(", weight = ").push_bind(request.weight.map(|d| d.to_f64().unwrap_or_default())); }
        if request.weight_unit.is_some() { builder.push(", weight_unit = ").push_bind(request.weight_unit); }
        if request.dimensions.is_some() { builder.push(", dimensions = ").push_bind(request.dimensions); }
        if request.reorder_level.is_some() { builder.push(", reorder_level = ").push_bind(request.reorder_level); }
        if request.reorder_quantity.is_some() { builder.push(", reorder_quantity = ").push_bind(request.reorder_quantity); }
        if request.max_stock_level.is_some() { builder.push(", max_stock_level = ").push_bind(request.max_stock_level); }
        if request.lead_time_days.is_some() { builder.push(", lead_time_days = ").push_bind(request.lead_time_days); }
        if request.warranty_period.is_some() { builder.push(", warranty_period = ").push_bind(request.warranty_period); }
        if request.images.is_some() { builder.push(", images = ").push_bind(serde_json::json!(request.images)); }
        if request.specifications.is_some() { builder.push(", specifications = ").push_bind(serde_json::json!(request.specifications)); }
        if request.tags.is_some() { builder.push(", tags = ").push_bind(serde_json::json!(request.tags)); }
        if request.is_active.is_some() { builder.push(", is_active = ").push_bind(if request.is_active.unwrap_or(true) { 1 } else { 0 }); }
        if request.is_discontinued.is_some() { builder.push(", is_discontinued = ").push_bind(if request.is_discontinued.unwrap_or(false) { 1 } else { 0 }); }
        if request.metadata.is_some() { builder.push(", metadata = ").push_bind(request.metadata); }
        
        builder.push(" WHERE id = ").push_bind(id);
        builder.push(" RETURNING *");
        
        let item_sqlite = builder.build_query_as::<InventoryItemSqlite>().fetch_one(pool).await?;
        Ok(InventoryItem::from(item_sqlite))
    }
    
    // ===== DELETE ITEM (SOFT) (Postgres) =====
    pub async fn delete_pg(pool: &PgPool, id: Uuid, updated_by: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE inventory_items SET is_active = false, updated_by = $1, updated_at = NOW() WHERE id = $2")
            .bind(updated_by)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
    
    // ===== DELETE ITEM (SOFT) (SQLite) =====
    pub async fn delete_sqlite(pool: &SqlitePool, id: Uuid, updated_by: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE inventory_items SET is_active = 0, updated_by = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(updated_by)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
    
    // ===== GET WITH STOCK (Postgres) =====
    pub async fn get_with_stock_pg(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<InventoryItemWithStock>, sqlx::Error> {
        #[derive(Debug, sqlx::FromRow)]
        struct Row {
            id: Uuid,
            company_id: Uuid,
            category_id: Option<Uuid>,
            sku: String,
            barcode: Option<String>,
            name: String,
            description: Option<String>,
            brand: Option<String>,
            model_number: Option<String>,
            serial_number: Option<String>,
            unit_of_measure: String,
            is_serialized: bool,
            is_batch_tracked: bool,
            cost_price: Decimal,
            selling_price: Decimal,
            msrp: Option<Decimal>,
            tax_rate: Decimal,
            weight: Option<Decimal>,
            weight_unit: Option<String>,
            dimensions: serde_json::Value,
            reorder_level: i32,
            reorder_quantity: i32,
            max_stock_level: Option<i32>,
            lead_time_days: i32,
            warranty_period: Option<i32>,
            images: serde_json::Value,
            specifications: serde_json::Value,
            tags: Vec<String>,
            is_active: bool,
            is_discontinued: bool,
            discontinued_at: Option<DateTime<Utc>>,
            metadata: serde_json::Value,
            created_at: DateTime<Utc>,
            updated_at: DateTime<Utc>,
            created_by: Option<Uuid>,
            updated_by: Option<Uuid>,
            total_on_hand: i64,
            total_allocated: i64,
            total_available: i64,
        }
        
        let row = sqlx::query_as::<Postgres, Row>(
            r#"
            SELECT 
                ii.*,
                COALESCE(SUM(il.quantity_on_hand), 0) as total_on_hand,
                COALESCE(SUM(il.quantity_allocated), 0) as total_allocated,
                COALESCE(SUM(il.quantity_on_hand - il.quantity_allocated), 0) as total_available
            FROM inventory_items ii
            LEFT JOIN inventory_locations il ON ii.id = il.inventory_item_id
            WHERE ii.id = $1
            GROUP BY ii.id
            "#
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(row.map(|r| InventoryItemWithStock {
            item: InventoryItem {
                id: r.id,
                company_id: r.company_id,
                category_id: r.category_id,
                sku: r.sku,
                barcode: r.barcode,
                name: r.name,
                description: r.description,
                brand: r.brand,
                model_number: r.model_number,
                serial_number: r.serial_number,
                unit_of_measure: r.unit_of_measure,
                is_serialized: r.is_serialized,
                is_batch_tracked: r.is_batch_tracked,
                cost_price: r.cost_price,
                selling_price: r.selling_price,
                msrp: r.msrp,
                tax_rate: r.tax_rate,
                weight: r.weight,
                weight_unit: r.weight_unit,
                dimensions: if r.dimensions.is_null() { Some(serde_json::json!({})) } else { Some(r.dimensions) },
                reorder_level: r.reorder_level,
                reorder_quantity: r.reorder_quantity,
                max_stock_level: r.max_stock_level,
                lead_time_days: r.lead_time_days,
                warranty_period: r.warranty_period,
                images: r.images,
                specifications: r.specifications,
                tags: r.tags,
                is_active: r.is_active,
                is_discontinued: r.is_discontinued,
                discontinued_at: r.discontinued_at,
                metadata: r.metadata,
                created_at: r.created_at,
                updated_at: r.updated_at,
                created_by: r.created_by,
                updated_by: r.updated_by,
            },
            total_on_hand: r.total_on_hand,
            total_allocated: r.total_allocated,
            total_available: r.total_available,
        }))
    }
    
    // ===== GET WITH STOCK (SQLite) =====
    pub async fn get_with_stock_sqlite(
        pool: &SqlitePool,
        id: Uuid,
    ) -> Result<Option<InventoryItemWithStock>, sqlx::Error> {
        let row = sqlx::query_as::<Sqlite, InventoryItemWithStockRowSqlite>(
            r#"
            SELECT 
                ii.*,
                COALESCE(SUM(il.quantity_on_hand), 0) as total_on_hand,
                COALESCE(SUM(il.quantity_allocated), 0) as total_allocated,
                COALESCE(SUM(il.quantity_on_hand - il.quantity_allocated), 0) as total_available
            FROM inventory_items ii
            LEFT JOIN inventory_locations il ON ii.id = il.inventory_item_id
            WHERE ii.id = ?
            GROUP BY ii.id
            "#
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(row.map(InventoryItemWithStock::from))
    }
    
    // ===== GET LOW STOCK ITEMS (Postgres) =====
    pub async fn get_low_stock_items_pg(
        pool: &PgPool,
        company_id: Uuid,
    ) -> Result<Vec<InventoryItemWithStock>, sqlx::Error> {
        #[derive(Debug, sqlx::FromRow)]
        struct Row {
            id: Uuid,
            company_id: Uuid,
            category_id: Option<Uuid>,
            sku: String,
            barcode: Option<String>,
            name: String,
            description: Option<String>,
            brand: Option<String>,
            model_number: Option<String>,
            serial_number: Option<String>,
            unit_of_measure: String,
            is_serialized: bool,
            is_batch_tracked: bool,
            cost_price: Decimal,
            selling_price: Decimal,
            msrp: Option<Decimal>,
            tax_rate: Decimal,
            weight: Option<Decimal>,
            weight_unit: Option<String>,
            dimensions: serde_json::Value,
            reorder_level: i32,
            reorder_quantity: i32,
            max_stock_level: Option<i32>,
            lead_time_days: i32,
            warranty_period: Option<i32>,
            images: serde_json::Value,
            specifications: serde_json::Value,
            tags: Vec<String>,
            is_active: bool,
            is_discontinued: bool,
            discontinued_at: Option<DateTime<Utc>>,
            metadata: serde_json::Value,
            created_at: DateTime<Utc>,
            updated_at: DateTime<Utc>,
            created_by: Option<Uuid>,
            updated_by: Option<Uuid>,
            total_on_hand: i64,
            total_allocated: i64,
            total_available: i64,
        }
        
        let items = sqlx::query_as::<Postgres, Row>(
            r#"
            SELECT 
                ii.*,
                COALESCE(SUM(il.quantity_on_hand), 0) as total_on_hand,
                COALESCE(SUM(il.quantity_allocated), 0) as total_allocated,
                COALESCE(SUM(il.quantity_on_hand - il.quantity_allocated), 0) as total_available
            FROM inventory_items ii
            LEFT JOIN inventory_locations il ON ii.id = il.inventory_item_id
            WHERE ii.company_id = $1 AND ii.is_active = true
            GROUP BY ii.id
            HAVING COALESCE(SUM(il.quantity_on_hand - il.quantity_allocated), 0) <= ii.reorder_level
            ORDER BY ii.name
            "#
        )
        .bind(company_id)
        .fetch_all(pool)
        .await?;
        
        Ok(items.into_iter().map(|r| InventoryItemWithStock {
            item: InventoryItem {
                id: r.id,
                company_id: r.company_id,
                category_id: r.category_id,
                sku: r.sku,
                barcode: r.barcode,
                name: r.name,
                description: r.description,
                brand: r.brand,
                model_number: r.model_number,
                serial_number: r.serial_number,
                unit_of_measure: r.unit_of_measure,
                is_serialized: r.is_serialized,
                is_batch_tracked: r.is_batch_tracked,
                cost_price: r.cost_price,
                selling_price: r.selling_price,
                msrp: r.msrp,
                tax_rate: r.tax_rate,
                weight: r.weight,
                weight_unit: r.weight_unit,
                dimensions: if r.dimensions.is_null() { Some(serde_json::json!({})) } else { Some(r.dimensions) },
                reorder_level: r.reorder_level,
                reorder_quantity: r.reorder_quantity,
                max_stock_level: r.max_stock_level,
                lead_time_days: r.lead_time_days,
                warranty_period: r.warranty_period,
                images: r.images,
                specifications: r.specifications,
                tags: r.tags,
                is_active: r.is_active,
                is_discontinued: r.is_discontinued,
                discontinued_at: r.discontinued_at,
                metadata: r.metadata,
                created_at: r.created_at,
                updated_at: r.updated_at,
                created_by: r.created_by,
                updated_by: r.updated_by,
            },
            total_on_hand: r.total_on_hand,
            total_allocated: r.total_allocated,
            total_available: r.total_available,
        }).collect())
    }
    
    // ===== GET LOW STOCK ITEMS (SQLite) =====
    pub async fn get_low_stock_items_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
    ) -> Result<Vec<InventoryItemWithStock>, sqlx::Error> {
        let items = sqlx::query_as::<Sqlite, InventoryItemWithStockRowSqlite>(
            r#"
            SELECT 
                ii.*,
                COALESCE(SUM(il.quantity_on_hand), 0) as total_on_hand,
                COALESCE(SUM(il.quantity_allocated), 0) as total_allocated,
                COALESCE(SUM(il.quantity_on_hand - il.quantity_allocated), 0) as total_available
            FROM inventory_items ii
            LEFT JOIN inventory_locations il ON ii.id = il.inventory_item_id
            WHERE ii.company_id = ? AND ii.is_active = 1
            GROUP BY ii.id
            HAVING COALESCE(SUM(il.quantity_on_hand - il.quantity_allocated), 0) <= ii.reorder_level
            ORDER BY ii.name
            "#
        )
        .bind(company_id)
        .fetch_all(pool)
        .await?;
        
        Ok(items.into_iter().map(InventoryItemWithStock::from).collect())
    }
}

// ===== CATEGORY DATABASE OPERATIONS =====
impl Category {
    // ===== CREATE CATEGORY (Postgres) =====
    pub async fn create_pg(
        pool: &PgPool,
        request: CreateCategoryRequest,
        company_id: Uuid,
        created_by: Uuid,
    ) -> Result<Category, sqlx::Error> {
        let category = sqlx::query_as::<Postgres, Category>(
            r#"
            INSERT INTO categories (
                company_id, parent_id, code, name, description,
                image_url, is_active, sort_order, metadata,
                created_by, updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#
        )
        .bind(company_id)
        .bind(request.parent_id)
        .bind(request.code)
        .bind(request.name)
        .bind(request.description)
        .bind(request.image_url)
        .bind(true)
        .bind(request.sort_order.unwrap_or(0))
        .bind(request.metadata.unwrap_or(serde_json::json!({})))
        .bind(created_by)
        .bind(created_by)
        .fetch_one(pool)
        .await?;
        
        Ok(category)
    }
    
    // ===== FIND CATEGORY BY ID (Postgres) =====
    pub async fn find_by_id_pg(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<Category>, sqlx::Error> {
        let category = sqlx::query_as::<Postgres, Category>(
            "SELECT * FROM categories WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(category)
    }
    
    // ===== LIST CATEGORIES BY COMPANY (Postgres) =====
    pub async fn list_by_company_pg(
        pool: &PgPool,
        company_id: Uuid,
        active_only: bool,
    ) -> Result<Vec<Category>, sqlx::Error> {
        let categories = sqlx::query_as::<Postgres, Category>(
            r#"
            SELECT * FROM categories 
            WHERE company_id = $1 AND ($2 = false OR is_active = true)
            ORDER BY sort_order, name
            "#
        )
        .bind(company_id)
        .bind(active_only)
        .fetch_all(pool)
        .await?;
        
        Ok(categories)
    }
    
    // ===== UPDATE CATEGORY (Postgres) =====
    pub async fn update_pg(
        pool: &PgPool,
        id: Uuid,
        request: UpdateCategoryRequest,
        updated_by: Uuid,
    ) -> Result<Category, sqlx::Error> {
        let mut builder = sqlx::QueryBuilder::<Postgres>::new("UPDATE categories SET ");
        builder.push("updated_by = ").push_bind(updated_by);
        builder.push(", updated_at = NOW()");
        
        if request.parent_id.is_some() { builder.push(", parent_id = ").push_bind(request.parent_id); }
        if request.code.is_some() { builder.push(", code = ").push_bind(request.code); }
        if request.name.is_some() { builder.push(", name = ").push_bind(request.name); }
        if request.description.is_some() { builder.push(", description = ").push_bind(request.description); }
        if request.image_url.is_some() { builder.push(", image_url = ").push_bind(request.image_url); }
        if request.is_active.is_some() { builder.push(", is_active = ").push_bind(request.is_active); }
        if request.sort_order.is_some() { builder.push(", sort_order = ").push_bind(request.sort_order); }
        if request.metadata.is_some() { builder.push(", metadata = ").push_bind(request.metadata); }
        
        builder.push(" WHERE id = ").push_bind(id);
        builder.push(" RETURNING *");
        builder.build_query_as::<Category>().fetch_one(pool).await
    }
    
    // ===== DELETE CATEGORY (SOFT) (Postgres) =====
    pub async fn delete_pg(pool: &PgPool, id: Uuid, updated_by: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE categories SET is_active = false, updated_by = $1, updated_at = NOW() WHERE id = $2")
            .bind(updated_by)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
