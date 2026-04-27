// src/models/sales.rs
// Sales management models for customer transactions
// Handles sales invoices, invoice items, customers, and payments

use chrono::{DateTime, Utc, Datelike};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, SqlitePool, Postgres, Sqlite};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;
use sqlx::types::Decimal;
use rust_decimal::prelude::ToPrimitive;

// ===== INVOICE STATUS ENUM =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "invoice_status", rename_all = "lowercase")]
pub enum InvoiceStatus {
    #[serde(rename = "draft")]
    Draft,
    
    #[serde(rename = "pending")]
    Pending,
    
    #[serde(rename = "approved")]
    Approved,
    
    #[serde(rename = "paid")]
    Paid,
    
    #[serde(rename = "cancelled")]
    Cancelled,
    
    #[serde(rename = "refunded")]
    Refunded,
}

// ===== SALES INVOICE MODEL =====
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SalesInvoice {
    pub id: Uuid,
    pub company_id: Uuid,
    pub branch_id: Uuid,
    pub customer_id: Uuid,
    
    #[schema(example = "INV-2024-00001")]
    pub invoice_number: String,
    
    pub invoice_date: chrono::NaiveDate,
    pub due_date: Option<chrono::NaiveDate>,
    pub status: InvoiceStatus,
    
    #[schema(value_type = f64, example = 10000.00)]
    pub subtotal: Decimal,
    
    #[schema(value_type = f64, example = 500.00)]
    pub discount_amount: Decimal,
    
    #[schema(value_type = f64, example = 850.00)]
    pub tax_amount: Decimal,
    
    #[schema(value_type = f64, example = 100.00)]
    pub shipping_amount: Decimal,
    
    #[schema(value_type = f64, example = 10450.00)]
    pub total_amount: Decimal,
    
    #[schema(value_type = f64, example = 5000.00)]
    pub paid_amount: Decimal,
    
    #[sqlx(default)]
    #[schema(value_type = f64, example = 5450.00)]
    pub balance_due: Decimal,
    
    #[schema(example = "bank_transfer")]
    pub payment_method: Option<String>,
    
    #[schema(example = "TXN-123456")]
    pub payment_reference: Option<String>,
    
    pub shipping_address: Option<String>,
    pub notes: Option<String>,
    pub terms_and_conditions: Option<String>,
    
    #[sqlx(default)]
    pub metadata: serde_json::Value,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Option<Uuid>,
}

// ===== SALES INVOICE SQLITE INTERMEDIATE STRUCT =====
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SalesInvoiceSqlite {
    pub id: Uuid,
    pub company_id: Uuid,
    pub branch_id: Uuid,
    pub customer_id: Uuid,
    pub invoice_number: String,
    pub invoice_date: chrono::NaiveDate,
    pub due_date: Option<chrono::NaiveDate>,
    pub status: InvoiceStatus,
    pub subtotal: f64,
    pub discount_amount: f64,
    pub tax_amount: f64,
    pub shipping_amount: f64,
    pub total_amount: f64,
    pub paid_amount: f64,
    pub balance_due: f64,
    pub payment_method: Option<String>,
    pub payment_reference: Option<String>,
    pub shipping_address: Option<String>,
    pub notes: Option<String>,
    pub terms_and_conditions: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Option<Uuid>,
}

// ===== SALES INVOICE ITEM MODEL =====
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SalesInvoiceItem {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub item_id: Uuid,
    pub description: Option<String>,
    
    #[schema(example = 5)]
    pub quantity: i32,
    
    #[schema(value_type = f64, example = 4999.00)]
    pub unit_price: Decimal,
    
    #[schema(value_type = f64, example = 5.0)]
    pub discount_percentage: Decimal,
    
    #[schema(value_type = f64, example = 250.00)]
    pub discount_amount: Decimal,
    
    #[schema(value_type = f64, example = 8.5)]
    pub tax_percentage: Decimal,
    
    #[schema(value_type = f64, example = 425.00)]
    pub tax_amount: Decimal,
    
    #[sqlx(default)]
    #[schema(value_type = f64, example = 5170.00)]
    pub line_total: Decimal,
    
    #[sqlx(default)]
    pub serial_numbers: Vec<String>,
    
    pub batch_number: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ===== SALES INVOICE ITEM SQLITE INTERMEDIATE STRUCT =====
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SalesInvoiceItemSqlite {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub item_id: Uuid,
    pub description: Option<String>,
    pub quantity: i32,
    pub unit_price: f64,
    pub discount_percentage: f64,
    pub discount_amount: f64,
    pub tax_percentage: f64,
    pub tax_amount: f64,
    pub line_total: f64,
    pub serial_numbers: sqlx::types::Json<Vec<String>>,
    pub batch_number: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ===== SALES INVOICE WITH ITEMS =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SalesInvoiceWithItems {
    #[serde(flatten)]
    pub invoice: SalesInvoice,
    
    pub items: Vec<SalesInvoiceItemWithDetails>,
    
    pub customer_name: String,
    pub customer_email: Option<String>,
    pub customer_phone: Option<String>,
    
    pub branch_name: String,
    pub created_by_username: String,
}

// ===== SALES INVOICE ITEM WITH DETAILS =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SalesInvoiceItemWithDetails {
    #[serde(flatten)]
    pub item: SalesInvoiceItem,
    
    pub item_sku: String,
    pub item_name: String,
    pub item_unit: String,
}

// ===== CREATE SALES INVOICE REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateSalesInvoiceRequest {
    pub company_id: Uuid,
    pub branch_id: Uuid,
    pub customer_id: Uuid,
    
    pub invoice_date: Option<chrono::NaiveDate>,
    pub due_date: Option<chrono::NaiveDate>,
    
    #[validate(length(min = 1))]
    pub items: Vec<CreateInvoiceItemRequest>,
    
    #[validate(custom = "validate_decimal_non_negative")]
    pub discount_amount: Option<Decimal>,
    
    #[validate(custom = "validate_decimal_non_negative")]
    pub shipping_amount: Option<Decimal>,
    
    pub payment_method: Option<String>,
    pub payment_reference: Option<String>,
    pub shipping_address: Option<String>,
    pub notes: Option<String>,
    pub terms_and_conditions: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

// ===== CREATE INVOICE ITEM REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateInvoiceItemRequest {
    pub item_id: Uuid,
    pub description: Option<String>,
    
    #[validate(range(min = 1))]
    #[schema(example = 5)]
    pub quantity: i32,
    
    #[validate(custom = "validate_decimal_non_negative")]
    #[schema(example = 4999.00)]
    pub unit_price: Decimal,
    
    #[validate(custom = "validate_decimal_percentage")]
    #[schema(example = 5.0)]
    pub discount_percentage: Option<Decimal>,
    
    #[validate(custom = "validate_decimal_percentage")]
    #[schema(example = 8.5)]
    pub tax_percentage: Option<Decimal>,
    
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

// ===== UPDATE SALES INVOICE REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateSalesInvoiceRequest {
    pub customer_id: Option<Uuid>,
    pub due_date: Option<chrono::NaiveDate>,
    
    #[validate(custom = "validate_decimal_non_negative")]
    pub discount_amount: Option<Decimal>,
    
    #[validate(custom = "validate_decimal_non_negative")]
    pub shipping_amount: Option<Decimal>,
    
    pub payment_method: Option<String>,
    pub payment_reference: Option<String>,
    pub shipping_address: Option<String>,
    pub notes: Option<String>,
    pub terms_and_conditions: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

// ===== APPROVE INVOICE REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApproveInvoiceRequest {
    pub notes: Option<String>,
}

// ===== RECORD PAYMENT REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct RecordPaymentRequest {
    #[validate(custom = "validate_decimal_positive")]
    #[schema(example = 5000.00)]
    pub amount: Decimal,
    
    #[validate(length(min = 1, max = 50))]
    #[schema(example = "bank_transfer")]
    pub payment_method: String,
    
    #[schema(example = "TXN-123456")]
    pub payment_reference: Option<String>,
    
    pub payment_date: Option<chrono::NaiveDate>,
    pub notes: Option<String>,
}

fn validate_decimal_positive(value: &Decimal) -> Result<(), validator::ValidationError> {
    if *value > Decimal::ZERO {
        Ok(())
    } else {
        Err(validator::ValidationError::new("Amount must be positive"))
    }
}

// ===== SALES SUMMARY =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SalesSummary {
    pub total_invoices: i64,
    
    #[schema(value_type = f64, example = 150000.00)]
    pub total_sales: Decimal,
    
    #[schema(value_type = f64, example = 120000.00)]
    pub total_paid: Decimal,
    
    #[schema(value_type = f64, example = 30000.00)]
    pub total_outstanding: Decimal,
    
    pub paid_invoices: i64,
    pub pending_invoices: i64,
    
    #[schema(value_type = f64, example = 5000.00)]
    pub average_invoice_value: Decimal,
}

// ===== HELPER STRUCT FOR SQL QUERIES =====
#[derive(Debug, FromRow)]
pub struct SalesInvoiceItemRow {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub item_id: Uuid,
    pub description: Option<String>,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub discount_percentage: Decimal,
    pub discount_amount: Decimal,
    pub tax_percentage: Decimal,
    pub tax_amount: Decimal,
    pub line_total: Decimal,
    pub serial_numbers: Vec<String>,
    pub batch_number: Option<String>,
    pub created_at: DateTime<Utc>,
    pub item_sku: String,
    pub item_name: String,
    pub item_unit: String,
}

// ===== HELPER STRUCT FOR SQLITE QUERIES =====
#[derive(Debug, FromRow)]
pub struct SalesInvoiceItemRowSqlite {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub item_id: Uuid,
    pub description: Option<String>,
    pub quantity: i32,
    pub unit_price: f64,
    pub discount_percentage: f64,
    pub discount_amount: f64,
    pub tax_percentage: f64,
    pub tax_amount: f64,
    pub line_total: f64,
    pub serial_numbers: sqlx::types::Json<Vec<String>>,
    pub batch_number: Option<String>,
    pub created_at: DateTime<Utc>,
    pub item_sku: String,
    pub item_name: String,
    pub item_unit: String,
}

// ===== CONVERSION IMPLEMENTATIONS =====

impl From<SalesInvoiceSqlite> for SalesInvoice {
    fn from(s: SalesInvoiceSqlite) -> Self {
        Self {
            id: s.id,
            company_id: s.company_id,
            branch_id: s.branch_id,
            customer_id: s.customer_id,
            invoice_number: s.invoice_number,
            invoice_date: s.invoice_date,
            due_date: s.due_date,
            status: s.status,
            subtotal: Decimal::from_f64_retain(s.subtotal).unwrap_or_default(),
            discount_amount: Decimal::from_f64_retain(s.discount_amount).unwrap_or_default(),
            tax_amount: Decimal::from_f64_retain(s.tax_amount).unwrap_or_default(),
            shipping_amount: Decimal::from_f64_retain(s.shipping_amount).unwrap_or_default(),
            total_amount: Decimal::from_f64_retain(s.total_amount).unwrap_or_default(),
            paid_amount: Decimal::from_f64_retain(s.paid_amount).unwrap_or_default(),
            balance_due: Decimal::from_f64_retain(s.balance_due).unwrap_or_default(),
            payment_method: s.payment_method,
            payment_reference: s.payment_reference,
            shipping_address: s.shipping_address,
            notes: s.notes,
            terms_and_conditions: s.terms_and_conditions,
            metadata: s.metadata,
            created_at: s.created_at,
            updated_at: s.updated_at,
            created_by: s.created_by,
            updated_by: s.updated_by,
        }
    }
}

impl From<SalesInvoiceItemSqlite> for SalesInvoiceItem {
    fn from(s: SalesInvoiceItemSqlite) -> Self {
        Self {
            id: s.id,
            invoice_id: s.invoice_id,
            item_id: s.item_id,
            description: s.description,
            quantity: s.quantity,
            unit_price: Decimal::from_f64_retain(s.unit_price).unwrap_or_default(),
            discount_percentage: Decimal::from_f64_retain(s.discount_percentage).unwrap_or_default(),
            discount_amount: Decimal::from_f64_retain(s.discount_amount).unwrap_or_default(),
            tax_percentage: Decimal::from_f64_retain(s.tax_percentage).unwrap_or_default(),
            tax_amount: Decimal::from_f64_retain(s.tax_amount).unwrap_or_default(),
            line_total: Decimal::from_f64_retain(s.line_total).unwrap_or_default(),
            serial_numbers: s.serial_numbers.0,
            batch_number: s.batch_number,
            created_at: s.created_at,
        }
    }
}

impl From<SalesInvoiceItemRowSqlite> for SalesInvoiceItemWithDetails {
    fn from(row: SalesInvoiceItemRowSqlite) -> Self {
        Self {
            item: SalesInvoiceItem {
                id: row.id,
                invoice_id: row.invoice_id,
                item_id: row.item_id,
                description: row.description,
                quantity: row.quantity,
                unit_price: Decimal::from_f64_retain(row.unit_price).unwrap_or_default(),
                discount_percentage: Decimal::from_f64_retain(row.discount_percentage).unwrap_or_default(),
                discount_amount: Decimal::from_f64_retain(row.discount_amount).unwrap_or_default(),
                tax_percentage: Decimal::from_f64_retain(row.tax_percentage).unwrap_or_default(),
                tax_amount: Decimal::from_f64_retain(row.tax_amount).unwrap_or_default(),
                line_total: Decimal::from_f64_retain(row.line_total).unwrap_or_default(),
                serial_numbers: row.serial_numbers.0,
                batch_number: row.batch_number,
                created_at: row.created_at,
            },
            item_sku: row.item_sku,
            item_name: row.item_name,
            item_unit: row.item_unit,
        }
    }
}

// ===== SALES INVOICE DATABASE OPERATIONS =====
impl SalesInvoice {

        // ===== UPDATE INVOICE =====
    pub async fn update_pg(
        pool: &PgPool,
        id: Uuid,
        _request: UpdateSalesInvoiceRequest,
        updated_by: Uuid,
    ) -> Result<SalesInvoice, sqlx::Error> {
        let invoice = sqlx::query_as::<Postgres, SalesInvoice>(
            r#"
            UPDATE sales_invoices
            SET updated_by = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING *
            "#
        )
        .bind(updated_by)
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(invoice)
    }
    
    pub async fn update_sqlite(
        pool: &SqlitePool,
        id: Uuid,
        _request: UpdateSalesInvoiceRequest,
        updated_by: Uuid,
    ) -> Result<SalesInvoice, sqlx::Error> {
        let invoice_sqlite = sqlx::query_as::<Sqlite, SalesInvoiceSqlite>(
            r#"
            UPDATE sales_invoices
            SET updated_by = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            RETURNING *
            "#
        )
        .bind(updated_by)
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(SalesInvoice::from(invoice_sqlite))
    }
    
    // ===== DELETE INVOICE =====
    pub async fn delete_pg(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM sales_invoices WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn delete_sqlite(
        pool: &SqlitePool,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM sales_invoices WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        
        Ok(())
    }
    
    // ===== APPROVE INVOICE =====
    pub async fn approve_pg(
        pool: &PgPool,
        id: Uuid,
        approved_by: Uuid,
    ) -> Result<SalesInvoice, sqlx::Error> {
        let invoice = sqlx::query_as::<Postgres, SalesInvoice>(
            r#"
            UPDATE sales_invoices
            SET status = $1, updated_by = $2, updated_at = NOW()
            WHERE id = $3
            RETURNING *
            "#
        )
        .bind(InvoiceStatus::Approved)
        .bind(approved_by)
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(invoice)
    }
    
    pub async fn approve_sqlite(
        pool: &SqlitePool,
        id: Uuid,
        approved_by: Uuid,
    ) -> Result<SalesInvoice, sqlx::Error> {
        let invoice_sqlite = sqlx::query_as::<Sqlite, SalesInvoiceSqlite>(
            r#"
            UPDATE sales_invoices
            SET status = ?, updated_by = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            RETURNING *
            "#
        )
        .bind(InvoiceStatus::Approved)
        .bind(approved_by)
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(SalesInvoice::from(invoice_sqlite))
    }
    
    // ===== RECORD PAYMENT =====
    pub async fn record_payment_pg(
        pool: &PgPool,
        id: Uuid,
        request: RecordPaymentRequest,
    ) -> Result<SalesInvoice, sqlx::Error> {
        let invoice = Self::find_by_id_pg(pool, id).await?
            .ok_or(sqlx::Error::RowNotFound)?;
        
        let new_paid = invoice.paid_amount + request.amount;
        let new_status = if new_paid >= invoice.total_amount {
            InvoiceStatus::Paid
        } else {
            InvoiceStatus::Approved
        };
        
        let updated = sqlx::query_as::<Postgres, SalesInvoice>(
            r#"
            UPDATE sales_invoices
            SET paid_amount = $1,
                status = $2,
                payment_method = COALESCE($3, payment_method),
                payment_reference = COALESCE($4, payment_reference),
                updated_at = NOW()
            WHERE id = $5
            RETURNING *
            "#
        )
        .bind(new_paid)
        .bind(new_status)
        .bind(request.payment_method)
        .bind(request.payment_reference)
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(updated)
    }
    
    pub async fn record_payment_sqlite(
        pool: &SqlitePool,
        id: Uuid,
        request: RecordPaymentRequest,
    ) -> Result<SalesInvoice, sqlx::Error> {
        let invoice = Self::find_by_id_sqlite(pool, id).await?
            .ok_or(sqlx::Error::RowNotFound)?;
        
        let new_paid = invoice.paid_amount + request.amount;
        let new_status = if new_paid >= invoice.total_amount {
            InvoiceStatus::Paid
        } else {
            InvoiceStatus::Approved
        };
        
        let updated_sqlite = sqlx::query_as::<Sqlite, SalesInvoiceSqlite>(
            r#"
            UPDATE sales_invoices
            SET paid_amount = ?,
                status = ?,
                payment_method = COALESCE(?, payment_method),
                payment_reference = COALESCE(?, payment_reference),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            RETURNING *
            "#
        )
        .bind(new_paid.to_f64().unwrap_or_default())
        .bind(new_status)
        .bind(request.payment_method)
        .bind(request.payment_reference)
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(SalesInvoice::from(updated_sqlite))
    }
    // Generate invoice number (Postgres)
    async fn generate_invoice_number_pg(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        company_id: Uuid,
    ) -> Result<String, sqlx::Error> {
        let year = chrono::Utc::now().year();
        
        let last_number: Option<String> = sqlx::query_scalar(
            r#"
            SELECT invoice_number FROM sales_invoices
            WHERE company_id = $1 
              AND EXTRACT(YEAR FROM invoice_date) = $2
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
        
        Ok(format!("INV-{}-{:05}", year, sequence))
    }
    
    // Generate invoice number (SQLite)
    async fn generate_invoice_number_sqlite(
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        company_id: Uuid,
    ) -> Result<String, sqlx::Error> {
        let year = chrono::Utc::now().year();
        
        let last_number: Option<String> = sqlx::query_scalar(
            r#"
            SELECT invoice_number FROM sales_invoices
            WHERE company_id = ? 
              AND strftime('%Y', invoice_date) = ?
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
        
        Ok(format!("INV-{}-{:05}", year, sequence))
    }
    
    // ===== CREATE SALES INVOICE (Postgres) =====
    pub async fn create_pg(
        pool: &PgPool,
        request: CreateSalesInvoiceRequest,
        created_by: Uuid,
    ) -> Result<SalesInvoiceWithItems, sqlx::Error> {
        let mut tx = pool.begin().await?;
        
        let invoice_number = Self::generate_invoice_number_pg(&mut tx, request.company_id).await?;
        
        // Calculate totals
        let mut subtotal = Decimal::ZERO;
        let mut total_tax = Decimal::ZERO;
        
        for item in &request.items {
            let line_subtotal = item.unit_price * Decimal::from(item.quantity);
            let discount_pct = item.discount_percentage.unwrap_or(Decimal::ZERO);
            let tax_pct = item.tax_percentage.unwrap_or(Decimal::ZERO);
            
            let discount_amt = line_subtotal * (discount_pct / Decimal::from(100));
            let taxable_amount = line_subtotal - discount_amt;
            let tax_amt = taxable_amount * (tax_pct / Decimal::from(100));
            
            subtotal += line_subtotal;
            total_tax += tax_amt;
        }
        
        let discount_amount = request.discount_amount.unwrap_or(Decimal::ZERO);
        let shipping_amount = request.shipping_amount.unwrap_or(Decimal::ZERO);
        let total_amount = subtotal - discount_amount + total_tax + shipping_amount;
        
        let invoice = sqlx::query_as::<Postgres, SalesInvoice>(
            r#"
            INSERT INTO sales_invoices (
                company_id, branch_id, customer_id, invoice_number,
                invoice_date, due_date, status, subtotal, discount_amount,
                tax_amount, shipping_amount, total_amount, paid_amount,
                payment_method, payment_reference, shipping_address,
                notes, terms_and_conditions, metadata, created_by, updated_by
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13,
                $14, $15, $16, $17, $18, $19, $20, $21
            )
            RETURNING *
            "#
        )
        .bind(request.company_id)
        .bind(request.branch_id)
        .bind(request.customer_id)
        .bind(&invoice_number)
        .bind(request.invoice_date.unwrap_or_else(|| chrono::Utc::now().date_naive()))
        .bind(request.due_date)
        .bind(InvoiceStatus::Draft)
        .bind(subtotal)
        .bind(discount_amount)
        .bind(total_tax)
        .bind(shipping_amount)
        .bind(total_amount)
        .bind(Decimal::ZERO)
        .bind(request.payment_method)
        .bind(request.payment_reference)
        .bind(request.shipping_address)
        .bind(request.notes)
        .bind(request.terms_and_conditions)
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
            
            let line_subtotal = item_req.unit_price * Decimal::from(item_req.quantity);
            let discount_pct = item_req.discount_percentage.unwrap_or(Decimal::ZERO);
            let tax_pct = item_req.tax_percentage.unwrap_or(Decimal::ZERO);
            
            let discount_amt = line_subtotal * (discount_pct / Decimal::from(100));
            let taxable_amount = line_subtotal - discount_amt;
            let tax_amt = taxable_amount * (tax_pct / Decimal::from(100));
            
            let invoice_item = sqlx::query_as::<Postgres, SalesInvoiceItem>(
                r#"
                INSERT INTO sales_invoice_items (
                    invoice_id, item_id, description, quantity, unit_price,
                    discount_percentage, discount_amount, tax_percentage,
                    tax_amount, serial_numbers, batch_number
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                RETURNING *
                "#
            )
            .bind(invoice.id)
            .bind(item_req.item_id)
            .bind(item_req.description)
            .bind(item_req.quantity)
            .bind(item_req.unit_price)
            .bind(discount_pct)
            .bind(discount_amt)
            .bind(tax_pct)
            .bind(tax_amt)
            .bind(item_req.serial_numbers.unwrap_or_default())
            .bind(item_req.batch_number)
            .fetch_one(&mut *tx)
            .await?;
            
            items_with_details.push(SalesInvoiceItemWithDetails {
                item: invoice_item,
                item_sku: item_details.0,
                item_name: item_details.1,
                item_unit: item_details.2,
            });
        }
        
        let (customer_name, customer_email, customer_phone): (String, Option<String>, Option<String>) = 
            sqlx::query_as("SELECT name, email, phone FROM customers WHERE id = $1")
            .bind(request.customer_id)
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
        
        Ok(SalesInvoiceWithItems {
            invoice,
            items: items_with_details,
            customer_name,
            customer_email,
            customer_phone,
            branch_name,
            created_by_username,
        })
    }
    
    // ===== CREATE SALES INVOICE (SQLite) =====
    pub async fn create_sqlite(
        pool: &SqlitePool,
        request: CreateSalesInvoiceRequest,
        created_by: Uuid,
    ) -> Result<SalesInvoiceWithItems, sqlx::Error> {
        let mut tx = pool.begin().await?;
        
        let invoice_number = Self::generate_invoice_number_sqlite(&mut tx, request.company_id).await?;
        
        let mut subtotal = Decimal::ZERO;
        let mut total_tax = Decimal::ZERO;
        
        for item in &request.items {
            let line_subtotal = item.unit_price * Decimal::from(item.quantity);
            let discount_pct = item.discount_percentage.unwrap_or(Decimal::ZERO);
            let tax_pct = item.tax_percentage.unwrap_or(Decimal::ZERO);
            
            let discount_amt = line_subtotal * (discount_pct / Decimal::from(100));
            let taxable_amount = line_subtotal - discount_amt;
            let tax_amt = taxable_amount * (tax_pct / Decimal::from(100));
            
            subtotal += line_subtotal;
            total_tax += tax_amt;
        }
        
        let discount_amount = request.discount_amount.unwrap_or(Decimal::ZERO);
        let shipping_amount = request.shipping_amount.unwrap_or(Decimal::ZERO);
        let total_amount = subtotal - discount_amount + total_tax + shipping_amount;
        
        let invoice_sqlite = sqlx::query_as::<Sqlite, SalesInvoiceSqlite>(
            r#"
            INSERT INTO sales_invoices (
                company_id, branch_id, customer_id, invoice_number,
                invoice_date, due_date, status, subtotal, discount_amount,
                tax_amount, shipping_amount, total_amount, paid_amount,
                payment_method, payment_reference, shipping_address,
                notes, terms_and_conditions, metadata, created_by, updated_by
            )
            VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
            RETURNING *
            "#
        )
        .bind(request.company_id)
        .bind(request.branch_id)
        .bind(request.customer_id)
        .bind(&invoice_number)
        .bind(request.invoice_date.unwrap_or_else(|| chrono::Utc::now().date_naive()))
        .bind(request.due_date)
        .bind(InvoiceStatus::Draft)
        .bind(subtotal.to_f64().unwrap_or_default())
        .bind(discount_amount.to_f64().unwrap_or_default())
        .bind(total_tax.to_f64().unwrap_or_default())
        .bind(shipping_amount.to_f64().unwrap_or_default())
        .bind(total_amount.to_f64().unwrap_or_default())
        .bind(0.0) // paid_amount
        .bind(request.payment_method)
        .bind(request.payment_reference)
        .bind(request.shipping_address)
        .bind(request.notes)
        .bind(request.terms_and_conditions)
        .bind(request.metadata.unwrap_or(serde_json::json!({})))
        .bind(created_by)
        .bind(created_by)
        .fetch_one(&mut *tx)
        .await?;
        
        let invoice = SalesInvoice::from(invoice_sqlite);
        
        let mut items_with_details = Vec::new();
        
        for item_req in request.items {
            let item_details: (String, String, String) = sqlx::query_as(
                "SELECT sku, name, unit_of_measure FROM inventory_items WHERE id = ?"
            )
            .bind(item_req.item_id)
            .fetch_one(&mut *tx)
            .await?;
            
            let line_subtotal = item_req.unit_price * Decimal::from(item_req.quantity);
            let discount_pct = item_req.discount_percentage.unwrap_or(Decimal::ZERO);
            let tax_pct = item_req.tax_percentage.unwrap_or(Decimal::ZERO);
            
            let discount_amt = line_subtotal * (discount_pct / Decimal::from(100));
            let taxable_amount = line_subtotal - discount_amt;
            let tax_amt = taxable_amount * (tax_pct / Decimal::from(100));
            
            let invoice_item_sqlite = sqlx::query_as::<Sqlite, SalesInvoiceItemSqlite>(
                r#"
                INSERT INTO sales_invoice_items (
                    invoice_id, item_id, description, quantity, unit_price,
                    discount_percentage, discount_amount, tax_percentage,
                    tax_amount, serial_numbers, batch_number
                )
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                RETURNING *
                "#
            )
            .bind(invoice.id)
            .bind(item_req.item_id)
            .bind(item_req.description)
            .bind(item_req.quantity)
            .bind(item_req.unit_price.to_f64().unwrap_or_default())
            .bind(discount_pct.to_f64().unwrap_or_default())
            .bind(discount_amt.to_f64().unwrap_or_default())
            .bind(tax_pct.to_f64().unwrap_or_default())
            .bind(tax_amt.to_f64().unwrap_or_default())
            .bind(serde_json::json!(item_req.serial_numbers.unwrap_or_default()))
            .bind(item_req.batch_number)
            .fetch_one(&mut *tx)
            .await?;
            
            items_with_details.push(SalesInvoiceItemWithDetails {
                item: SalesInvoiceItem::from(invoice_item_sqlite),
                item_sku: item_details.0,
                item_name: item_details.1,
                item_unit: item_details.2,
            });
        }
        
        let (customer_name, customer_email, customer_phone): (String, Option<String>, Option<String>) = 
            sqlx::query_as("SELECT name, email, phone FROM customers WHERE id = ?")
            .bind(request.customer_id)
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
        
        Ok(SalesInvoiceWithItems {
            invoice,
            items: items_with_details,
            customer_name,
            customer_email,
            customer_phone,
            branch_name,
            created_by_username,
        })
    }
    
    // ===== FIND INVOICE BY ID (Postgres) =====
    pub async fn find_by_id_pg(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<SalesInvoice>, sqlx::Error> {
        let invoice = sqlx::query_as::<Postgres, SalesInvoice>(
            "SELECT * FROM sales_invoices WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(invoice)
    }
    
    // ===== FIND INVOICE BY ID (SQLite) =====
    pub async fn find_by_id_sqlite(
        pool: &SqlitePool,
        id: Uuid,
    ) -> Result<Option<SalesInvoice>, sqlx::Error> {
        let invoice = sqlx::query_as::<Sqlite, SalesInvoiceSqlite>(
            "SELECT * FROM sales_invoices WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(invoice.map(SalesInvoice::from))
    }
    
    // ===== GET INVOICE WITH ITEMS (Postgres) =====
    pub async fn get_with_items_pg(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<SalesInvoiceWithItems>, sqlx::Error> {
        let invoice = match Self::find_by_id_pg(pool, id).await? {
            Some(inv) => inv,
            None => return Ok(None),
        };
        
        let item_rows = sqlx::query_as::<Postgres, SalesInvoiceItemRow>(
            r#"
            SELECT 
                sii.id, sii.invoice_id, sii.item_id, sii.description,
                sii.quantity, sii.unit_price, sii.discount_percentage,
                sii.discount_amount, sii.tax_percentage, sii.tax_amount,
                sii.line_total, sii.serial_numbers, sii.batch_number,
                sii.created_at,
                i.sku as item_sku, i.name as item_name, i.unit_of_measure as item_unit
            FROM sales_invoice_items sii
            JOIN inventory_items i ON sii.item_id = i.id
            WHERE sii.invoice_id = $1
            ORDER BY sii.created_at
            "#
        )
        .bind(id)
        .fetch_all(pool)
        .await?;
        
        let items = item_rows.into_iter().map(|row| {
            SalesInvoiceItemWithDetails {
                item: SalesInvoiceItem {
                    id: row.id,
                    invoice_id: row.invoice_id,
                    item_id: row.item_id,
                    description: row.description,
                    quantity: row.quantity,
                    unit_price: row.unit_price,
                    discount_percentage: row.discount_percentage,
                    discount_amount: row.discount_amount,
                    tax_percentage: row.tax_percentage,
                    tax_amount: row.tax_amount,
                    line_total: row.line_total,
                    serial_numbers: row.serial_numbers,
                    batch_number: row.batch_number,
                    created_at: row.created_at,
                },
                item_sku: row.item_sku,
                item_name: row.item_name,
                item_unit: row.item_unit,
            }
        }).collect();
        
        let (customer_name, customer_email, customer_phone): (String, Option<String>, Option<String>) = 
            sqlx::query_as("SELECT name, email, phone FROM customers WHERE id = $1")
            .bind(invoice.customer_id)
            .fetch_one(pool)
            .await?;
        
        let branch_name: String = sqlx::query_scalar("SELECT name FROM branches WHERE id = $1")
            .bind(invoice.branch_id)
            .fetch_one(pool)
            .await?;
        
        let created_by_username: String = sqlx::query_scalar("SELECT username FROM users WHERE id = $1")
            .bind(invoice.created_by)
            .fetch_one(pool)
            .await?;
        
        Ok(Some(SalesInvoiceWithItems {
            invoice,
            items,
            customer_name,
            customer_email,
            customer_phone,
            branch_name,
            created_by_username,
        }))
    }
    
    // ===== GET INVOICE WITH ITEMS (SQLite) =====
    pub async fn get_with_items_sqlite(
        pool: &SqlitePool,
        id: Uuid,
    ) -> Result<Option<SalesInvoiceWithItems>, sqlx::Error> {
        let invoice = match Self::find_by_id_sqlite(pool, id).await? {
            Some(inv) => inv,
            None => return Ok(None),
        };
        
        let item_rows = sqlx::query_as::<Sqlite, SalesInvoiceItemRowSqlite>(
            r#"
            SELECT 
                sii.id, sii.invoice_id, sii.item_id, sii.description,
                sii.quantity, sii.unit_price, sii.discount_percentage,
                sii.discount_amount, sii.tax_percentage, sii.tax_amount,
                sii.line_total, sii.serial_numbers, sii.batch_number,
                sii.created_at,
                i.sku as item_sku, i.name as item_name, i.unit_of_measure as item_unit
            FROM sales_invoice_items sii
            JOIN inventory_items i ON sii.item_id = i.id
            WHERE sii.invoice_id = ?
            ORDER BY sii.created_at
            "#
        )
        .bind(id)
        .fetch_all(pool)
        .await?;
        
        let items = item_rows.into_iter().map(SalesInvoiceItemWithDetails::from).collect();
        
        let (customer_name, customer_email, customer_phone): (String, Option<String>, Option<String>) = 
            sqlx::query_as("SELECT name, email, phone FROM customers WHERE id = ?")
            .bind(invoice.customer_id)
            .fetch_one(pool)
            .await?;
        
        let branch_name: String = sqlx::query_scalar("SELECT name FROM branches WHERE id = ?")
            .bind(invoice.branch_id)
            .fetch_one(pool)
            .await?;
        
        let created_by_username: String = sqlx::query_scalar("SELECT username FROM users WHERE id = ?")
            .bind(invoice.created_by)
            .fetch_one(pool)
            .await?;
        
        Ok(Some(SalesInvoiceWithItems {
            invoice,
            items,
            customer_name,
            customer_email,
            customer_phone,
            branch_name,
            created_by_username,
        }))
    }
    
    // ===== LIST INVOICES BY COMPANY =====
    pub async fn list_by_company_pg(
        pool: &PgPool,
        company_id: Uuid,
        status: Option<InvoiceStatus>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SalesInvoice>, sqlx::Error> {
        let invoices = if let Some(status) = status {
            sqlx::query_as::<Postgres, SalesInvoice>(
                r#"
                SELECT * FROM sales_invoices
                WHERE company_id = $1 AND status = $2
                ORDER BY invoice_date DESC, created_at DESC
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
            sqlx::query_as::<Postgres, SalesInvoice>(
                r#"
                SELECT * FROM sales_invoices
                WHERE company_id = $1
                ORDER BY invoice_date DESC, created_at DESC
                LIMIT $2 OFFSET $3
                "#
            )
            .bind(company_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?
        };
        
        Ok(invoices)
    }
    
    pub async fn list_by_company_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
        status: Option<InvoiceStatus>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SalesInvoice>, sqlx::Error> {
        let invoices = if let Some(status) = status {
            sqlx::query_as::<Sqlite, SalesInvoiceSqlite>(
                r#"
                SELECT * FROM sales_invoices
                WHERE company_id = ? AND status = ?
                ORDER BY invoice_date DESC, created_at DESC
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
            sqlx::query_as::<Sqlite, SalesInvoiceSqlite>(
                r#"
                SELECT * FROM sales_invoices
                WHERE company_id = ?
                ORDER BY invoice_date DESC, created_at DESC
                LIMIT ? OFFSET ?
                "#
            )
            .bind(company_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?
        };
        
        Ok(invoices.into_iter().map(SalesInvoice::from).collect())
    }
    
    // ===== GET SALES SUMMARY =====
    pub async fn get_sales_summary_pg(
        pool: &PgPool,
        company_id: Uuid,
        start_date: Option<chrono::NaiveDate>,
        end_date: Option<chrono::NaiveDate>,
    ) -> Result<SalesSummary, sqlx::Error> {
        let row: (i64, Option<Decimal>, Option<Decimal>, Option<Decimal>, i64, i64, Option<Decimal>) = 
            sqlx::query_as(
                r#"
                SELECT 
                    COUNT(*) as total_invoices,
                    COALESCE(SUM(total_amount), 0) as total_sales,
                    COALESCE(SUM(paid_amount), 0) as total_paid,
                    COALESCE(SUM(balance_due), 0) as total_outstanding,
                    COUNT(*) FILTER (WHERE status = 'paid') as paid_invoices,
                    COUNT(*) FILTER (WHERE status IN ('pending', 'approved')) as pending_invoices,
                    COALESCE(AVG(total_amount), 0) as average_invoice_value
                FROM sales_invoices
                WHERE company_id = $1
                  AND status != 'cancelled'
                  AND ($2::date IS NULL OR invoice_date >= $2)
                  AND ($3::date IS NULL OR invoice_date <= $3)
                "#
            )
            .bind(company_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_one(pool)
            .await?;
        
        Ok(SalesSummary {
            total_invoices: row.0,
            total_sales: row.1.unwrap_or(Decimal::ZERO),
            total_paid: row.2.unwrap_or(Decimal::ZERO),
            total_outstanding: row.3.unwrap_or(Decimal::ZERO),
            paid_invoices: row.4,
            pending_invoices: row.5,
            average_invoice_value: row.6.unwrap_or(Decimal::ZERO),
        })
    }

    pub async fn get_sales_summary_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
        start_date: Option<chrono::NaiveDate>,
        end_date: Option<chrono::NaiveDate>,
    ) -> Result<SalesSummary, sqlx::Error> {
        let row: (i64, Option<f64>, Option<f64>, Option<f64>, i64, i64, Option<f64>) = 
            sqlx::query_as(
                r#"
                SELECT 
                    COUNT(*) as total_invoices,
                    COALESCE(SUM(total_amount), 0) as total_sales,
                    COALESCE(SUM(paid_amount), 0) as total_paid,
                    COALESCE(SUM(balance_due), 0) as total_outstanding,
                    COUNT(*) FILTER (WHERE status = 'paid') as paid_invoices,
                    COUNT(*) FILTER (WHERE status IN ('pending', 'approved')) as pending_invoices,
                    COALESCE(AVG(total_amount), 0) as average_invoice_value
                FROM sales_invoices
                WHERE company_id = ?
                  AND status != 'cancelled'
                  AND (? IS NULL OR invoice_date >= ?)
                  AND (? IS NULL OR invoice_date <= ?)
                "#
            )
            .bind(company_id)
            .bind(start_date)
            .bind(start_date)
            .bind(end_date)
            .bind(end_date)
            .fetch_one(pool)
            .await?;
        
        Ok(SalesSummary {
            total_invoices: row.0,
            total_sales: Decimal::from_f64_retain(row.1.unwrap_or(0.0)).unwrap_or(Decimal::ZERO),
            total_paid: Decimal::from_f64_retain(row.2.unwrap_or(0.0)).unwrap_or(Decimal::ZERO),
            total_outstanding: Decimal::from_f64_retain(row.3.unwrap_or(0.0)).unwrap_or(Decimal::ZERO),
            paid_invoices: row.4,
            pending_invoices: row.5,
            average_invoice_value: Decimal::from_f64_retain(row.6.unwrap_or(0.0)).unwrap_or(Decimal::ZERO),
        })
    }
}

// ===== UNIT TESTS =====
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use validator::Validate;
    
    #[test]
    fn test_invoice_status_serialization() {
        let status = InvoiceStatus::Approved;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, r#""approved""#);
    }
    
    #[test]
    fn test_create_invoice_validation() {
        let request = CreateSalesInvoiceRequest {
            company_id: Uuid::new_v4(),
            branch_id: Uuid::new_v4(),
            customer_id: Uuid::new_v4(),
            invoice_date: None,
            due_date: None,
            items: vec![
                CreateInvoiceItemRequest {
                    item_id: Uuid::new_v4(),
                    description: None,
                    quantity: 5,
                    unit_price: Decimal::from(4999),
                    discount_percentage: Some(Decimal::from(5)),
                    tax_percentage: Some(Decimal::from_str("8.5").unwrap()),
                    serial_numbers: None,
                    batch_number: None,
                }
            ],
            discount_amount: None,
            shipping_amount: None,
            payment_method: None,
            payment_reference: None,
            shipping_address: None,
            notes: None,
            terms_and_conditions: None,
            metadata: None,
        };
        
        assert!(request.validate().is_ok());
    }
}