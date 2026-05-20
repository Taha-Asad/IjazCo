// src/models/branch.rs
// Branch/warehouse management models
// Handles multiple locations for inventory and operations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Sqlite, SqlitePool, PgPool, Postgres};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;

// ===== BRANCH MODEL =====
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Branch {
    pub id: Uuid,
    
    pub company_id: Uuid,
    
    #[schema(example = "Main Warehouse")]
    pub name: String,
    
    #[schema(example = "WH-001")]
    pub code: String,
    
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    
    pub phone: Option<String>,
    pub email: Option<String>,
    
    pub manager_name: Option<String>,
    pub manager_phone: Option<String>,
    
    pub warehouse_type: Option<String>,
    pub storage_capacity: Option<i32>,
    
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

// ===== CREATE BRANCH REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateBranchRequest {
    pub company_id: Uuid,
    
    #[validate(length(min = 1, max = 255))]
    #[schema(example = "Main Warehouse")]
    pub name: String,
    
    #[validate(length(min = 1, max = 50))]
    #[schema(example = "WH-001")]
    pub code: String,
    
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    
    pub phone: Option<String>,
    
    #[validate(email)]
    pub email: Option<String>,
    
    pub manager_name: Option<String>,
    pub manager_phone: Option<String>,
    
    pub warehouse_type: Option<String>,
    pub storage_capacity: Option<i32>,
        
    pub settings: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

// ===== UPDATE BRANCH REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateBranchRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    
    pub code: Option<String>,
    
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    
    pub phone: Option<String>,
    
    #[validate(email)]
    pub email: Option<String>,
    
    pub manager_name: Option<String>,
    pub manager_phone: Option<String>,
    
    pub warehouse_type: Option<String>,
    pub storage_capacity: Option<i32>,
    
    pub is_active: Option<bool>,
    
    pub settings: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

// ===== BRANCH DATABASE OPERATIONS =====
impl Branch {
    // ===== CREATE =====
    pub async fn create_pg(
        pool: &sqlx::PgPool,
        request: CreateBranchRequest,
        created_by: Option<Uuid>,
    ) -> Result<Branch, sqlx::Error> {
        let branch = sqlx::query_as::<sqlx::Postgres, Branch>(
            r#"
            INSERT INTO branches (
                company_id, name, code, address, city, state, country,
                postal_code, phone, email, manager_name, manager_phone,
                warehouse_type, storage_capacity, is_active,
                settings, metadata, created_by, updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)
            RETURNING *
            "#
        )
        .bind(request.company_id)
        .bind(request.name)
        .bind(request.code)
        .bind(request.address)
        .bind(request.city)
        .bind(request.state)
        .bind(request.country)
        .bind(request.postal_code)
        .bind(request.phone)
        .bind(request.email)
        .bind(request.manager_name)
        .bind(request.manager_phone)
        .bind(request.warehouse_type)
        .bind(request.storage_capacity)
        .bind(true)
        .bind(request.settings.unwrap_or(serde_json::json!({})))
        .bind(request.metadata.unwrap_or(serde_json::json!({})))
        .bind(created_by)
        .bind(created_by)
        .fetch_one(pool)
        .await?;
        
        Ok(branch)
    }

    pub async fn create_sqlite(
        pool: &SqlitePool,
        request: CreateBranchRequest,
        created_by: Option<Uuid>,
    ) -> Result<Branch, sqlx::Error> {
        let id = Uuid::new_v4();
        let branch = sqlx::query_as::<Sqlite, Branch>(
            r#"
            INSERT INTO branches (
                id, company_id, name, code, address, city, state, country,
                postal_code, phone, email, manager_name, manager_phone,
                warehouse_type, storage_capacity, is_active,
                settings, metadata, created_by, updated_by
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(request.company_id)
        .bind(request.name)
        .bind(request.code)
        .bind(request.address)
        .bind(request.city)
        .bind(request.state)
        .bind(request.country)
        .bind(request.postal_code)
        .bind(request.phone)
        .bind(request.email)
        .bind(request.manager_name)
        .bind(request.manager_phone)
        .bind(request.warehouse_type)
        .bind(request.storage_capacity)
        .bind(true)
        .bind(request.settings.unwrap_or(serde_json::json!({})))
        .bind(request.metadata.unwrap_or(serde_json::json!({})))
        .bind(created_by)
        .bind(created_by)
        .fetch_one(pool)
        .await?;
        
        Ok(branch)
    }

    pub async fn find_by_id_pg(
        pool: &sqlx::PgPool,
        id: Uuid,
    ) -> Result<Option<Branch>, sqlx::Error> {
        let branch = sqlx::query_as::<sqlx::Postgres, Branch>(
            "SELECT * FROM branches WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(branch)
    }

    pub async fn find_by_id_sqlite(
        pool: &SqlitePool,
        id: Uuid,
    ) -> Result<Option<Branch>, sqlx::Error> {
        let branch = sqlx::query_as::<Sqlite, Branch>(
            "SELECT * FROM branches WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(branch)
    }

    pub async fn find_default_pg(
        pool: &sqlx::PgPool,
        company_id: Uuid,
    ) -> Result<Option<Branch>, sqlx::Error> {
        let branch = sqlx::query_as::<sqlx::Postgres, Branch>(
            "SELECT * FROM branches WHERE company_id = $1 AND is_active = true LIMIT 1"
        )
        .bind(company_id)
        .fetch_optional(pool)
        .await?;
        
        Ok(branch)
    }

    pub async fn find_default_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
    ) -> Result<Option<Branch>, sqlx::Error> {
        let branch = sqlx::query_as::<Sqlite, Branch>(
            "SELECT * FROM branches WHERE company_id = ? AND is_active = true LIMIT 1"
        )
        .bind(company_id)
        .fetch_optional(pool)
        .await?;
        
        Ok(branch)
    }
    
    pub async fn list_by_company(
        pool: &sqlx::PgPool,
        company_id: Uuid,
        active_only: bool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Branch>, sqlx::Error> {
        let branches = if active_only {
            sqlx::query_as::<_, Branch>(
                "SELECT * FROM branches WHERE company_id = $1 AND is_active = true ORDER BY name LIMIT $2 OFFSET $3"
            )
            .bind(company_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as::<_, Branch>(
                "SELECT * FROM branches WHERE company_id = $1 ORDER BY name LIMIT $2 OFFSET $3"
            )
            .bind(company_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?
        };
        
        Ok(branches)
    }
    
    pub async fn get_default_branch(
        pool: &sqlx::PgPool,
        company_id: Uuid,
    ) -> Result<Option<Branch>, sqlx::Error> {
        let branch = sqlx::query_as::<_, Branch>(
            "SELECT * FROM branches WHERE company_id = $1 AND is_active = true"
        )
        .bind(company_id)
        .fetch_optional(pool)
        .await?;
        
        Ok(branch)
    }
    
    pub async fn update(
        pool: &sqlx::PgPool,
        id: Uuid,
        request: UpdateBranchRequest,
        updated_by: Uuid,
    ) -> Result<Branch, sqlx::Error> {
        let mut query = String::from("UPDATE branches SET updated_by = $1, updated_at = NOW()");
        let mut param_index = 2;
        
        if request.name.is_some() {
            query.push_str(&format!(", name = ${}", param_index));
            param_index += 1;
        }
        if request.email.is_some() {
            query.push_str(&format!(", email = ${}", param_index));
            param_index += 1;
        }
        if request.is_active.is_some() {
            query.push_str(&format!(", is_active = ${}", param_index));
            param_index += 1;
        }
        
        query.push_str(&format!(" WHERE id = ${} RETURNING *", param_index));
        
        let mut query_builder = sqlx::query_as::<_, Branch>(&query);
        query_builder = query_builder.bind(updated_by);
        
        if let Some(name) = request.name {
            query_builder = query_builder.bind(name);
        }
        if let Some(email) = request.email {
            query_builder = query_builder.bind(email);
        }
        if let Some(is_active) = request.is_active {
            query_builder = query_builder.bind(is_active);
        }
        
        query_builder = query_builder.bind(id);
        
        let branch = query_builder.fetch_one(pool).await?;
        
        Ok(branch)
    }
    
    pub async fn delete(
        pool: &sqlx::PgPool,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE branches SET is_active = false, updated_at = NOW() WHERE id = $1"
        )
        .bind(id)
        .execute(pool)
        .await?;
        
        Ok(())
    }
}