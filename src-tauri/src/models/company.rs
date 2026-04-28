// src/models/company.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, SqlitePool, Postgres, Sqlite};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub trade_name: Option<String>,
    pub registration_number: Option<String>,
    pub tax_id: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: String,
    pub postal_code: Option<String>,
    pub phone: Option<String>,
    pub email: String,
    pub website: Option<String>,
    pub logo_url: Option<String>,
    pub currency: String,
    pub timezone: String,
    pub is_active: bool,
    pub subscription_plan: Option<String>,
    pub subscription_expires_at: Option<DateTime<Utc>>,
    pub max_users: Option<i32>,
    pub max_branches: Option<i32>,
    pub features: serde_json::Value,
    pub settings: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateCompanyRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub trade_name: Option<String>,
    pub registration_number: Option<String>,
    pub tax_id: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub phone: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(url)]
    pub website: Option<String>,
    pub logo_url: Option<String>,
    pub currency: Option<String>,
    pub timezone: Option<String>,
    pub settings: Option<serde_json::Value>,
    pub features: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateCompanyRequest {
    pub name: Option<String>,
    pub trade_name: Option<String>,
    pub registration_number: Option<String>,
    pub tax_id: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub logo_url: Option<String>,
    pub currency: Option<String>,
    pub timezone: Option<String>,
    pub is_active: Option<bool>,
    pub settings: Option<serde_json::Value>,
    pub features: Option<serde_json::Value>,
}

impl Company {
    // ===== CREATE =====
    pub async fn create_pg(pool: &PgPool, req: CreateCompanyRequest, creator: Uuid) -> Result<Company, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<Postgres, Company>(
            r#"INSERT INTO companies (id, name, trade_name, registration_number, tax_id, address, city, state, country, postal_code, phone, email, website, logo_url, currency, timezone, is_active, features, settings, created_by, updated_by)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21) RETURNING *"#
        )
        .bind(id).bind(&req.name).bind(&req.trade_name).bind(&req.registration_number).bind(&req.tax_id).bind(&req.address).bind(&req.city).bind(&req.state).bind(&req.country).bind(&req.postal_code).bind(&req.phone).bind(&req.email).bind(&req.website).bind(&req.logo_url)
        .bind(&req.currency.unwrap_or_else(|| "USD".to_string())).bind(&req.timezone.unwrap_or_else(|| "UTC".to_string())).bind(true).bind(req.features.unwrap_or_default()).bind(req.settings.unwrap_or_default()).bind(creator).bind(creator)
        .fetch_one(pool).await
    }

    pub async fn create_sqlite(pool: &SqlitePool, req: CreateCompanyRequest, creator: Uuid) -> Result<Company, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<Sqlite, Company>(
            r#"INSERT INTO companies (id, name, trade_name, registration_number, tax_id, address, city, state, country, postal_code, phone, email, website, logo_url, currency, timezone, is_active, features, settings, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING *"#
        )
        .bind(id).bind(&req.name).bind(&req.trade_name).bind(&req.registration_number).bind(&req.tax_id).bind(&req.address).bind(&req.city).bind(&req.state).bind(&req.country).bind(&req.postal_code).bind(&req.phone).bind(&req.email).bind(&req.website).bind(&req.logo_url)
        .bind(&req.currency.unwrap_or_else(|| "USD".to_string())).bind(&req.timezone.unwrap_or_else(|| "UTC".to_string())).bind(1).bind(req.features.unwrap_or_default()).bind(req.settings.unwrap_or_default()).bind(creator).bind(creator)
        .fetch_one(pool).await
    }

    // ===== FIND BY ID =====
    pub async fn find_by_id_pg(pool: &PgPool, id: Uuid) -> Result<Option<Company>, sqlx::Error> {
        sqlx::query_as::<Postgres, Company>("SELECT * FROM companies WHERE id = $1").bind(id).fetch_optional(pool).await
    }

    pub async fn find_by_id_sqlite(pool: &SqlitePool, id: Uuid) -> Result<Option<Company>, sqlx::Error> {
        sqlx::query_as::<Sqlite, Company>("SELECT * FROM companies WHERE id = ?").bind(id).fetch_optional(pool).await
    }

    // ===== UPDATE =====
    pub async fn update_pg(pool: &PgPool, id: Uuid, req: UpdateCompanyRequest, user: Uuid) -> Result<Company, sqlx::Error> {
        let mut builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("UPDATE companies SET ");
        builder.push("updated_by = ").push_bind(user);
        builder.push(", updated_at = NOW()");
        
        if req.name.is_some() { builder.push(", name = ").push_bind(&req.name); }
        if req.trade_name.is_some() { builder.push(", trade_name = ").push_bind(&req.trade_name); }
        if req.registration_number.is_some() { builder.push(", registration_number = ").push_bind(&req.registration_number); }
        if req.tax_id.is_some() { builder.push(", tax_id = ").push_bind(&req.tax_id); }
        if req.address.is_some() { builder.push(", address = ").push_bind(&req.address); }
        if req.city.is_some() { builder.push(", city = ").push_bind(&req.city); }
        if req.state.is_some() { builder.push(", state = ").push_bind(&req.state); }
        if req.country.is_some() { builder.push(", country = ").push_bind(&req.country); }
        if req.postal_code.is_some() { builder.push(", postal_code = ").push_bind(&req.postal_code); }
        if req.phone.is_some() { builder.push(", phone = ").push_bind(&req.phone); }
        if req.email.is_some() { builder.push(", email = ").push_bind(&req.email); }
        if req.website.is_some() { builder.push(", website = ").push_bind(&req.website); }
        if req.logo_url.is_some() { builder.push(", logo_url = ").push_bind(&req.logo_url); }
        if req.currency.is_some() { builder.push(", currency = ").push_bind(&req.currency); }
        if req.timezone.is_some() { builder.push(", timezone = ").push_bind(&req.timezone); }
        if req.is_active.is_some() { builder.push(", is_active = ").push_bind(req.is_active); }
        if req.settings.is_some() { builder.push(", settings = ").push_bind(req.settings); }
        if req.features.is_some() { builder.push(", features = ").push_bind(req.features); }
        
        builder.push(" WHERE id = ").push_bind(id);
        builder.push(" RETURNING *");
        builder.build_query_as::<Company>().fetch_one(pool).await
    }

    pub async fn update_sqlite(pool: &SqlitePool, id: Uuid, req: UpdateCompanyRequest, user: Uuid) -> Result<Company, sqlx::Error> {
        let mut builder = sqlx::QueryBuilder::<sqlx::Sqlite>::new("UPDATE companies SET ");
        builder.push("updated_by = ").push_bind(user);
        builder.push(", updated_at = CURRENT_TIMESTAMP");
        
        if req.name.is_some() { builder.push(", name = ").push_bind(&req.name); }
        if req.trade_name.is_some() { builder.push(", trade_name = ").push_bind(&req.trade_name); }
        if req.registration_number.is_some() { builder.push(", registration_number = ").push_bind(&req.registration_number); }
        if req.tax_id.is_some() { builder.push(", tax_id = ").push_bind(&req.tax_id); }
        if req.address.is_some() { builder.push(", address = ").push_bind(&req.address); }
        if req.city.is_some() { builder.push(", city = ").push_bind(&req.city); }
        if req.state.is_some() { builder.push(", state = ").push_bind(&req.state); }
        if req.country.is_some() { builder.push(", country = ").push_bind(&req.country); }
        if req.postal_code.is_some() { builder.push(", postal_code = ").push_bind(&req.postal_code); }
        if req.phone.is_some() { builder.push(", phone = ").push_bind(&req.phone); }
        if req.email.is_some() { builder.push(", email = ").push_bind(&req.email); }
        if req.website.is_some() { builder.push(", website = ").push_bind(&req.website); }
        if req.logo_url.is_some() { builder.push(", logo_url = ").push_bind(&req.logo_url); }
        if req.currency.is_some() { builder.push(", currency = ").push_bind(&req.currency); }
        if req.timezone.is_some() { builder.push(", timezone = ").push_bind(&req.timezone); }
        if req.is_active.is_some() { builder.push(", is_active = ").push_bind(if req.is_active.unwrap() { 1 } else { 0 }); }
        if req.settings.is_some() { builder.push(", settings = ").push_bind(req.settings); }
        if req.features.is_some() { builder.push(", features = ").push_bind(req.features); }
        
        builder.push(" WHERE id = ").push_bind(id);
        builder.push(" RETURNING *");
        builder.build_query_as::<Company>().fetch_one(pool).await
    }

    // ===== LIST ALL =====
    pub async fn list_all_pg(pool: &PgPool) -> Result<Vec<Company>, sqlx::Error> {
        sqlx::query_as::<Postgres, Company>("SELECT * FROM companies ORDER BY name")
            .fetch_all(pool).await
    }

    pub async fn list_all_sqlite(pool: &SqlitePool) -> Result<Vec<Company>, sqlx::Error> {
        sqlx::query_as::<Sqlite, Company>("SELECT * FROM companies ORDER BY name")
            .fetch_all(pool).await
    }

    // ===== DELETE (soft) =====
    pub async fn delete_pg(pool: &PgPool, id: Uuid, user: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE companies SET is_active = false, updated_by = $1, updated_at = NOW() WHERE id = $2")
            .bind(user).bind(id).execute(pool).await?;
        Ok(())
    }

    pub async fn delete_sqlite(pool: &SqlitePool, id: Uuid, user: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE companies SET is_active = 0, updated_by = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(user).bind(id).execute(pool).await?;
        Ok(())
    }
}