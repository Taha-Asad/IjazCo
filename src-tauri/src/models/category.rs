// src/models/category.rs
// Product category management models
// Handles hierarchical product categorization

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{ FromRow, Sqlite, SqlitePool, PgPool, Postgres };
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;

// ===== CATEGORY MODEL =====
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Category {
    pub id: Uuid,
    pub company_id: Uuid,
    pub parent_id: Option<Uuid>,
    
    #[schema(example = "CAT-MICRO")]
    pub code: String,
    
    #[schema(example = "Microscopes")]
    pub name: String,
    
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub sort_order: i32,
    pub is_active: bool,
    
    #[sqlx(default)]
    pub metadata: serde_json::Value,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// ===== CATEGORY WITH ITEM COUNT =====
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CategoryWithCount {
    #[serde(flatten)]
    pub category: Category,
    pub item_count: i64,
    pub subcategory_count: i64,
}

// ===== CREATE CATEGORY REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateCategoryRequest {
    pub company_id: Uuid,
    pub parent_id: Option<Uuid>,
    
    #[validate(length(min = 1, max = 50))]
    #[schema(example = "CAT-MICRO")]
    pub code: String,
    
    #[validate(length(min = 1, max = 255))]
    #[schema(example = "Microscopes")]
    pub name: String,
    
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub sort_order: Option<i32>,
    pub metadata: Option<serde_json::Value>,
}

// ===== UPDATE CATEGORY REQUEST =====
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateCategoryRequest {
    pub parent_id: Option<Uuid>,
    
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
    pub metadata: Option<serde_json::Value>,
}

// ===== CATEGORY DATABASE OPERATIONS =====
impl Category {
    // ===== CREATE =====
    pub async fn create_pg(
        pool: &PgPool,
        request: CreateCategoryRequest,
        created_by: Uuid,
    ) -> Result<Category, sqlx::Error> {
        sqlx::query_as::<Postgres, Category>(
            r#"
            INSERT INTO categories (
                company_id, parent_id, code, name, description,
                image_url, sort_order, is_active, metadata,
                created_by, updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#
        )
        .bind(request.company_id)
        .bind(request.parent_id)
        .bind(request.code)
        .bind(request.name)
        .bind(request.description)
        .bind(request.image_url)
        .bind(request.sort_order.unwrap_or(0))
        .bind(true)
        .bind(request.metadata.unwrap_or(serde_json::json!({})))
        .bind(created_by)
        .bind(created_by)
        .fetch_one(pool)
        .await
    }

    pub async fn create_sqlite(
        pool: &SqlitePool,
        request: CreateCategoryRequest,
        created_by: Uuid,
    ) -> Result<Category, sqlx::Error> {
        sqlx::query_as::<Sqlite, Category>(
            r#"
            INSERT INTO categories (
                company_id, parent_id, code, name, description,
                image_url, sort_order, is_active, metadata,
                created_by, updated_by
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(request.company_id)
        .bind(request.parent_id)
        .bind(request.code)
        .bind(request.name)
        .bind(request.description)
        .bind(request.image_url)
        .bind(request.sort_order.unwrap_or(0))
        .bind(true)
        .bind(request.metadata.unwrap_or(serde_json::json!({})))
        .bind(created_by)
        .bind(created_by)
        .fetch_one(pool)
        .await
    }

    // ===== FIND BY ID =====
    pub async fn find_by_id_pg(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<Category>, sqlx::Error> {
        sqlx::query_as::<Postgres, Category>("SELECT * FROM categories WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_id_sqlite(
        pool: &SqlitePool,
        id: Uuid,
    ) -> Result<Option<Category>, sqlx::Error> {
        sqlx::query_as::<Sqlite, Category>("SELECT * FROM categories WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    // ===== LIST BY COMPANY =====
    pub async fn list_by_company_pg(
        pool: &PgPool,
        company_id: Uuid,
        active_only: bool,
    ) -> Result<Vec<Category>, sqlx::Error> {
        let sql = if active_only {
            "SELECT * FROM categories WHERE company_id = $1 AND is_active = true ORDER BY sort_order, name"
        } else {
            "SELECT * FROM categories WHERE company_id = $1 ORDER BY sort_order, name"
        };
        
        sqlx::query_as::<Postgres, Category>(sql)
            .bind(company_id)
            .fetch_all(pool)
            .await
    }

    pub async fn list_by_company_sqlite(
        pool: &SqlitePool,
        company_id: Uuid,
        active_only: bool,
    ) -> Result<Vec<Category>, sqlx::Error> {
        let sql = if active_only {
            "SELECT * FROM categories WHERE company_id = ? AND is_active = 1 ORDER BY sort_order, name"
        } else {
            "SELECT * FROM categories WHERE company_id = ? ORDER BY sort_order, name"
        };
        
        sqlx::query_as::<Sqlite, Category>(sql)
            .bind(company_id)
            .fetch_all(pool)
            .await
    }

    // ===== GET SUBCATEGORIES =====
    pub async fn get_subcategories_pg(
        pool: &PgPool,
        parent_id: Uuid,
    ) -> Result<Vec<Category>, sqlx::Error> {
        sqlx::query_as::<Postgres, Category>(
            "SELECT * FROM categories WHERE parent_id = $1 ORDER BY sort_order"
        )
        .bind(parent_id)
        .fetch_all(pool)
        .await
    }

    pub async fn get_subcategories_sqlite(
        pool: &SqlitePool,
        parent_id: Uuid,
    ) -> Result<Vec<Category>, sqlx::Error> {
        sqlx::query_as::<Sqlite, Category>(
            "SELECT * FROM categories WHERE parent_id = ? ORDER BY sort_order"
        )
        .bind(parent_id)
        .fetch_all(pool)
        .await
    }

    // ===== COUNT SUBCATEGORIES =====
    pub async fn count_subcategories_pg(
        pool: &PgPool,
        parent_id: Uuid,
    ) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM categories WHERE parent_id = $1"
        )
        .bind(parent_id)
        .fetch_one(pool)
        .await
    }

    pub async fn count_subcategories_sqlite(
        pool: &SqlitePool,
        parent_id: Uuid,
    ) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM categories WHERE parent_id = ?"
        )
        .bind(parent_id)
        .fetch_one(pool)
        .await
    }

    // ===== COUNT ITEMS IN CATEGORY =====
    pub async fn count_items_pg(
        pool: &PgPool,
        category_id: Uuid,
    ) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM inventory_items WHERE category_id = $1 AND is_active = true"
        )
        .bind(category_id)
        .fetch_one(pool)
        .await
    }

    pub async fn count_items_sqlite(
        pool: &SqlitePool,
        category_id: Uuid,
    ) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM inventory_items WHERE category_id = ? AND is_active = true"
        )
        .bind(category_id)
        .fetch_one(pool)
        .await
    }

    // ===== UPDATE =====
    pub async fn update_pg(
        pool: &PgPool,
        id: Uuid,
        request: UpdateCategoryRequest,
        updated_by: Uuid,
    ) -> Result<Category, sqlx::Error> {
        let mut builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("UPDATE categories SET ");
        
        builder.push("updated_by = ").push_bind(updated_by);
        builder.push(", updated_at = NOW()");
        
        if let Some(parent_id) = request.parent_id {
            builder.push(", parent_id = ").push_bind(parent_id);
        }
        if let Some(name) = &request.name {
            builder.push(", name = ").push_bind(name);
        }
        if let Some(description) = &request.description {
            builder.push(", description = ").push_bind(description);
        }
        if let Some(image_url) = &request.image_url {
            builder.push(", image_url = ").push_bind(image_url);
        }
        if let Some(sort_order) = request.sort_order {
            builder.push(", sort_order = ").push_bind(sort_order);
        }
        if let Some(is_active) = request.is_active {
            builder.push(", is_active = ").push_bind(is_active);
        }
        if let Some(metadata) = &request.metadata {
            builder.push(", metadata = ").push_bind(metadata);
        }
        
        builder.push(" WHERE id = ").push_bind(id);
        builder.push(" RETURNING *");
        
        builder.build_query_as::<Category>().fetch_one(pool).await
    }

    pub async fn update_sqlite(
        pool: &SqlitePool,
        id: Uuid,
        request: UpdateCategoryRequest,
        updated_by: Uuid,
    ) -> Result<Category, sqlx::Error> {
        let mut builder = sqlx::QueryBuilder::<sqlx::Sqlite>::new("UPDATE categories SET ");
        
        builder.push("updated_by = ").push_bind(updated_by);
        builder.push(", updated_at = CURRENT_TIMESTAMP");
        
        if let Some(parent_id) = request.parent_id {
            builder.push(", parent_id = ").push_bind(parent_id);
        }
        if let Some(name) = &request.name {
            builder.push(", name = ").push_bind(name);
        }
        if let Some(description) = &request.description {
            builder.push(", description = ").push_bind(description);
        }
        if let Some(image_url) = &request.image_url {
            builder.push(", image_url = ").push_bind(image_url);
        }
        if let Some(sort_order) = request.sort_order {
            builder.push(", sort_order = ").push_bind(sort_order);
        }
        if let Some(is_active) = request.is_active {
            builder.push(", is_active = ").push_bind(if is_active { 1 } else { 0 });
        }
        if let Some(metadata) = &request.metadata {
            builder.push(", metadata = ").push_bind(metadata);
        }
        
        builder.push(" WHERE id = ").push_bind(id);
        builder.push(" RETURNING *");
        
        builder.build_query_as::<Category>().fetch_one(pool).await
    }

    // ===== REASSIGN SUBCATEGORIES TO TOP-LEVEL =====
    pub async fn reassign_subcategories_to_top_pg(
        pool: &PgPool,
        parent_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE categories SET parent_id = NULL WHERE parent_id = $1")
            .bind(parent_id)
            .execute(pool)
            .await
            .map(|_| ())
    }

    pub async fn reassign_subcategories_to_top_sqlite(
        pool: &SqlitePool,
        parent_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE categories SET parent_id = NULL WHERE parent_id = ?")
            .bind(parent_id)
            .execute(pool)
            .await
            .map(|_| ())
    }

    // ===== DELETE =====
    pub async fn delete_pg(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM categories WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map(|_| ())
    }

    pub async fn delete_sqlite(pool: &SqlitePool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM categories WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
            .map(|_| ())
    }
}