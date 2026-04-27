// src/models/supplier.rs
// Supplier relationship management models
// Handles supplier information for procurement

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Postgres, Sqlite};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
// ===== SUPPLIER MODEL =====
// Main supplier entity for purchase transactions
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema , FromRow)]
pub struct Supplier {
    // Primary key
    pub id: Uuid,
    
    // Foreign key - company/tenant
    pub company_id: Uuid,
    
    // Unique supplier code (e.g., SUPP-001)
    #[schema(example = "SUPP-001")]
    pub supplier_code: String,
    
    // Supplier name
    #[schema(example = "Scientific Equipment Co.")]
    pub name: String,
    
    // Primary contact person
    #[schema(example = "Jane Doe")]
    pub contact_person: Option<String>,
    
    // Email address
    #[schema(example = "sales@sciequipco.com")]
    pub email: Option<String>,
    
    // Phone number
    #[schema(example = "+92-300-0000000")]
    pub phone: Option<String>,
    
    // Website URL
    #[schema(example = "https://sciequipco.com")]
    pub website: Option<String>,
    
    // Tax ID / VAT number
    #[schema(example = "VAT-987654321")]
    pub tax_id: Option<String>,
    
    // Physical address
    pub address: Option<String>,
    
    // City
    #[schema(example = "New York")]
    pub city: Option<String>,
    
    // State/province
    #[schema(example = "New York")]
    pub state: Option<String>,
    
    // Country
    #[schema(example = "USA")]
    pub country: Option<String>,
    
    // Postal code
    #[schema(example = "10001")]
    pub postal_code: Option<String>,
    
    // Payment terms in days (e.g., Net 30)
    #[schema(example = 30)]
    pub payment_terms: i32,
    
    // Standard lead time in days
    #[schema(example = 14)]
    pub lead_time_days: i32,
    
    // Supplier rating (0.0 to 5.0)
    #[schema(value_type = f64, example = 4.5)]
    pub rating: Option<Decimal>,
    
    // Active supplier flag
    pub is_active: bool,
    
    // Supplier tags
    pub tags: sqlx::types::Json<Vec<String>>,
    
    // Internal notes
    pub notes: Option<String>,
    
    // Custom metadata
    pub metadata: serde_json::Value,
    
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}



// ===== SQLITE INTERMEDIATE STRUCT (uses f64 for Decimal fields) =====
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SupplierSqlite {
    // Primary key
    pub id: Uuid,
    
    // Foreign key - company/tenant
    pub company_id: Uuid,
    
    // Unique supplier code (e.g., SUPP-001)
    pub supplier_code: String,
    
    // Supplier name
    pub name: String,
    
    // Primary contact person
    pub contact_person: Option<String>,
    
    // Email address
    pub email: Option<String>,
    
    // Phone number
    pub phone: Option<String>,
    
    // Website URL
    pub website: Option<String>,
    
    // Tax ID / VAT number
    pub tax_id: Option<String>,
    
    // Physical address
    pub address: Option<String>,
    
    // City
    pub city: Option<String>,
    
    // State/province
    pub state: Option<String>,
    
    // Country
    pub country: Option<String>,
    
    // Postal code
    pub postal_code: Option<String>,
    
    // Payment terms in days (e.g., Net 30)
    pub payment_terms: i32,
    
    // Standard lead time in days
    pub lead_time_days: i32,
    
    // Supplier rating (0.0 to 5.0) - stored as f64 in SQLite
    pub rating: Option<f64>,
    
    // Active supplier flag
    pub is_active: bool,
    
    // Supplier tags
    pub tags: sqlx::types::Json<Vec<String>>,
    
    // Internal notes
    pub notes: Option<String>,
    
    // Custom metadata
    pub metadata: serde_json::Value,
    
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// ===== SUPPLIER WITH STATISTICS =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SupplierWithStats {
    // Base supplier data
    #[serde(flatten)]
    pub supplier: Supplier,
    
    // Total purchase orders
    pub total_purchase_orders: i64,
    
    // Total purchase value
    #[schema(value_type = f64, example = 500000.00)]
    pub total_purchases: Decimal,
    
    // Last purchase date
    pub last_purchase_date: Option<chrono::NaiveDate>,
    
    // Average order value
    #[schema(value_type = f64, example = 25000.00)]
    pub average_order_value: Decimal,
}

// ===== CREATE SUPPLIER REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateSupplierRequest {
    // Company ID
    pub company_id: Uuid,
    
    // Supplier code (unique within company)
    #[validate(length(min = 1, max = 50))]
    #[schema(example = "SUPP-001")]
    pub supplier_code: String,
    
    // Supplier name (required)
    #[validate(length(min = 1, max = 255))]
    #[schema(example = "Scientific Equipment Co.")]
    pub name: String,
    
    // Contact person (optional)
    pub contact_person: Option<String>,
    
    // Email (optional)
    #[validate(email)]
    pub email: Option<String>,
    
    // Phone (optional)
    pub phone: Option<String>,
    
    // Website (optional)
    #[validate(url)]
    pub website: Option<String>,
    
    // Tax ID (optional)
    pub tax_id: Option<String>,
    
    // Address fields
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    
    // Payment terms (default 30 days)
    #[validate(range(min = 0))]
    pub payment_terms: Option<i32>,
    
    // Lead time (default 7 days)
    #[validate(range(min = 0))]
    pub lead_time_days: Option<i32>,
    
    // Rating (0-5) - use custom validator instead of range
    #[validate(custom = "validate_rating")]
    pub rating: Option<Decimal>,
    
    // Tags (optional)
    pub tags: Option<Vec<String>>,
    
    // Notes (optional)
    pub notes: Option<String>,
    
    // Metadata (optional)
    pub metadata: Option<serde_json::Value>,
}

// ===== UPDATE SUPPLIER REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateSupplierRequest {
    // All fields optional for partial updates
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    
    pub contact_person: Option<String>,
    
    #[validate(email)]
    pub email: Option<String>,
    
    pub phone: Option<String>,
    
    #[validate(url)]
    pub website: Option<String>,
    
    pub tax_id: Option<String>,
    
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    
    pub payment_terms: Option<i32>,
    pub lead_time_days: Option<i32>,
    
    // Rating (0-5) - use custom validator
    #[validate(custom = "validate_rating")]
    pub rating: Option<Decimal>,
    
    pub is_active: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

// ===== RATING VALIDATION =====
// Custom validator for rating (0.0 to 5.0)
fn validate_rating(rating: &Decimal) -> Result<(), validator::ValidationError> {
    if *rating >= Decimal::ZERO && *rating <= Decimal::new(5, 0) {
        Ok(())
    } else {
        Err(validator::ValidationError::new(
            "Rating must be between 0.0 and 5.0"
        ))
    }
}


impl From<SupplierSqlite> for Supplier {
    fn from(s: SupplierSqlite) -> Self {
        Self {
            id: s.id,
            company_id: s.company_id,
            supplier_code: s.supplier_code,
            name: s.name,
            contact_person: s.contact_person,
            email: s.email,
            phone: s.phone,
            website: s.website,
            tax_id: s.tax_id,
            address: s.address,
            city: s.city,
            state: s.state,
            country: s.country,
            postal_code: s.postal_code,
            payment_terms: s.payment_terms,
            lead_time_days: s.lead_time_days,
            // Convert f64 back to Decimal
            rating: s.rating.map(|r| Decimal::from_f64_retain(r).unwrap_or_default()),
            is_active: s.is_active,
            tags: s.tags,
            notes: s.notes,
            metadata: s.metadata,
            created_at: s.created_at,
            updated_at: s.updated_at,
            created_by: s.created_by,
            updated_by: s.updated_by,
        }
    }
}

// ===== SUPPLIER DATABASE OPERATIONS =====
impl Supplier {
    // ===== CREATE SUPPLIER =====
    pub async fn create_pg(
        pool: &sqlx::PgPool,
        request: CreateSupplierRequest,
        created_by: Uuid,
    ) -> Result<Supplier, sqlx::Error> {
        let supplier = sqlx::query_as::<Postgres, Supplier>(
            r#"
            INSERT INTO suppliers (
                company_id, supplier_code, name, contact_person, email,
                phone, website, tax_id, address, city, state, country,
                postal_code, payment_terms, lead_time_days, rating,
                is_active, tags, notes, metadata, created_by, updated_by
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12,
                $13, $14, $15, $16, $17, $18, $19, $20, $21, $22
            )
            RETURNING *
            "#
        )
        .bind(request.company_id)
        .bind(request.supplier_code)
        .bind(request.name)
        .bind(request.contact_person)
        .bind(request.email)
        .bind(request.phone)
        .bind(request.website)
        .bind(request.tax_id)
        .bind(request.address)
        .bind(request.city)
        .bind(request.state)
        .bind(request.country)
        .bind(request.postal_code)
        .bind(request.payment_terms.unwrap_or(30))
        .bind(request.lead_time_days.unwrap_or(7))
        .bind(request.rating)
        .bind(true) // is_active = true
        .bind(request.tags.unwrap_or_default())
        .bind(request.notes)
        .bind(request.metadata.unwrap_or(serde_json::json!({})))
        .bind(created_by)
        .bind(created_by)
        .fetch_one(pool)
        .await?;
        
        Ok(supplier)
    }
    
    pub async fn create_sqlite(
        pool: &sqlx::SqlitePool,
        request: CreateSupplierRequest,
        created_by: Uuid,
    ) -> Result<Supplier, sqlx::Error> {
    let rating_f64 = request.rating.map(|d| d.to_f64().unwrap_or_default());

        let supplier_sqlite = sqlx::query_as::<Sqlite, SupplierSqlite>(
            r#"
            INSERT INTO suppliers (
                company_id, supplier_code, name, contact_person, email,
                phone, website, tax_id, address, city, state, country,
                postal_code, payment_terms, lead_time_days, rating,
                is_active, tags, notes, metadata, created_by, updated_by
            )
            VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
            RETURNING *
            "#
        )
        .bind(request.company_id)
        .bind(request.supplier_code)
        .bind(request.name)
        .bind(request.contact_person)
        .bind(request.email)
        .bind(request.phone)
        .bind(request.website)
        .bind(request.tax_id)
        .bind(request.address)
        .bind(request.city)
        .bind(request.state)
        .bind(request.country)
        .bind(request.postal_code)
        .bind(request.payment_terms.unwrap_or(30))
        .bind(request.lead_time_days.unwrap_or(7))
        .bind(rating_f64)
        .bind(1) // is_active = true
        .bind(serde_json::json!(request.tags.unwrap_or_default()))
        .bind(request.notes)
        .bind(request.metadata.unwrap_or(serde_json::json!({})))
        .bind(created_by)
        .bind(created_by)
        .fetch_one(pool)
        .await?;
        
        Ok(Supplier::from(supplier_sqlite))
    }
    
    // ===== FIND SUPPLIER BY ID =====
    pub async fn find_by_id_pg(
        pool: &sqlx::PgPool,
        id: Uuid,
    ) -> Result<Option<Supplier>, sqlx::Error> {
        let supplier = sqlx::query_as::<Postgres, Supplier>(
            "SELECT * FROM suppliers WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(supplier)
    }
    pub async fn find_by_id_sqlite(
        pool: &sqlx::SqlitePool,
        id: Uuid,
    ) -> Result<Option<Supplier>, sqlx::Error> {
        let supplier = sqlx::query_as::<Sqlite, SupplierSqlite>(
            "SELECT * FROM suppliers WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(supplier.map(Supplier::from))
    }
    
    // ===== LIST SUPPLIERS BY COMPANY =====
    pub async fn list_by_company_sqlite(
        pool: &sqlx::SqlitePool,
        company_id: Uuid,
        active_only: bool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Supplier>, sqlx::Error> {
        let suppliers = if active_only {
            sqlx::query_as::<Sqlite, SupplierSqlite>(
                r#"
                SELECT * FROM suppliers
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
            sqlx::query_as::<Sqlite, SupplierSqlite>(
                r#"
                SELECT * FROM suppliers
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
        
        Ok(suppliers.into_iter().map(Supplier::from).collect())
    }

        pub async fn list_by_company_pg(
        pool: &sqlx::PgPool,
        company_id: Uuid,
        active_only: bool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Supplier>, sqlx::Error> {
        let suppliers = if active_only {
            sqlx::query_as::<Postgres, Supplier>(
                r#"
                SELECT * FROM suppliers
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
            sqlx::query_as::<Postgres, Supplier>(
                r#"
                SELECT * FROM suppliers
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
        
        Ok(suppliers)
    }

    // ===== SEARCH SUPPLIERS =====
    pub async fn search_pg(
        pool: &sqlx::PgPool,
        company_id: Uuid,
        search_term: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Supplier>, sqlx::Error> {
        let suppliers = sqlx::query_as::<_, Supplier>(
            r#"
            SELECT * FROM suppliers
            WHERE company_id = $1
              AND is_active = true
              AND (
                name ILIKE $2
                OR supplier_code ILIKE $2
                OR email ILIKE $2
              )
            ORDER BY name
            LIMIT $3 OFFSET $4
            "#
        )
        .bind(company_id)
        .bind(format!("%{}%", search_term))
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(suppliers)
    }
    
        pub async fn search_sqlite(
        pool: &sqlx::SqlitePool,
        company_id: Uuid,
        search_term: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Supplier>, sqlx::Error> {
        let suppliers = sqlx::query_as::<Sqlite, SupplierSqlite>(
            r#"
            SELECT * FROM suppliers
            WHERE company_id = ?
              AND is_active = true
              AND (
                name LIKE ?
                OR supplier_code LIKE ?
                OR email LIKE ?
              )
            ORDER BY name
            LIMIT ? OFFSET ?
            "#
        )
        .bind(company_id)
        .bind(format!("%{}%", search_term))
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(suppliers.into_iter().map(Supplier::from).collect())
    }
    
    pub async fn update_pg(
        pool: &sqlx::PgPool,
        id: Uuid,
        request: UpdateSupplierRequest,
        updated_by: Uuid,
    ) -> Result<Supplier, sqlx::Error> {
        let mut builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("UPDATE suppliers SET ");
        
        // Standard fields
        builder.push("updated_by = ").push_bind(updated_by);
        builder.push(", updated_at = NOW()");

        // Dynamic fields
        if let Some(name) = &request.name {
            builder.push(", name = ").push_bind(name);
        }
        if let Some(contact_person) = &request.contact_person {
            builder.push(", contact_person = ").push_bind(contact_person);
        }
        if let Some(email) = &request.email {
            builder.push(", email = ").push_bind(email);
        }
        if let Some(phone) = &request.phone {
            builder.push(", phone = ").push_bind(phone);
        }
        if let Some(website) = &request.website {
            builder.push(", website = ").push_bind(website);
        }
        if let Some(tax_id) = &request.tax_id {
            builder.push(", tax_id = ").push_bind(tax_id);
        }
        if let Some(address) = &request.address {
            builder.push(", address = ").push_bind(address);
        }
        if let Some(city) = &request.city {
            builder.push(", city = ").push_bind(city);
        }
        if let Some(state) = &request.state {
            builder.push(", state = ").push_bind(state);
        }
        if let Some(country) = &request.country {
            builder.push(", country = ").push_bind(country);
        }
        if let Some(postal_code) = &request.postal_code {
            builder.push(", postal_code = ").push_bind(postal_code);
        }
        if let Some(payment_terms) = request.payment_terms {
            builder.push(", payment_terms = ").push_bind(payment_terms);
        }
        if let Some(lead_time_days) = request.lead_time_days {
            builder.push(", lead_time_days = ").push_bind(lead_time_days);
        }
        if let Some(rating) = request.rating {
            builder.push(", rating = ").push_bind(rating);
        }
        if let Some(is_active) = request.is_active {
            builder.push(", is_active = ").push_bind(is_active);
        }
        if let Some(tags) = &request.tags {
            builder.push(", tags = ").push_bind(tags);
        }
        if let Some(notes) = &request.notes {
            builder.push(", notes = ").push_bind(notes);
        }
        if let Some(metadata) = &request.metadata {
            builder.push(", metadata = ").push_bind(metadata);
        }
        
        // Finalize query
        builder.push(" WHERE id = ").push_bind(id);
        builder.push(" RETURNING *");
        
        let supplier = builder.build_query_as::<Supplier>().fetch_one(pool).await?;
        Ok(supplier)
    }
        pub async fn update_sqlite(
        pool: &sqlx::SqlitePool,
        id: Uuid,
        request: UpdateSupplierRequest,
        updated_by: Uuid,
    ) -> Result<Supplier, sqlx::Error> {
        let mut builder = sqlx::QueryBuilder::<sqlx::Sqlite>::new("UPDATE suppliers SET ");
        
        builder.push("updated_by = ").push_bind(updated_by);
        builder.push(", updated_at = CURRENT_TIMESTAMP");

        if let Some(name) = &request.name {
            builder.push(", name = ").push_bind(name);
        }
        if let Some(contact_person) = &request.contact_person {
            builder.push(", contact_person = ").push_bind(contact_person);
        }
        if let Some(email) = &request.email {
            builder.push(", email = ").push_bind(email);
        }
        if let Some(phone) = &request.phone {
            builder.push(", phone = ").push_bind(phone);
        }
        if let Some(website) = &request.website {
            builder.push(", website = ").push_bind(website);
        }
        if let Some(tax_id) = &request.tax_id {
            builder.push(", tax_id = ").push_bind(tax_id);
        }
        if let Some(address) = &request.address {
            builder.push(", address = ").push_bind(address);
        }
        if let Some(city) = &request.city {
            builder.push(", city = ").push_bind(city);
        }
        if let Some(state) = &request.state {
            builder.push(", state = ").push_bind(state);
        }
        if let Some(country) = &request.country {
            builder.push(", country = ").push_bind(country);
        }
        if let Some(postal_code) = &request.postal_code {
            builder.push(", postal_code = ").push_bind(postal_code);
        }
        if let Some(payment_terms) = request.payment_terms {
            builder.push(", payment_terms = ").push_bind(payment_terms);
        }
        if let Some(lead_time_days) = request.lead_time_days {
            builder.push(", lead_time_days = ").push_bind(lead_time_days);
        }
        
        // SQLite Specific Conversions
        if let Some(rating) = request.rating {
            use rust_decimal::prelude::ToPrimitive;
            builder.push(", rating = ").push_bind(rating.to_f64().unwrap_or_default());
        }
        if let Some(is_active) = request.is_active {
            builder.push(", is_active = ").push_bind(if is_active { 1 } else { 0 });
        }
        if let Some(tags) = &request.tags {
            builder.push(", tags = ").push_bind(serde_json::json!(tags));
        }
        
        if let Some(notes) = &request.notes {
            builder.push(", notes = ").push_bind(notes);
        }
        if let Some(metadata) = &request.metadata {
            builder.push(", metadata = ").push_bind(metadata);
        }
        
        builder.push(" WHERE id = ").push_bind(id);
        builder.push(" RETURNING *");

        // Use the Intermediate Sqlite struct then convert back to Supplier
        let supplier_sqlite = builder.build_query_as::<SupplierSqlite>().fetch_one(pool).await?;
        Ok(Supplier::from(supplier_sqlite))
    }
    // ===== DELETE SUPPLIER =====
    pub async fn delete_pg(
        pool: &sqlx::PgPool,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE suppliers SET is_active = false WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        
        Ok(())
    }
    pub async fn delete_sqlite(
        pool: &sqlx::SqlitePool,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE suppliers SET is_active = false WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        
        Ok(())
    }
    // ===== GET SUPPLIER WITH STATISTICS =====
    pub async fn get_with_stats_pg(
        pool: &sqlx::PgPool,
        id: Uuid,
    ) -> Result<Option<SupplierWithStats>, sqlx::Error> {
        let supplier = match Self::find_by_id_pg(pool, id).await? {
            Some(s) => s,
            None => return Ok(None),
        };
        
        // Fetch statistics separately to avoid Decimal type issues
        let stats: (i64, Option<rust_decimal::Decimal>, Option<chrono::NaiveDate>, Option<rust_decimal::Decimal>) = 
            sqlx::query_as(
                r#"
                SELECT 
                    COUNT(*) as total_orders,
                    COALESCE(SUM(total_amount), 0) as total_purchases,
                    MAX(po_date) as last_purchase_date,
                    COALESCE(AVG(total_amount), 0) as average_order_value
                FROM purchase_orders
                WHERE supplier_id = $1 AND status != 'cancelled'
                "#
            )
            .bind(id)
            .fetch_one(pool)
            .await?;
        
        Ok(Some(SupplierWithStats {
            supplier,
            total_purchase_orders: stats.0,
            total_purchases: stats.1.unwrap_or(Decimal::ZERO),
            last_purchase_date: stats.2,
            average_order_value: stats.3.unwrap_or(Decimal::ZERO),
        }))
    }


    pub async fn get_with_stats_sqlite(
        pool: &sqlx::SqlitePool,
        id: Uuid,
    ) -> Result<Option<SupplierWithStats>, sqlx::Error> {
        let supplier = match Self::find_by_id_sqlite(pool, id).await? {
            Some(s) => s,
            None => return Ok(None),
        };
        
        // Fetch statistics separately to avoid Decimal type issues
        let stats: (i64, Option<f64>, Option<chrono::NaiveDate>, Option<f64>) = 
            sqlx::query_as(
                r#"
                SELECT 
                    COUNT(*) as total_orders,
                    COALESCE(SUM(total_amount), 0) as total_purchases,
                    MAX(po_date) as last_purchase_date,
                    COALESCE(AVG(total_amount), 0) as average_order_value
                FROM purchase_orders
                WHERE supplier_id = ? AND status != 'cancelled'
                "#
            )
            .bind(id)
            .fetch_one(pool)
            .await?;
        
        Ok(Some(SupplierWithStats {
            supplier,
            total_purchase_orders: stats.0,
            total_purchases: Decimal::from_f64_retain(stats.1.unwrap_or(0.0)).unwrap_or_default(),
            last_purchase_date: stats.2,
            average_order_value: Decimal::from_f64_retain(stats.3.unwrap_or(0.0)).unwrap_or_default(),
        }))
    }


}