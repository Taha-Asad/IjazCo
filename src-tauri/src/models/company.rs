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
    pub code: String,
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
    pub currency: String,
    pub timezone: String,
    pub is_active: bool,
    #[sqlx(default)]
    pub settings: serde_json::Value,
    #[sqlx(default)]
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateCompanyRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 1, max = 50))]
    pub code: String,
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
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateCompanyRequest {
    pub name: Option<String>,
    pub is_active: Option<bool>,
    pub settings: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

impl Company {
    // ===== CREATE =====
    pub async fn create_pg(pool: &PgPool, req: CreateCompanyRequest, creator: Uuid) -> Result<Company, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<Postgres, Company>(
            r#"INSERT INTO companies (id, name, code, registration_number, tax_id, address, city, state, country, postal_code, phone, email, website, logo_url, currency, timezone, is_active, settings, metadata, created_by, updated_by)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21) RETURNING *"#
        )
        .bind(id).bind(req.name).bind(req.code).bind(req.registration_number).bind(req.tax_id).bind(req.address).bind(req.city).bind(req.state).bind(req.country).bind(req.postal_code).bind(req.phone).bind(req.email).bind(req.website).bind(req.logo_url)
        .bind(req.currency.unwrap_or_else(|| "USD".to_string())).bind(req.timezone.unwrap_or_else(|| "UTC".to_string())).bind(true).bind(req.settings.unwrap_or_default()).bind(req.metadata.unwrap_or_default()).bind(creator).bind(creator)
        .fetch_one(pool).await
    }

    pub async fn create_sqlite(pool: &SqlitePool, req: CreateCompanyRequest, creator: Uuid) -> Result<Company, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<Sqlite, Company>(
            r#"INSERT INTO companies (id, name, code, registration_number, tax_id, address, city, state, country, postal_code, phone, email, website, logo_url, currency, timezone, is_active, settings, metadata, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING *"#
        )
        .bind(id).bind(req.name).bind(req.code).bind(req.registration_number).bind(req.tax_id).bind(req.address).bind(req.city).bind(req.state).bind(req.country).bind(req.postal_code).bind(req.phone).bind(req.email).bind(req.website).bind(req.logo_url)
        .bind(req.currency.unwrap_or_else(|| "USD".to_string())).bind(req.timezone.unwrap_or_else(|| "UTC".to_string())).bind(1).bind(req.settings.unwrap_or_default()).bind(req.metadata.unwrap_or_default()).bind(creator).bind(creator)
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
        sqlx::query_as::<Postgres, Company>(
            "UPDATE companies SET name = COALESCE($1, name), is_active = COALESCE($2, is_active), updated_by = $3, updated_at = NOW() WHERE id = $4 RETURNING *"
        )
        .bind(req.name).bind(req.is_active).bind(user).bind(id).fetch_one(pool).await
    }

    pub async fn update_sqlite(pool: &SqlitePool, id: Uuid, req: UpdateCompanyRequest, user: Uuid) -> Result<Company, sqlx::Error> {
        sqlx::query_as::<Sqlite, Company>(
            "UPDATE companies SET name = COALESCE(?, name), is_active = COALESCE(?, is_active), updated_by = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? RETURNING *"
        )
        .bind(req.name).bind(req.is_active).bind(user).bind(id).fetch_one(pool).await
    }
}