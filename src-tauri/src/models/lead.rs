// src/models/lead.rs
// Lead management models
// Handles lead information, status tracking, and conversion

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;
use sqlx::{PgPool, SqlitePool};
use rust_decimal::prelude::FromPrimitive; // 👈 ADD THIS for Decimal::from_f64

// ===== LEAD STATUS ENUM =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "lead_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum LeadStatus {
    New,
    Contacted,
    Qualified,
    Proposal,
    Negotiation,
    Won,
    Lost,
}

// ===== LEAD SOURCE ENUM =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "lead_source", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum LeadSource {
    Website,
    Referral,
    ColdCall,
    SocialMedia,
    Email,
    Other,
}

// ===== LEAD MODEL =====
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Lead {
    pub id: Uuid,
    pub company_id: Uuid,

    #[schema(example = "LEAD-001")]
    pub lead_number: String,

    #[schema(example = "John Doe")]
    pub name: String,

    #[schema(example = "john@example.com")]
    pub email: Option<String>,

    #[schema(example = "+1-555-1234")]
    pub phone: Option<String>,

    pub company_name: Option<String>,

    pub status: LeadStatus,

    pub source: LeadSource,

    #[schema(value_type = f64, example = 5000.00)]
    pub estimated_value: Option<sqlx::types::Decimal>,

    #[schema(example = "Need 50 microscopes for new lab")]
    pub description: Option<String>,

    pub assigned_to: Option<Uuid>,

    pub converted_to_customer: Option<Uuid>,

    pub expected_close_date: Option<chrono::NaiveDate>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// ===== LEAD SQLITE INTERMEDIATE STRUCT =====
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LeadSqlite {
    pub id: Uuid,
    pub company_id: Uuid,
    pub lead_number: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub company_name: Option<String>,
    pub status: String,
    pub source: String,
    pub estimated_value: Option<f64>,
    pub description: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub converted_to_customer: Option<Uuid>,
    pub expected_close_date: Option<chrono::NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// ===== LEAD WITH DETAILS =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LeadWithDetails {
    #[serde(flatten)]
    pub lead: Lead,

    pub assigned_to_name: Option<String>,
    pub created_by_name: Option<String>,
}

// ===== CREATE LEAD REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateLeadRequest {
    pub company_id: Uuid,

    #[validate(length(min = 1, max = 100))]
    #[schema(example = "John Doe")]
    pub name: String,

    #[validate(email)]
    #[schema(example = "john@example.com")]
    pub email: Option<String>,

    #[schema(example = "+1-555-1234")]
    pub phone: Option<String>,

    #[schema(example = "ABC Corp")]
    pub company_name: Option<String>,

    #[schema(example = "new")]
    pub status: Option<LeadStatus>,

    #[schema(example = "website")]
    pub source: Option<LeadSource>,

    #[schema(value_type = f64, example = 5000.00)]
    pub estimated_value: Option<sqlx::types::Decimal>,

    #[schema(example = "Need 50 microscopes for new lab")]
    pub description: Option<String>,

    pub assigned_to: Option<Uuid>,

    pub expected_close_date: Option<chrono::NaiveDate>,
}

// ===== UPDATE LEAD REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateLeadRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,

    #[validate(email)]
    pub email: Option<String>,

    pub phone: Option<String>,

    pub company_name: Option<String>,

    pub status: Option<LeadStatus>,

    pub source: Option<LeadSource>,

    #[schema(value_type = f64)]
    pub estimated_value: Option<sqlx::types::Decimal>,

    pub description: Option<String>,

    pub assigned_to: Option<Uuid>,

    pub expected_close_date: Option<chrono::NaiveDate>,
}

// ===== POSTGRES IMPLEMENTATIONS =====
impl Lead {
    // List leads by company
    pub async fn list_by_company_pg(
        pool: &PgPool,
        company_id: Uuid,
        status: Option<LeadStatus>,
        limit: i64,
        offset: i64,
    ) -> sqlx::Result<Vec<Lead>> {
        let status_filter = status.as_ref().map(|s| format!("{:?}", s).to_lowercase());

        sqlx::query_as::<_, Lead>(
            r#"
            SELECT * FROM leads 
            WHERE company_id = $1 
            AND ($2::text IS NULL OR LOWER(status::text) = $2)
            ORDER BY created_at DESC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(company_id)
        .bind(status_filter)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }

    // Find by ID
    pub async fn find_by_id_pg(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<Lead>> {
        sqlx::query_as::<_, Lead>("SELECT * FROM leads WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    // Create lead
    pub async fn create_pg(
        pool: &PgPool,
        req: CreateLeadRequest,
        user_id: Uuid,
    ) -> sqlx::Result<Lead> {
        let lead_number = format!(
            "LEAD-{}",
            uuid::Uuid::new_v4().simple().to_string()[..8].to_uppercase()
        );

        sqlx::query_as::<_, Lead>(
            r#"
            INSERT INTO leads (company_id, lead_number, name, email, phone, company_name, 
                            status, source, estimated_value, description, assigned_to, 
                            expected_close_date, created_by, updated_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $13)
            RETURNING *
            "#,
        )
        .bind(req.company_id)
        .bind(lead_number)
        .bind(req.name)
        .bind(req.email)
        .bind(req.phone)
        .bind(req.company_name)
        .bind(req.status.unwrap_or(LeadStatus::New))
        .bind(req.source.unwrap_or(LeadSource::Other))
        .bind(req.estimated_value)
        .bind(req.description)
        .bind(req.assigned_to)
        .bind(req.expected_close_date)
        .bind(user_id)
        .fetch_one(pool)
        .await
    }

    // Update lead
    pub async fn update_pg(
        pool: &PgPool,
        id: Uuid,
        req: UpdateLeadRequest,
        user_id: Uuid,
    ) -> sqlx::Result<Lead> {
        sqlx::query_as::<_, Lead>(
            r#"
            UPDATE leads 
            SET name = COALESCE($2, name),
                email = COALESCE($3, email),
                phone = COALESCE($4, phone),
                company_name = COALESCE($5, company_name),
                status = COALESCE($6, status),
                source = COALESCE($7, source),
                estimated_value = COALESCE($8, estimated_value),
                description = COALESCE($9, description),
                assigned_to = COALESCE($10, assigned_to),
                expected_close_date = COALESCE($11, expected_close_date),
                updated_by = $12,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(req.name)
        .bind(req.email)
        .bind(req.phone)
        .bind(req.company_name)
        .bind(req.status)
        .bind(req.source)
        .bind(req.estimated_value)
        .bind(req.description)
        .bind(req.assigned_to)
        .bind(req.expected_close_date)
        .bind(user_id)
        .fetch_one(pool)
        .await
    }

    // Delete lead
    pub async fn delete_pg(pool: &PgPool, id: Uuid) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM leads WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

// ===== SQLITE IMPLEMENTATIONS =====
impl LeadSqlite {
    pub async fn list_by_company_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
        status: Option<LeadStatus>,
        limit: i64,
        offset: i64,
    ) -> sqlx::Result<Vec<Lead>> {
        let status_str = status.as_ref().map(|s| format!("{:?}", s).to_lowercase());

        let rows = sqlx::query_as::<_, LeadSqlite>(
            r#"
            SELECT * FROM leads 
            WHERE company_id = ? 
            AND (? IS NULL OR LOWER(status) = ?)
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(company_id)
        .bind(&status_str)
        .bind(&status_str)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn find_by_id_sqlite(
        pool: &SqlitePool,
        id: Uuid,
    ) -> sqlx::Result<Option<Lead>> {
        let row = sqlx::query_as::<_, LeadSqlite>("SELECT * FROM leads WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(row.map(|r| r.into()))
    }
}

// ===== CONVERSION FROM SQLITE TO MAIN MODEL =====
impl From<LeadSqlite> for Lead {
    fn from(s: LeadSqlite) -> Self {
        Lead {
            id: s.id,
            company_id: s.company_id,
            lead_number: s.lead_number,
            name: s.name,
            email: s.email,
            phone: s.phone,
            company_name: s.company_name,
            status: serde_json::from_value(serde_json::json!(s.status))
                .unwrap_or(LeadStatus::New),
            source: serde_json::from_value(serde_json::json!(s.source))
                .unwrap_or(LeadSource::Other),
            // 👇 FIXED: Use from_f64_retain for safer conversion (no panic on NaN/Inf)
            estimated_value: s
                .estimated_value
                .and_then(sqlx::types::Decimal::from_f64_retain),
            description: s.description,
            converted_to_customer: s.converted_to_customer,
            assigned_to: s.assigned_to,
            expected_close_date: s.expected_close_date,
            created_at: s.created_at,
            updated_at: s.updated_at,
            created_by: s.created_by,
            updated_by: s.updated_by,
        }
    }
}