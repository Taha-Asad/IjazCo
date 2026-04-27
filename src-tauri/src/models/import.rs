// src/models/import.rs
// Import order management models
// Handles international shipments with customs, duties, and freight tracking

use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;
use sqlx::types::Decimal;
use sqlx::{PgPool, SqlitePool, Postgres, Sqlite};
use rust_decimal::prelude::ToPrimitive;

// ===== IMPORT ORDER MODEL =====
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ImportOrder {
    pub id: Uuid,
    pub company_id: Uuid,
    pub po_id: Option<Uuid>,
    
    #[schema(example = "IMP-2024-00001")]
    pub import_number: String,
    
    pub supplier_id: Uuid,
    pub shipment_date: Option<chrono::NaiveDate>,
    pub arrival_date: Option<chrono::NaiveDate>,
    pub customs_clearance_date: Option<chrono::NaiveDate>,
    
    #[schema(example = "in_transit")]
    pub status: String,
    
    #[schema(example = "sea")]
    pub shipping_method: Option<String>,
    
    #[schema(example = "TRACK-123456789")]
    pub tracking_number: Option<String>,
    
    #[schema(example = "CONT-ABC123")]
    pub container_number: Option<String>,
    
    #[schema(value_type = f64, example = 5000.00)]
    pub freight_cost: Decimal,
    
    #[schema(value_type = f64, example = 500.00)]
    pub insurance_cost: Decimal,
    
    #[schema(value_type = f64, example = 2500.00)]
    pub customs_duty: Decimal,
    
    #[schema(value_type = f64, example = 300.00)]
    pub other_charges: Decimal,
    
    #[sqlx(default)]
    #[schema(value_type = f64, example = 8300.00)]
    pub total_cost: Decimal,
    
    #[sqlx(default)]
    #[schema(value_type = Vec<Object>, example = json!([
        {"type": "commercial_invoice", "url": "https://cdn.example.com/invoice.pdf"},
        {"type": "bill_of_lading", "url": "https://cdn.example.com/bol.pdf"}
    ]))]
    pub documents: serde_json::Value,
    
    pub notes: Option<String>,
    
    #[sqlx(default)]
    pub metadata: serde_json::Value,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Option<Uuid>,
}

// ===== IMPORT ORDER SQLITE INTERMEDIATE STRUCT =====
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ImportOrderSqlite {
    pub id: Uuid,
    pub company_id: Uuid,
    pub po_id: Option<Uuid>,
    pub import_number: String,
    pub supplier_id: Uuid,
    pub shipment_date: Option<chrono::NaiveDate>,
    pub arrival_date: Option<chrono::NaiveDate>,
    pub customs_clearance_date: Option<chrono::NaiveDate>,
    pub status: String,
    pub shipping_method: Option<String>,
    pub tracking_number: Option<String>,
    pub container_number: Option<String>,
    pub freight_cost: f64,
    pub insurance_cost: f64,
    pub customs_duty: f64,
    pub other_charges: f64,
    pub total_cost: f64,
    pub documents: serde_json::Value,
    pub notes: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Option<Uuid>,
}

// ===== IMPORT ORDER WITH DETAILS =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ImportOrderWithDetails {
    #[serde(flatten)]
    pub import_order: ImportOrder,
    
    pub supplier_name: String,
    pub supplier_country: Option<String>,
    pub po_number: Option<String>,
    pub created_by_username: String,
}

// ===== CREATE IMPORT ORDER REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateImportOrderRequest {
    pub company_id: Uuid,
    pub po_id: Option<Uuid>,
    pub supplier_id: Uuid,
    pub shipment_date: Option<chrono::NaiveDate>,
    pub arrival_date: Option<chrono::NaiveDate>,
    
    #[validate(length(max = 100))]
    pub shipping_method: Option<String>,
    
    #[validate(length(max = 255))]
    pub tracking_number: Option<String>,
    
    #[validate(length(max = 100))]
    pub container_number: Option<String>,
    
    #[validate(custom = "validate_decimal_non_negative")]
    pub freight_cost: Option<Decimal>,
    
    #[validate(custom = "validate_decimal_non_negative")]
    pub insurance_cost: Option<Decimal>,
    
    #[validate(custom = "validate_decimal_non_negative")]
    pub customs_duty: Option<Decimal>,
    
    #[validate(custom = "validate_decimal_non_negative")]
    pub other_charges: Option<Decimal>,
    
    pub documents: Option<serde_json::Value>,
    pub notes: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

// ===== UPDATE IMPORT ORDER REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateImportOrderRequest {
    pub shipment_date: Option<chrono::NaiveDate>,
    pub arrival_date: Option<chrono::NaiveDate>,
    pub customs_clearance_date: Option<chrono::NaiveDate>,
    pub status: Option<String>,
    pub shipping_method: Option<String>,
    pub tracking_number: Option<String>,
    pub container_number: Option<String>,
    
    #[validate(custom = "validate_decimal_non_negative")]
    pub freight_cost: Option<Decimal>,
    
    #[validate(custom = "validate_decimal_non_negative")]
    pub insurance_cost: Option<Decimal>,
    
    #[validate(custom = "validate_decimal_non_negative")]
    pub customs_duty: Option<Decimal>,
    
    #[validate(custom = "validate_decimal_non_negative")]
    pub other_charges: Option<Decimal>,
    
    pub documents: Option<serde_json::Value>,
    pub notes: Option<String>,
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

// ===== CONVERSION IMPLEMENTATION =====
impl From<ImportOrderSqlite> for ImportOrder {
    fn from(s: ImportOrderSqlite) -> Self {
        Self {
            id: s.id,
            company_id: s.company_id,
            po_id: s.po_id,
            import_number: s.import_number,
            supplier_id: s.supplier_id,
            shipment_date: s.shipment_date,
            arrival_date: s.arrival_date,
            customs_clearance_date: s.customs_clearance_date,
            status: s.status,
            shipping_method: s.shipping_method,
            tracking_number: s.tracking_number,
            container_number: s.container_number,
            freight_cost: Decimal::from_f64_retain(s.freight_cost).unwrap_or_default(),
            insurance_cost: Decimal::from_f64_retain(s.insurance_cost).unwrap_or_default(),
            customs_duty: Decimal::from_f64_retain(s.customs_duty).unwrap_or_default(),
            other_charges: Decimal::from_f64_retain(s.other_charges).unwrap_or_default(),
            total_cost: Decimal::from_f64_retain(s.total_cost).unwrap_or_default(),
            documents: s.documents,
            notes: s.notes,
            metadata: s.metadata,
            created_at: s.created_at,
            updated_at: s.updated_at,
            created_by: s.created_by,
            updated_by: s.updated_by,
        }
    }
}

// ===== IMPORT ORDER DATABASE OPERATIONS =====
impl ImportOrder {
    // Postgres: uses EXTRACT(YEAR FROM ...)
    async fn generate_import_number_pg(
        pool: &PgPool,
        company_id: Uuid,
    ) -> Result<String, sqlx::Error> {
        let year = chrono::Utc::now().year();
        
        let last_number: Option<String> = sqlx::query_scalar(
            r#"
            SELECT import_number FROM import_orders
            WHERE company_id = $1 
              AND EXTRACT(YEAR FROM created_at) = $2
            ORDER BY created_at DESC
            LIMIT 1
            "#
        )
        .bind(company_id)
        .bind(year as i32)
        .fetch_optional(pool)
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
        
        Ok(format!("IMP-{}-{:05}", year, sequence))
    }
    
    // SQLite: uses strftime('%Y', ...)
    async fn generate_import_number_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
    ) -> Result<String, sqlx::Error> {
        let year = chrono::Utc::now().year();
        
        let last_number: Option<String> = sqlx::query_scalar(
            r#"
            SELECT import_number FROM import_orders
            WHERE company_id = ? 
              AND strftime('%Y', created_at) = ?
            ORDER BY created_at DESC
            LIMIT 1
            "#
        )
        .bind(company_id)
        .bind(year.to_string())
        .fetch_optional(pool)
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
        
        Ok(format!("IMP-{}-{:05}", year, sequence))
    }
    
    // ===== CREATE IMPORT ORDER (Postgres) =====
    pub async fn create_pg(
        pool: &PgPool,
        request: CreateImportOrderRequest,
        created_by: Uuid,
    ) -> Result<ImportOrder, sqlx::Error> {
        let import_number = Self::generate_import_number_pg(pool, request.company_id).await?;
        
        let import_order = sqlx::query_as::<Postgres, ImportOrder>(
            r#"
            INSERT INTO import_orders (
                company_id, po_id, import_number, supplier_id,
                shipment_date, arrival_date, status, shipping_method,
                tracking_number, container_number, freight_cost,
                insurance_cost, customs_duty, other_charges, documents,
                notes, metadata, created_by, updated_by
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12,
                $13, $14, $15, $16, $17, $18, $19
            )
            RETURNING *
            "#
        )
        .bind(request.company_id)
        .bind(request.po_id)
        .bind(&import_number)
        .bind(request.supplier_id)
        .bind(request.shipment_date)
        .bind(request.arrival_date)
        .bind("pending")
        .bind(request.shipping_method)
        .bind(request.tracking_number)
        .bind(request.container_number)
        .bind(request.freight_cost.unwrap_or(Decimal::ZERO))
        .bind(request.insurance_cost.unwrap_or(Decimal::ZERO))
        .bind(request.customs_duty.unwrap_or(Decimal::ZERO))
        .bind(request.other_charges.unwrap_or(Decimal::ZERO))
        .bind(request.documents.unwrap_or(serde_json::json!([])))
        .bind(request.notes)
        .bind(request.metadata.unwrap_or(serde_json::json!({})))
        .bind(created_by)
        .bind(created_by)
        .fetch_one(pool)
        .await?;
        
        Ok(import_order)
    }
    
    // ===== CREATE IMPORT ORDER (SQLite) =====
    pub async fn create_sqlite(
        pool: &SqlitePool,
        request: CreateImportOrderRequest,
        created_by: Uuid,
    ) -> Result<ImportOrder, sqlx::Error> {
        let import_number = Self::generate_import_number_sqlite(pool, request.company_id).await?;
        
        let import_order_sqlite = sqlx::query_as::<Sqlite, ImportOrderSqlite>(
            r#"
            INSERT INTO import_orders (
                company_id, po_id, import_number, supplier_id,
                shipment_date, arrival_date, status, shipping_method,
                tracking_number, container_number, freight_cost,
                insurance_cost, customs_duty, other_charges, documents,
                notes, metadata, created_by, updated_by
            )
            VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?
            )
            RETURNING *
            "#
        )
        .bind(request.company_id)
        .bind(request.po_id)
        .bind(&import_number)
        .bind(request.supplier_id)
        .bind(request.shipment_date)
        .bind(request.arrival_date)
        .bind("pending")
        .bind(request.shipping_method)
        .bind(request.tracking_number)
        .bind(request.container_number)
        .bind(request.freight_cost.unwrap_or(Decimal::ZERO).to_f64().unwrap_or_default())
        .bind(request.insurance_cost.unwrap_or(Decimal::ZERO).to_f64().unwrap_or_default())
        .bind(request.customs_duty.unwrap_or(Decimal::ZERO).to_f64().unwrap_or_default())
        .bind(request.other_charges.unwrap_or(Decimal::ZERO).to_f64().unwrap_or_default())
        .bind(request.documents.unwrap_or(serde_json::json!([])))
        .bind(request.notes)
        .bind(request.metadata.unwrap_or(serde_json::json!({})))
        .bind(created_by)
        .bind(created_by)
        .fetch_one(pool)
        .await?;
        
        Ok(ImportOrder::from(import_order_sqlite))
    }
    
    // ===== FIND IMPORT ORDER BY ID =====
    pub async fn find_by_id_pg(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<ImportOrder>, sqlx::Error> {
        let import_order = sqlx::query_as::<Postgres, ImportOrder>(
            "SELECT * FROM import_orders WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(import_order)
    }
    
    pub async fn find_by_id_sqlite(
        pool: &SqlitePool,
        id: Uuid,
    ) -> Result<Option<ImportOrder>, sqlx::Error> {
        let import_order = sqlx::query_as::<Sqlite, ImportOrderSqlite>(
            "SELECT * FROM import_orders WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(import_order.map(ImportOrder::from))
    }
    
    // ===== GET IMPORT ORDER WITH DETAILS (Postgres) =====
    pub async fn get_with_details_pg(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<ImportOrderWithDetails>, sqlx::Error> {
        let import_order = match Self::find_by_id_pg(pool, id).await? {
            Some(i) => i,
            None => return Ok(None),
        };
        
        let (supplier_name, supplier_country): (String, Option<String>) = 
            sqlx::query_as("SELECT name, country FROM suppliers WHERE id = $1")
            .bind(import_order.supplier_id)
            .fetch_one(pool)
            .await?;
        
        let po_number: Option<String> = if let Some(po_id) = import_order.po_id {
            sqlx::query_scalar("SELECT po_number FROM purchase_orders WHERE id = $1")
                .bind(po_id)
                .fetch_optional(pool)
                .await?
        } else {
            None
        };
        
        let created_by_username: String = 
            sqlx::query_scalar("SELECT username FROM users WHERE id = $1")
            .bind(import_order.created_by)
            .fetch_one(pool)
            .await?;
        
        Ok(Some(ImportOrderWithDetails {
            import_order,
            supplier_name,
            supplier_country,
            po_number,
            created_by_username,
        }))
    }
    
    // ===== GET IMPORT ORDER WITH DETAILS (SQLite) =====
    pub async fn get_with_details_sqlite(
        pool: &SqlitePool,
        id: Uuid,
    ) -> Result<Option<ImportOrderWithDetails>, sqlx::Error> {
        let import_order = match Self::find_by_id_sqlite(pool, id).await? {
            Some(i) => i,
            None => return Ok(None),
        };
        
        let (supplier_name, supplier_country): (String, Option<String>) = 
            sqlx::query_as("SELECT name, country FROM suppliers WHERE id = ?")
            .bind(import_order.supplier_id)
            .fetch_one(pool)
            .await?;
        
        let po_number: Option<String> = if let Some(po_id) = import_order.po_id {
            sqlx::query_scalar("SELECT po_number FROM purchase_orders WHERE id = ?")
                .bind(po_id)
                .fetch_optional(pool)
                .await?
        } else {
            None
        };
        
        let created_by_username: String = 
            sqlx::query_scalar("SELECT username FROM users WHERE id = ?")
            .bind(import_order.created_by)
            .fetch_one(pool)
            .await?;
        
        Ok(Some(ImportOrderWithDetails {
            import_order,
            supplier_name,
            supplier_country,
            po_number,
            created_by_username,
        }))
    }
    
    // ===== LIST IMPORT ORDERS BY COMPANY =====
    pub async fn list_by_company_pg(
        pool: &PgPool,
        company_id: Uuid,
        status: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ImportOrder>, sqlx::Error> {
        let imports = if let Some(status) = status {
            sqlx::query_as::<Postgres, ImportOrder>(
                r#"
                SELECT * FROM import_orders
                WHERE company_id = $1 AND status = $2
                ORDER BY created_at DESC
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
            sqlx::query_as::<Postgres, ImportOrder>(
                r#"
                SELECT * FROM import_orders
                WHERE company_id = $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#
            )
            .bind(company_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?
        };
        
        Ok(imports)
    }
    
    pub async fn list_by_company_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
        status: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ImportOrder>, sqlx::Error> {
        let imports = if let Some(status) = status {
            sqlx::query_as::<Sqlite, ImportOrderSqlite>(
                r#"
                SELECT * FROM import_orders
                WHERE company_id = ? AND status = ?
                ORDER BY created_at DESC
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
            sqlx::query_as::<Sqlite, ImportOrderSqlite>(
                r#"
                SELECT * FROM import_orders
                WHERE company_id = ?
                ORDER BY created_at DESC
                LIMIT ? OFFSET ?
                "#
            )
            .bind(company_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?
        };
        
        Ok(imports.into_iter().map(ImportOrder::from).collect())
    }
    
    // ===== LIST IMPORT ORDERS BY SUPPLIER =====
    pub async fn list_by_supplier_pg(
        pool: &PgPool,
        supplier_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ImportOrder>, sqlx::Error> {
        let imports = sqlx::query_as::<Postgres, ImportOrder>(
            r#"
            SELECT * FROM import_orders
            WHERE supplier_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(supplier_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(imports)
    }
    
    pub async fn list_by_supplier_sqlite(
        pool: &SqlitePool,
        supplier_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ImportOrder>, sqlx::Error> {
        let imports = sqlx::query_as::<Sqlite, ImportOrderSqlite>(
            r#"
            SELECT * FROM import_orders
            WHERE supplier_id = ?
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#
        )
        .bind(supplier_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(imports.into_iter().map(ImportOrder::from).collect())
    }
    
    // ===== UPDATE IMPORT ORDER =====
    pub async fn update_pg(
        pool: &PgPool,
        id: Uuid,
        _request: UpdateImportOrderRequest,
        updated_by: Uuid,
    ) -> Result<ImportOrder, sqlx::Error> {
        let import_order = sqlx::query_as::<Postgres, ImportOrder>(
            r#"
            UPDATE import_orders
            SET updated_by = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING *
            "#
        )
        .bind(updated_by)
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(import_order)
    }
    
    pub async fn update_sqlite(
        pool: &SqlitePool,
        id: Uuid,
        _request: UpdateImportOrderRequest,
        updated_by: Uuid,
    ) -> Result<ImportOrder, sqlx::Error> {
        let import_order_sqlite = sqlx::query_as::<Sqlite, ImportOrderSqlite>(
            r#"
            UPDATE import_orders
            SET updated_by = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            RETURNING *
            "#
        )
        .bind(updated_by)
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(ImportOrder::from(import_order_sqlite))
    }
    
    // ===== DELETE IMPORT ORDER =====
    pub async fn delete_pg(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM import_orders WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn delete_sqlite(
        pool: &SqlitePool,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM import_orders WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        
        Ok(())
    }
}

// ===== UNIT TESTS =====
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_import_request_validation() {
        let request = CreateImportOrderRequest {
            company_id: Uuid::new_v4(),
            po_id: None,
            supplier_id: Uuid::new_v4(),
            shipment_date: None,
            arrival_date: None,
            shipping_method: Some("sea".to_string()),
            tracking_number: Some("TRACK-123".to_string()),
            container_number: None,
            freight_cost: Some(Decimal::from(5000)),
            insurance_cost: None,
            customs_duty: None,
            other_charges: None,
            documents: None,
            notes: None,
            metadata: None,
        };
        
        assert!(request.validate().is_ok());
    }
}