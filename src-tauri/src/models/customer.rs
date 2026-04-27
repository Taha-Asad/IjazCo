// src/models/customer.rs
// Customer relationship management models
// Handles customer information, contacts, and billing details

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;
use sqlx::types::Decimal;
use sqlx::{PgPool, SqlitePool, Postgres, Sqlite};
use rust_decimal::prelude::ToPrimitive;

// ===== CUSTOMER MODEL =====
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Customer {
    pub id: Uuid,
    pub company_id: Uuid,
    
    #[schema(example = "CUST-001")]
    pub customer_code: String,
    
    #[schema(example = "ABC Research Institute")]
    pub name: String,
    
    #[schema(example = "Dr. John Smith")]
    pub contact_person: Option<String>,
    
    #[schema(example = "contact@abcresearch.com")]
    pub email: Option<String>,
    
    #[schema(example = "+1-555-1234")]
    pub phone: Option<String>,
    
    #[schema(example = "+1-555-5678")]
    pub mobile: Option<String>,
    
    #[schema(example = "TAX-123456789")]
    pub tax_id: Option<String>,
    
    pub billing_address: Option<String>,
    
    #[schema(example = "San Francisco")]
    pub billing_city: Option<String>,
    
    #[schema(example = "California")]
    pub billing_state: Option<String>,
    
    #[schema(example = "USA")]
    pub billing_country: Option<String>,
    
    #[schema(example = "94102")]
    pub billing_postal_code: Option<String>,
    
    pub shipping_address: Option<String>,
    pub shipping_city: Option<String>,
    pub shipping_state: Option<String>,
    pub shipping_country: Option<String>,
    pub shipping_postal_code: Option<String>,
    
    #[schema(value_type = f64, example = 50000.00)]
    pub credit_limit: Decimal,
    
    #[schema(example = 30)]
    pub credit_days: i32,
    
    #[schema(value_type = f64, example = 5.0)]
    pub discount_percentage: Decimal,
    
    pub is_active: bool,
    
    #[sqlx(default)]
    pub tags: Vec<String>,
    
    pub notes: Option<String>,
    
    #[sqlx(default)]
    pub metadata: serde_json::Value,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// ===== CUSTOMER SQLITE INTERMEDIATE STRUCT =====
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CustomerSqlite {
    pub id: Uuid,
    pub company_id: Uuid,
    pub customer_code: String,
    pub name: String,
    pub contact_person: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub mobile: Option<String>,
    pub tax_id: Option<String>,
    pub billing_address: Option<String>,
    pub billing_city: Option<String>,
    pub billing_state: Option<String>,
    pub billing_country: Option<String>,
    pub billing_postal_code: Option<String>,
    pub shipping_address: Option<String>,
    pub shipping_city: Option<String>,
    pub shipping_state: Option<String>,
    pub shipping_country: Option<String>,
    pub shipping_postal_code: Option<String>,
    pub credit_limit: f64,
    pub credit_days: i32,
    pub discount_percentage: f64,
    pub is_active: bool,
    pub tags: sqlx::types::Json<Vec<String>>,
    pub notes: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// ===== CUSTOMER WITH STATISTICS =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CustomerWithStats {
    #[serde(flatten)]
    pub customer: Customer,
    
    pub total_invoices: i64,
    
    #[schema(value_type = f64, example = 150000.00)]
    pub total_sales: Decimal,
    
    #[schema(value_type = f64, example = 5000.00)]
    pub outstanding_balance: Decimal,
    
    pub last_invoice_date: Option<chrono::NaiveDate>,
    
    #[schema(value_type = f64, example = 5000.00)]
    pub average_invoice_value: Decimal,
}

// ===== CREATE CUSTOMER REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateCustomerRequest {
    pub company_id: Uuid,
    
    #[validate(length(min = 1, max = 50))]
    #[schema(example = "CUST-001")]
    pub customer_code: String,
    
    #[validate(length(min = 1, max = 255))]
    #[schema(example = "ABC Research Institute")]
    pub name: String,
    
    #[validate(length(max = 255))]
    pub contact_person: Option<String>,
    
    #[validate(email)]
    pub email: Option<String>,
    
    #[validate(length(max = 20))]
    pub phone: Option<String>,
    
    pub mobile: Option<String>,
    pub tax_id: Option<String>,
    
    pub billing_address: Option<String>,
    pub billing_city: Option<String>,
    pub billing_state: Option<String>,
    pub billing_country: Option<String>,
    pub billing_postal_code: Option<String>,
    
    pub shipping_address: Option<String>,
    pub shipping_city: Option<String>,
    pub shipping_state: Option<String>,
    pub shipping_country: Option<String>,
    pub shipping_postal_code: Option<String>,
    
    #[validate(custom = "validate_decimal_non_negative")]
    pub credit_limit: Option<Decimal>,
    
    #[validate(range(min = 0))]
    pub credit_days: Option<i32>,
    
    #[validate(custom = "validate_decimal_percentage")]
    pub discount_percentage: Option<Decimal>,
    
    pub tags: Option<Vec<String>>,
    pub notes: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

// ===== UPDATE CUSTOMER REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateCustomerRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    
    pub contact_person: Option<String>,
    
    #[validate(email)]
    pub email: Option<String>,
    
    pub phone: Option<String>,
    pub mobile: Option<String>,
    pub tax_id: Option<String>,
    
    pub billing_address: Option<String>,
    pub billing_city: Option<String>,
    pub billing_state: Option<String>,
    pub billing_country: Option<String>,
    pub billing_postal_code: Option<String>,
    
    pub shipping_address: Option<String>,
    pub shipping_city: Option<String>,
    pub shipping_state: Option<String>,
    pub shipping_country: Option<String>,
    pub shipping_postal_code: Option<String>,
    
    #[validate(custom = "validate_decimal_non_negative")]
    pub credit_limit: Option<Decimal>,
    
    pub credit_days: Option<i32>,
    
    #[validate(custom = "validate_decimal_percentage")]
    pub discount_percentage: Option<Decimal>,
    
    pub is_active: Option<bool>,
    pub tags: Option<Vec<String>>,
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

fn validate_decimal_percentage(value: &Decimal) -> Result<(), validator::ValidationError> {
    if *value >= Decimal::ZERO && *value <= Decimal::new(100, 0) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("Percentage must be between 0 and 100"))
    }
}

// ===== CONVERSION IMPLEMENTATION =====
impl From<CustomerSqlite> for Customer {
    fn from(s: CustomerSqlite) -> Self {
        Self {
            id: s.id,
            company_id: s.company_id,
            customer_code: s.customer_code,
            name: s.name,
            contact_person: s.contact_person,
            email: s.email,
            phone: s.phone,
            mobile: s.mobile,
            tax_id: s.tax_id,
            billing_address: s.billing_address,
            billing_city: s.billing_city,
            billing_state: s.billing_state,
            billing_country: s.billing_country,
            billing_postal_code: s.billing_postal_code,
            shipping_address: s.shipping_address,
            shipping_city: s.shipping_city,
            shipping_state: s.shipping_state,
            shipping_country: s.shipping_country,
            shipping_postal_code: s.shipping_postal_code,
            credit_limit: Decimal::from_f64_retain(s.credit_limit).unwrap_or_default(),
            credit_days: s.credit_days,
            discount_percentage: Decimal::from_f64_retain(s.discount_percentage).unwrap_or_default(),
            is_active: s.is_active,
            tags: s.tags.0,
            notes: s.notes,
            metadata: s.metadata,
            created_at: s.created_at,
            updated_at: s.updated_at,
            created_by: s.created_by,
            updated_by: s.updated_by,
        }
    }
}

// ===== CUSTOMER DATABASE OPERATIONS =====
impl Customer {
    // ===== CREATE CUSTOMER (Postgres) =====
    pub async fn create_pg(
        pool: &PgPool,
        request: CreateCustomerRequest,
        created_by: Uuid,
    ) -> Result<Customer, sqlx::Error> {
        let customer = sqlx::query_as::<Postgres, Customer>(
            r#"
            INSERT INTO customers (
                company_id, customer_code, name, contact_person, email,
                phone, mobile, tax_id, billing_address, billing_city,
                billing_state, billing_country, billing_postal_code,
                shipping_address, shipping_city, shipping_state,
                shipping_country, shipping_postal_code, credit_limit,
                credit_days, discount_percentage, is_active, tags,
                notes, metadata, created_by, updated_by
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15, $16, $17, $18, $19,
                $20, $21, $22, $23, $24, $25, $26, $27
            )
            RETURNING *
            "#
        )
        .bind(request.company_id)
        .bind(request.customer_code)
        .bind(request.name)
        .bind(request.contact_person)
        .bind(request.email)
        .bind(request.phone)
        .bind(request.mobile)
        .bind(request.tax_id)
        .bind(request.billing_address)
        .bind(request.billing_city)
        .bind(request.billing_state)
        .bind(request.billing_country)
        .bind(request.billing_postal_code)
        .bind(request.shipping_address)
        .bind(request.shipping_city)
        .bind(request.shipping_state)
        .bind(request.shipping_country)
        .bind(request.shipping_postal_code)
        .bind(request.credit_limit.unwrap_or(Decimal::ZERO))
        .bind(request.credit_days.unwrap_or(30))
        .bind(request.discount_percentage.unwrap_or(Decimal::ZERO))
        .bind(true)
        .bind(request.tags.unwrap_or_default())
        .bind(request.notes)
        .bind(request.metadata.unwrap_or(serde_json::json!({})))
        .bind(created_by)
        .bind(created_by)
        .fetch_one(pool)
        .await?;
        
        Ok(customer)
    }
    
    // ===== CREATE CUSTOMER (SQLite) =====
    pub async fn create_sqlite(
        pool: &SqlitePool,
        request: CreateCustomerRequest,
        created_by: Uuid,
    ) -> Result<Customer, sqlx::Error> {
        let customer_sqlite = sqlx::query_as::<Sqlite, CustomerSqlite>(
            r#"
            INSERT INTO customers (
                company_id, customer_code, name, contact_person, email,
                phone, mobile, tax_id, billing_address, billing_city,
                billing_state, billing_country, billing_postal_code,
                shipping_address, shipping_city, shipping_state,
                shipping_country, shipping_postal_code, credit_limit,
                credit_days, discount_percentage, is_active, tags,
                notes, metadata, created_by, updated_by
            )
            VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?, ?
            )
            RETURNING *
            "#
        )
        .bind(request.company_id)
        .bind(request.customer_code)
        .bind(request.name)
        .bind(request.contact_person)
        .bind(request.email)
        .bind(request.phone)
        .bind(request.mobile)
        .bind(request.tax_id)
        .bind(request.billing_address)
        .bind(request.billing_city)
        .bind(request.billing_state)
        .bind(request.billing_country)
        .bind(request.billing_postal_code)
        .bind(request.shipping_address)
        .bind(request.shipping_city)
        .bind(request.shipping_state)
        .bind(request.shipping_country)
        .bind(request.shipping_postal_code)
        .bind(request.credit_limit.unwrap_or(Decimal::ZERO).to_f64().unwrap_or_default())
        .bind(request.credit_days.unwrap_or(30))
        .bind(request.discount_percentage.unwrap_or(Decimal::ZERO).to_f64().unwrap_or_default())
        .bind(true)
        .bind(serde_json::json!(request.tags.unwrap_or_default()))
        .bind(request.notes)
        .bind(request.metadata.unwrap_or(serde_json::json!({})))
        .bind(created_by)
        .bind(created_by)
        .fetch_one(pool)
        .await?;
        
        Ok(Customer::from(customer_sqlite))
    }
    
    // ===== FIND CUSTOMER BY ID =====
    pub async fn find_by_id_pg(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<Customer>, sqlx::Error> {
        let customer = sqlx::query_as::<Postgres, Customer>(
            "SELECT * FROM customers WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(customer)
    }
    
    pub async fn find_by_id_sqlite(
        pool: &SqlitePool,
        id: Uuid,
    ) -> Result<Option<Customer>, sqlx::Error> {
        let customer = sqlx::query_as::<Sqlite, CustomerSqlite>(
            "SELECT * FROM customers WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(customer.map(Customer::from))
    }
    
    // ===== FIND CUSTOMER BY CODE =====
    pub async fn find_by_code_pg(
        pool: &PgPool,
        company_id: Uuid,
        customer_code: &str,
    ) -> Result<Option<Customer>, sqlx::Error> {
        let customer = sqlx::query_as::<Postgres, Customer>(
            "SELECT * FROM customers WHERE company_id = $1 AND customer_code = $2"
        )
        .bind(company_id)
        .bind(customer_code)
        .fetch_optional(pool)
        .await?;
        
        Ok(customer)
    }
    
    pub async fn find_by_code_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
        customer_code: &str,
    ) -> Result<Option<Customer>, sqlx::Error> {
        let customer = sqlx::query_as::<Sqlite, CustomerSqlite>(
            "SELECT * FROM customers WHERE company_id = ? AND customer_code = ?"
        )
        .bind(company_id)
        .bind(customer_code)
        .fetch_optional(pool)
        .await?;
        
        Ok(customer.map(Customer::from))
    }
    
    // ===== LIST CUSTOMERS BY COMPANY =====
    pub async fn list_by_company_pg(
        pool: &PgPool,
        company_id: Uuid,
        active_only: bool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Customer>, sqlx::Error> {
        let customers = if active_only {
            sqlx::query_as::<Postgres, Customer>(
                r#"
                SELECT * FROM customers
                WHERE company_id = $1 AND is_active = true
                ORDER BY name
                LIMIT $2 OFFSET $3
                "#
            )
            .bind(company_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as::<Postgres, Customer>(
                r#"
                SELECT * FROM customers
                WHERE company_id = $1
                ORDER BY name
                LIMIT $2 OFFSET $3
                "#
            )
            .bind(company_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?
        };
        
        Ok(customers)
    }
    
    pub async fn list_by_company_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
        active_only: bool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Customer>, sqlx::Error> {
        let customers = if active_only {
            sqlx::query_as::<Sqlite, CustomerSqlite>(
                r#"
                SELECT * FROM customers
                WHERE company_id = ? AND is_active = true
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
            sqlx::query_as::<Sqlite, CustomerSqlite>(
                r#"
                SELECT * FROM customers
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
        
        Ok(customers.into_iter().map(Customer::from).collect())
    }
    
    // ===== SEARCH CUSTOMERS =====
    pub async fn search_pg(
        pool: &PgPool,
        company_id: Uuid,
        search_term: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Customer>, sqlx::Error> {
        let pattern = format!("%{}%", search_term);
        let customers = sqlx::query_as::<Postgres, Customer>(
            r#"
            SELECT * FROM customers
            WHERE company_id = $1
              AND is_active = true
              AND (
                name ILIKE $2
                OR customer_code ILIKE $2
                OR email ILIKE $2
                OR contact_person ILIKE $2
              )
            ORDER BY name
            LIMIT $3 OFFSET $4
            "#
        )
        .bind(company_id)
        .bind(&pattern)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(customers)
    }
    
    pub async fn search_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
        search_term: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Customer>, sqlx::Error> {
        let pattern = format!("%{}%", search_term);
        let customers = sqlx::query_as::<Sqlite, CustomerSqlite>(
            r#"
            SELECT * FROM customers
            WHERE company_id = ?
              AND is_active = true
              AND (
                name LIKE ?
                OR customer_code LIKE ?
                OR email LIKE ?
                OR contact_person LIKE ?
              )
            ORDER BY name
            LIMIT ? OFFSET ?
            "#
        )
        .bind(company_id)
        .bind(&pattern)
        .bind(&pattern)
        .bind(&pattern)
        .bind(&pattern)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(customers.into_iter().map(Customer::from).collect())
    }
    
    // ===== UPDATE CUSTOMER =====
    pub async fn update_pg(
        pool: &PgPool,
        id: Uuid,
        _request: UpdateCustomerRequest,
        updated_by: Uuid,
    ) -> Result<Customer, sqlx::Error> {
        let customer = sqlx::query_as::<Postgres, Customer>(
            r#"
            UPDATE customers
            SET updated_by = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING *
            "#
        )
        .bind(updated_by)
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(customer)
    }
    
    pub async fn update_sqlite(
        pool: &SqlitePool,
        id: Uuid,
        _request: UpdateCustomerRequest,
        updated_by: Uuid,
    ) -> Result<Customer, sqlx::Error> {
        let customer_sqlite = sqlx::query_as::<Sqlite, CustomerSqlite>(
            r#"
            UPDATE customers
            SET updated_by = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            RETURNING *
            "#
        )
        .bind(updated_by)
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(Customer::from(customer_sqlite))
    }
    
    // ===== DELETE CUSTOMER =====
    pub async fn delete_pg(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE customers SET is_active = false WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn delete_sqlite(
        pool: &SqlitePool,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE customers SET is_active = false WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        
        Ok(())
    }
    
    // ===== GET CUSTOMER WITH STATISTICS (Postgres) =====
    pub async fn get_with_stats_pg(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<CustomerWithStats>, sqlx::Error> {
        let customer = match Self::find_by_id_pg(pool, id).await? {
            Some(c) => c,
            None => return Ok(None),
        };
        
        let stats: (i64, Option<Decimal>, Option<Decimal>, Option<chrono::NaiveDate>, Option<Decimal>) = 
            sqlx::query_as(
                r#"
                SELECT 
                    COUNT(*) as total_invoices,
                    COALESCE(SUM(total_amount), 0) as total_sales,
                    COALESCE(SUM(balance_due), 0) as outstanding_balance,
                    MAX(invoice_date) as last_invoice_date,
                    COALESCE(AVG(total_amount), 0) as average_invoice_value
                FROM sales_invoices
                WHERE customer_id = $1 AND status != 'cancelled'
                "#
            )
            .bind(id)
            .fetch_one(pool)
            .await?;
        
        Ok(Some(CustomerWithStats {
            customer,
            total_invoices: stats.0,
            total_sales: stats.1.unwrap_or(Decimal::ZERO),
            outstanding_balance: stats.2.unwrap_or(Decimal::ZERO),
            last_invoice_date: stats.3,
            average_invoice_value: stats.4.unwrap_or(Decimal::ZERO),
        }))
    }
    
    // ===== GET CUSTOMER WITH STATISTICS (SQLite) =====
    pub async fn get_with_stats_sqlite(
        pool: &SqlitePool,
        id: Uuid,
    ) -> Result<Option<CustomerWithStats>, sqlx::Error> {
        let customer = match Self::find_by_id_sqlite(pool, id).await? {
            Some(c) => c,
            None => return Ok(None),
        };
        
        let stats: (i64, Option<f64>, Option<f64>, Option<chrono::NaiveDate>, Option<f64>) = 
            sqlx::query_as(
                r#"
                SELECT 
                    COUNT(*) as total_invoices,
                    COALESCE(SUM(total_amount), 0) as total_sales,
                    COALESCE(SUM(balance_due), 0) as outstanding_balance,
                    MAX(invoice_date) as last_invoice_date,
                    COALESCE(AVG(total_amount), 0) as average_invoice_value
                FROM sales_invoices
                WHERE customer_id = ? AND status != 'cancelled'
                "#
            )
            .bind(id)
            .fetch_one(pool)
            .await?;
        
        Ok(Some(CustomerWithStats {
            customer,
            total_invoices: stats.0,
            total_sales: Decimal::from_f64_retain(stats.1.unwrap_or(0.0)).unwrap_or_default(),
            outstanding_balance: Decimal::from_f64_retain(stats.2.unwrap_or(0.0)).unwrap_or_default(),
            last_invoice_date: stats.3,
            average_invoice_value: Decimal::from_f64_retain(stats.4.unwrap_or(0.0)).unwrap_or_default(),
        }))
    }
}