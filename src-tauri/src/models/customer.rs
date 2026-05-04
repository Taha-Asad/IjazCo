// src/models/customer.rs
// Customer relationship management models
// Handles customer information, contacts, and billing details

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, SqlitePool, Postgres, Sqlite};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;
use rust_decimal::prelude::FromPrimitive;

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
    
    pub credit_limit: Decimal,
    pub credit_days: i32,
    pub discount_percentage: Decimal,
    
    pub is_active: bool,
    pub tags: Vec<String>,
    pub notes: Option<String>,
    pub metadata: serde_json::Value,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// ===== CUSTOMER SQLITE INTERMEDIATE STRUCT =====
#[derive(Debug, Clone, sqlx::FromRow)]
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
    pub tags: serde_json::Value,
    pub notes: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

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
            credit_limit: Decimal::from_f64(s.credit_limit).unwrap_or_default(),
            credit_days: s.credit_days,
            discount_percentage: Decimal::from_f64(s.discount_percentage).unwrap_or_default(),
            is_active: s.is_active,
            tags: {
                if let serde_json::Value::Array(arr) = &s.tags {
                    arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()
                } else {
                    vec![]
                }
            },
            notes: s.notes,
            metadata: s.metadata,
            created_at: s.created_at,
            updated_at: s.updated_at,
            created_by: s.created_by,
            updated_by: s.updated_by,
        }
    }
}

// ===== CREATE CUSTOMER REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateCustomerRequest {
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
    
    pub credit_limit: Option<Decimal>,
    pub credit_days: Option<i32>,
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
    
    pub credit_limit: Option<Decimal>,
    pub credit_days: Option<i32>,
    pub discount_percentage: Option<Decimal>,
    
    pub is_active: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<String>,
    pub metadata: Option<serde_json::Value>,
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
}

// ===== VALIDATION FUNCTIONS =====
fn validate_decimal_non_negative(dec: &Decimal) -> Result<(), validator::ValidationError> {
    if dec.is_negative() {
        return Err(validator::ValidationError::new("Decimal must be non-negative"));
    }
    Ok(())
}

fn validate_decimal_percentage(value: &Decimal) -> Result<(), validator::ValidationError> {
    if *value >= Decimal::ZERO && *value <= Decimal::new(100, 0) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("Percentage must be between 0 and 100"))
    }
}

// ===== CUSTOMER DATABASE OPERATIONS =====
impl Customer {
    // ===== CREATE CUSTOMER (Postgres) =====
    pub async fn create_pg(
        pool: &PgPool,
        request: CreateCustomerRequest,
        company_id: Uuid,
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
                credit_days, discount_percentage, tags,
                notes, metadata, created_by, updated_by
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15, $16, $17,
                $18, $19, $20, $21, $22, $23, $24,
                $25, $26
            )
            RETURNING *
            "#
        )
        .bind(company_id)                                    // $1
        .bind(request.customer_code)                          // $2
        .bind(request.name)                                  // $3
        .bind(request.contact_person)                        // $4
        .bind(request.email)                                 // $5
        .bind(request.phone)                                 // $6
        .bind(request.mobile)                                // $7
        .bind(request.tax_id)                                // $8
        .bind(request.billing_address)                        // $9
        .bind(request.billing_city)                           // $10
        .bind(request.billing_state)                          // $11
        .bind(request.billing_country)                        // $12
        .bind(request.billing_postal_code)                    // $13
        .bind(request.shipping_address)                       // $14
        .bind(request.shipping_city)                          // $15
        .bind(request.shipping_state)                         // $16
        .bind(request.shipping_country)                       // $17
        .bind(request.shipping_postal_code)                   // $18
        .bind(request.credit_limit.unwrap_or(Decimal::ZERO)) // $19
        .bind(request.credit_days.unwrap_or(30))             // $20
        .bind(request.discount_percentage.unwrap_or(Decimal::ZERO)) // $21
        .bind(request.tags.unwrap_or_default())              // $22
        .bind(request.notes)                                // $23
        .bind(request.metadata.unwrap_or(serde_json::json!({}))) // $24
        .bind(created_by)                                    // $25
        .bind(created_by)                                    // $26
        .fetch_one(pool)
        .await?;
        
        Ok(customer)
    }
    
    // ===== FIND CUSTOMER BY ID (Postgres) =====
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
    
    // ===== FIND CUSTOMER BY ID (SQLite) =====
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
    
    // ===== FIND CUSTOMER BY CODE (Postgres) =====
    pub async fn find_by_code_pg(
        pool: &PgPool,
        company_id: Uuid,
        code: &str,
    ) -> Result<Option<Customer>, sqlx::Error> {
        let customer = sqlx::query_as::<Postgres, Customer>(
            "SELECT * FROM customers WHERE company_id = $1 AND customer_code = $2"
        )
        .bind(company_id)
        .bind(code)
        .fetch_optional(pool)
        .await?;
        
        Ok(customer)
    }
    
    // ===== LIST CUSTOMERS BY COMPANY (Postgres) =====
    pub async fn list_by_company_pg(
        pool: &PgPool,
        company_id: Uuid,
        active_only: bool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Customer>, sqlx::Error> {
        let customers = sqlx::query_as::<Postgres, Customer>(
            r#"
            SELECT * FROM customers 
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
        
        Ok(customers)
    }
    
    // ===== SEARCH CUSTOMERS (Postgres) =====
    pub async fn search_pg(
        pool: &PgPool,
        company_id: Uuid,
        search_term: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Customer>, sqlx::Error> {
        let search_pattern = format!("%{}%", search_term);
        let customers = sqlx::query_as::<Postgres, Customer>(
            r#"
            SELECT * FROM customers 
            WHERE company_id = $1 
            AND (customer_code ILIKE $2 OR name ILIKE $2 OR contact_person ILIKE $2)
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
        
        Ok(customers)
    }
    
    // ===== UPDATE CUSTOMER (Postgres) =====
    pub async fn update_pg(
        pool: &PgPool,
        id: Uuid,
        request: UpdateCustomerRequest,
        updated_by: Uuid,
    ) -> Result<Customer, sqlx::Error> {
        let mut builder = sqlx::QueryBuilder::<Postgres>::new("UPDATE customers SET ");
        builder.push("updated_by = ").push_bind(updated_by);
        builder.push(", updated_at = NOW()");
        
        if request.name.is_some() { builder.push(", name = ").push_bind(request.name); }
        if request.contact_person.is_some() { builder.push(", contact_person = ").push_bind(request.contact_person); }
        if request.email.is_some() { builder.push(", email = ").push_bind(request.email); }
        if request.phone.is_some() { builder.push(", phone = ").push_bind(request.phone); }
        if request.mobile.is_some() { builder.push(", mobile = ").push_bind(request.mobile); }
        if request.tax_id.is_some() { builder.push(", tax_id = ").push_bind(request.tax_id); }
        if request.billing_address.is_some() { builder.push(", billing_address = ").push_bind(request.billing_address); }
        if request.billing_city.is_some() { builder.push(", billing_city = ").push_bind(request.billing_city); }
        if request.billing_state.is_some() { builder.push(", billing_state = ").push_bind(request.billing_state); }
        if request.billing_country.is_some() { builder.push(", billing_country = ").push_bind(request.billing_country); }
        if request.billing_postal_code.is_some() { builder.push(", billing_postal_code = ").push_bind(request.billing_postal_code); }
        if request.shipping_address.is_some() { builder.push(", shipping_address = ").push_bind(request.shipping_address); }
        if request.shipping_city.is_some() { builder.push(", shipping_city = ").push_bind(request.shipping_city); }
        if request.shipping_state.is_some() { builder.push(", shipping_state = ").push_bind(request.shipping_state); }
        if request.shipping_country.is_some() { builder.push(", shipping_country = ").push_bind(request.shipping_country); }
        if request.shipping_postal_code.is_some() { builder.push(", shipping_postal_code = ").push_bind(request.shipping_postal_code); }
        if request.credit_limit.is_some() { builder.push(", credit_limit = ").push_bind(request.credit_limit); }
        if request.credit_days.is_some() { builder.push(", credit_days = ").push_bind(request.credit_days); }
        if request.discount_percentage.is_some() { builder.push(", discount_percentage = ").push_bind(request.discount_percentage); }
        if request.is_active.is_some() { builder.push(", is_active = ").push_bind(request.is_active); }
        if request.tags.is_some() { builder.push(", tags = ").push_bind(request.tags); }
        if request.notes.is_some() { builder.push(", notes = ").push_bind(request.notes); }
        if request.metadata.is_some() { builder.push(", metadata = ").push_bind(request.metadata); }
        
        builder.push(" WHERE id = ").push_bind(id);
        builder.push(" RETURNING *");
        builder.build_query_as::<Customer>().fetch_one(pool).await
    }
    
    // ===== DELETE CUSTOMER (SOFT) (Postgres) =====
    pub async fn delete_pg(pool: &PgPool, id: Uuid, updated_by: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE customers SET is_active = false, updated_by = $1, updated_at = NOW() WHERE id = $2")
            .bind(updated_by)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
    
    // ===== GET WITH STATS (Postgres) =====
    pub async fn get_with_stats_pg(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<CustomerWithStats>, sqlx::Error> {
        let row = sqlx::query_as::<Postgres, CustomerWithStatsRow>(
            r#"
            SELECT 
                c.*,
                COUNT(i.id) as total_invoices,
                COALESCE(SUM(i.total_amount), 0) as total_sales,
                COALESCE(SUM(i.balance_due), 0) as outstanding_balance
            FROM customers c
            LEFT JOIN sales_invoices i ON c.id = i.customer_id AND i.status != 'draft'
            WHERE c.id = $1
            GROUP BY c.id
            "#
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(row.map(|r| CustomerWithStats {
            customer: Customer {
                id: r.id,
                company_id: r.company_id,
                customer_code: r.customer_code,
                name: r.name,
                contact_person: r.contact_person,
                email: r.email,
                phone: r.phone,
                mobile: r.mobile,
                tax_id: r.tax_id,
                billing_address: r.billing_address,
                billing_city: r.billing_city,
                billing_state: r.billing_state,
                billing_country: r.billing_country,
                billing_postal_code: r.billing_postal_code,
                shipping_address: r.shipping_address,
                shipping_city: r.shipping_city,
                shipping_state: r.shipping_state,
                shipping_country: r.shipping_country,
                shipping_postal_code: r.shipping_postal_code,
                credit_limit: r.credit_limit,
                credit_days: r.credit_days,
                discount_percentage: r.discount_percentage,
                is_active: r.is_active,
                tags: r.tags,
                notes: r.notes,
                metadata: r.metadata,
                created_at: r.created_at,
                updated_at: r.updated_at,
                created_by: r.created_by,
                updated_by: r.updated_by,
            },
            total_invoices: r.total_invoices,
            total_sales: r.total_sales,
            outstanding_balance: r.outstanding_balance,
        }))
    }
}

// ===== POSTGRES ROW STRUCT FOR STATS =====
#[derive(Debug, sqlx::FromRow)]
struct CustomerWithStatsRow {
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
    pub credit_limit: Decimal,
    pub credit_days: i32,
    pub discount_percentage: Decimal,
    pub is_active: bool,
    pub tags: Vec<String>,
    pub notes: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    // Stats fields
    pub total_invoices: i64,
    pub total_sales: Decimal,
    pub outstanding_balance: Decimal,
}
