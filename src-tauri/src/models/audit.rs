// src/models/audit.rs
// Audit logging models
// Tracks all system changes for compliance and security

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;

// ===== AUDIT ACTION ENUM =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "audit_action", rename_all = "lowercase")]
pub enum AuditAction {
    #[serde(rename = "create")]
    Create,
    
    #[serde(rename = "read")]
    Read,
    
    #[serde(rename = "update")]
    Update,
    
    #[serde(rename = "delete")]
    Delete,
    
    #[serde(rename = "login")]
    Login,
    
    #[serde(rename = "logout")]
    Logout,
    
    #[serde(rename = "approval")]
    Approval,
    
    #[serde(rename = "rejection")]
    Rejection,
}

// ===== AUDIT LOG MODEL =====
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct AuditLog {
    pub id: Uuid,
    
    pub company_id: Uuid,
    
    pub action: AuditAction,
    
    pub entity_type: String,
    
    pub entity_id: Option<Uuid>,
    
    pub user_id: Uuid,
    
    pub username: String,
    
    pub ip_address: Option<String>,
    
    pub user_agent: Option<String>,
    
    pub description: String,
    
    #[sqlx(default)]
    pub old_values: serde_json::Value,
    
    #[sqlx(default)]
    pub new_values: serde_json::Value,
    
    #[sqlx(default)]
    pub metadata: serde_json::Value,
    
    pub created_at: DateTime<Utc>,
}

// ===== CREATE AUDIT LOG REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateAuditLogRequest {
    pub company_id: Uuid,
    
    pub action: AuditAction,
    
    pub entity_type: String,
    
    pub entity_id: Option<Uuid>,
    
    pub user_id: Uuid,
    
    pub ip_address: Option<String>,
    
    pub user_agent: Option<String>,
    
    pub description: String,
    
    pub old_values: Option<serde_json::Value>,
    
    pub new_values: Option<serde_json::Value>,
    
    pub metadata: Option<serde_json::Value>,
}

// ===== AUDIT LOG FILTER =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct AuditLogFilter {
    pub company_id: Option<Uuid>,
    
    pub action: Option<AuditAction>,
    
    pub entity_type: Option<String>,
    
    pub entity_id: Option<Uuid>,
    
    pub user_id: Option<Uuid>,
    
    pub date_from: Option<chrono::NaiveDate>,
    
    pub date_to: Option<chrono::NaiveDate>,
}

// ===== AUDIT LOG DATABASE OPERATIONS =====
impl AuditLog {
    pub async fn create(
        pool: &sqlx::PgPool,
        request: CreateAuditLogRequest,
    ) -> Result<AuditLog, sqlx::Error> {
        // Get username
        let username: String = sqlx::query_scalar(
            "SELECT username FROM users WHERE id = $1"
        )
        .bind(request.user_id)
        .fetch_one(pool)
        .await?;
        
        let audit_log = sqlx::query_as::<_, AuditLog>(
            r#"
            INSERT INTO audit_logs (
                company_id, action, entity_type, entity_id,
                user_id, username, ip_address, user_agent,
                description, old_values, new_values, metadata
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *
            "#
        )
        .bind(request.company_id)
        .bind(request.action)
        .bind(request.entity_type)
        .bind(request.entity_id)
        .bind(request.user_id)
        .bind(username)
        .bind(request.ip_address)
        .bind(request.user_agent)
        .bind(request.description)
        .bind(request.old_values.unwrap_or(serde_json::json!({})))
        .bind(request.new_values.unwrap_or(serde_json::json!({})))
        .bind(request.metadata.unwrap_or(serde_json::json!({})))
        .fetch_one(pool)
        .await?;
        
        Ok(audit_log)
    }
    
    pub async fn log_action(
        pool: &sqlx::PgPool,
        company_id: Uuid,
        user_id: Uuid,
        action: AuditAction,
        entity_type: &str,
        entity_id: Option<Uuid>,
        description: &str,
        old_values: Option<serde_json::Value>,
        new_values: Option<serde_json::Value>,
        ip_address: Option<String>,
    ) -> Result<AuditLog, sqlx::Error> {
        let request = CreateAuditLogRequest {
            company_id,
            action,
            entity_type: entity_type.to_string(),
            entity_id,
            user_id,
            ip_address,
            user_agent: None,
            description: description.to_string(),
            old_values,
            new_values,
            metadata: None,
        };
        
        Self::create(pool, request).await
    }
    
    pub async fn find_by_id(
        pool: &sqlx::PgPool,
        id: Uuid,
    ) -> Result<Option<AuditLog>, sqlx::Error> {
        let log = sqlx::query_as::<_, AuditLog>(
            "SELECT * FROM audit_logs WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(log)
    }
    
    pub async fn list_by_company(
        pool: &sqlx::PgPool,
        company_id: Uuid,
        filter: Option<AuditLogFilter>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AuditLog>, sqlx::Error> {
        let mut query = String::from(
            "SELECT * FROM audit_logs WHERE company_id = $1"
        );
        let mut params = vec![company_id];
        let mut param_index = 2;
        
        if let Some(filter) = filter {
            if let Some(action) = filter.action {
                query.push_str(&format!(" AND action = ${}", param_index));
                param_index += 1;
            }
            if let Some(entity_type) = filter.entity_type {
                query.push_str(&format!(" AND entity_type = ${}", param_index));
                param_index += 1;
            }
            if let Some(entity_id) = filter.entity_id {
                query.push_str(&format!(" AND entity_id = ${}", param_index));
                param_index += 1;
            }
            if let Some(user_id) = filter.user_id {
                query.push_str(&format!(" AND user_id = ${}", param_index));
                param_index += 1;
            }
            if let Some(date_from) = filter.date_from {
                query.push_str(&format!(" AND created_at >= ${}::date", param_index));
                param_index += 1;
            }
            if let Some(date_to) = filter.date_to {
                query.push_str(&format!(" AND created_at <= ${}::date", param_index));
                param_index += 1;
            }
        }
        
        query.push_str(&format!(" ORDER BY created_at DESC LIMIT ${} OFFSET ${}", param_index, param_index + 1));
        
        let mut query_builder = sqlx::query_as::<_, AuditLog>(&query);
        
        for param in params {
            query_builder = query_builder.bind(param);
        }
        
        query_builder = query_builder.bind(limit).bind(offset);
        
        let logs = query_builder.fetch_all(pool).await?;
        
        Ok(logs)
    }
    
    pub async fn list_by_entity(
        pool: &sqlx::PgPool,
        entity_type: &str,
        entity_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AuditLog>, sqlx::Error> {
        let logs = sqlx::query_as::<_, AuditLog>(
            "SELECT * FROM audit_logs WHERE entity_type = $1 AND entity_id = $2 ORDER BY created_at DESC LIMIT $3 OFFSET $4"
        )
        .bind(entity_type)
        .bind(entity_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(logs)
    }
    
    pub async fn list_by_user(
        pool: &sqlx::PgPool,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AuditLog>, sqlx::Error> {
        let logs = sqlx::query_as::<_, AuditLog>(
            "SELECT * FROM audit_logs WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(logs)
    }
    
    pub async fn cleanup_old_logs(
        pool: &sqlx::PgPool,
        days_to_keep: i32,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM audit_logs WHERE created_at < NOW() - ($1 || ' days')::INTERVAL"
        )
        .bind(days_to_keep)
        .execute(pool)
        .await?;
        
        Ok(result.rows_affected())
    }
}

// ===== HELPER FUNCTIONS =====

pub async fn log_create<T: Serialize>(
    pool: &sqlx::PgPool,
    company_id: Uuid,
    user_id: Uuid,
    entity_type: &str,
    entity_id: Uuid,
    new_value: &T,
    ip_address: Option<String>,
) -> Result<AuditLog, sqlx::Error> {
    AuditLog::log_action(
        pool,
        company_id,
        user_id,
        AuditAction::Create,
        entity_type,
        Some(entity_id),
        &format!("Created {}", entity_type),
        None,
        Some(serde_json::to_value(new_value).unwrap_or(serde_json::json!({}))),
        ip_address,
    )
    .await
}

pub async fn log_update<T: Serialize>(
    pool: &sqlx::PgPool,
    company_id: Uuid,
    user_id: Uuid,
    entity_type: &str,
    entity_id: Uuid,
    old_value: &T,
    new_value: &T,
    ip_address: Option<String>,
) -> Result<AuditLog, sqlx::Error> {
    AuditLog::log_action(
        pool,
        company_id,
        user_id,
        AuditAction::Update,
        entity_type,
        Some(entity_id),
        &format!("Updated {}", entity_type),
        Some(serde_json::to_value(old_value).unwrap_or(serde_json::json!({}))),
        Some(serde_json::to_value(new_value).unwrap_or(serde_json::json!({}))),
        ip_address,
    )
    .await
}

pub async fn log_delete(
    pool: &sqlx::PgPool,
    company_id: Uuid,
    user_id: Uuid,
    entity_type: &str,
    entity_id: Uuid,
    old_value: &serde_json::Value,
    ip_address: Option<String>,
) -> Result<AuditLog, sqlx::Error> {
    AuditLog::log_action(
        pool,
        company_id,
        user_id,
        AuditAction::Delete,
        entity_type,
        Some(entity_id),
        &format!("Deleted {}", entity_type),
        Some(old_value.clone()),
        None,
        ip_address,
    )
    .await
}

pub async fn log_login(
    pool: &sqlx::PgPool,
    company_id: Uuid,
    user_id: Uuid,
    username: &str,
    ip_address: Option<String>,
) -> Result<AuditLog, sqlx::Error> {
    AuditLog::log_action(
        pool,
        company_id,
        user_id,
        AuditAction::Login,
        "user",
        Some(user_id),
        &format!("User {} logged in", username),
        None,
        None,
        ip_address,
    )
    .await
}