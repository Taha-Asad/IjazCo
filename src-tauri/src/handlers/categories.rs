// src/handlers/categories.rs
// Category management endpoints
// CRUD operations for product categories with hierarchical support

use axum::{
    extract::{Path, Query, State},
    Json,
};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    config::DbPool,
    handlers::roles::PaginationParams,
    middleware::auth::{verify_company_access, AuthUser},
    models::category::{Category, CreateCategoryRequest, UpdateCategoryRequest},
    utils::{
        error::{AppError, Result},
        response::{created, no_content, paginated, success},
    },
    AppState,
};

// ===== LIST CATEGORIES ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/categories",
    tag = "categories",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 20)"),
        ("search" = Option<String>, Query, description = "Search term"),
        ("parent_id" = Option<Uuid>, Query, description = "Filter by parent category ID"),
    ),
    responses(
        (status = 200, description = "List of categories"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_categories(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Query(params): Query<PaginationParams>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        company_id = %auth_user.company_id,
        "Listing categories"
    );

    // Get pagination values
    let current_page = params.page();
    let per_page = params.per_page();
    let offset = params.offset() as usize;
    let per_page_usize = per_page as usize;

    // Fetch all categories for the company
    let all_categories = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            Category::list_by_company_pg(&pool, auth_user.company_id, false).await?
        }
        DbPool::Sqlite(pool) => {
            Category::list_by_company_sqlite(&pool, auth_user.company_id, false).await?
        }
    };

    let total = all_categories.len() as i64;

    // Apply in-memory pagination
    let paginated_categories: Vec<Category> = all_categories
        .into_iter()
        .skip(offset)
        .take(per_page_usize)
        .collect();

    tracing::debug!(
        count = paginated_categories.len(),
        "Categories retrieved successfully"
    );

    Ok(paginated(paginated_categories, current_page, per_page, total))
}

// ===== GET CATEGORY ENDPOINT =====
#[utoipa::path(
    get,
    path = "/api/v1/categories/{id}",
    tag = "categories",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Category ID")
    ),
    responses(
        (status = 200, description = "Category found", body = Category),
        (status = 404, description = "Category not found"),
        (status = 403, description = "Access denied")
    )
)]
pub async fn get_category(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::debug!(
        user_id = %auth_user.id,
        category_id = %id,
        "Fetching category"
    );
    
    let category = match state.db.as_ref() {
        DbPool::Postgres(pool) => Category::find_by_id_pg(&pool, id).await?,
        DbPool::Sqlite(pool) => Category::find_by_id_sqlite(&pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;
    
    verify_company_access(&auth_user, category.company_id)?;
    
    tracing::info!(
        category_id = %id,
        name = %category.name,
        "Category retrieved successfully"
    );
    
    Ok(Json(category))
}

// ===== CREATE CATEGORY ENDPOINT =====
#[utoipa::path(
    post,
    path = "/api/v1/categories",
    tag = "categories",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateCategoryRequest,
    responses(
        (status = 201, description = "Category created successfully", body = Category),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Category code already exists"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn create_category(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(mut payload): Json<CreateCategoryRequest>,
) -> Result<impl axum::response::IntoResponse> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        category_code = %payload.code,
        "Creating category"
    );
    
    payload.company_id = auth_user.company_id;
    
    // Verify parent category exists if specified
    if let Some(parent_id) = payload.parent_id {
        let parent_exists = match state.db.as_ref() {
            DbPool::Postgres(pool) => {
                Category::find_by_id_pg(&pool, parent_id).await?.is_some()
            }
            DbPool::Sqlite(pool) => {
                Category::find_by_id_sqlite(&pool, parent_id).await?.is_some()
            }
        };
        
        if !parent_exists {
            return Err(AppError::NotFound("Parent category not found".to_string()));
        }
    }
    
    let category = match state.db.as_ref() {
        DbPool::Postgres(pool) => Category::create_pg(&pool, payload, auth_user.id).await?,
        DbPool::Sqlite(pool) => Category::create_sqlite(&pool, payload, auth_user.id).await?,
    };
    
    tracing::info!(
        category_id = %category.id,
        name = %category.name,
        "Category created successfully"
    );
    
    Ok(created("Category created successfully", category))
}

// ===== UPDATE CATEGORY ENDPOINT =====
#[utoipa::path(
    put,
    path = "/api/v1/categories/{id}",
    tag = "categories",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Category ID")
    ),
    request_body = UpdateCategoryRequest,
    responses(
        (status = 200, description = "Category updated successfully", body = Category),
        (status = 404, description = "Category not found"),
        (status = 400, description = "Validation error"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn update_category(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> Result<Json<Category>> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        category_id = %id,
        "Updating category"
    );
    
    let existing_category = match state.db.as_ref() {
        DbPool::Postgres(pool) => Category::find_by_id_pg(&pool, id).await?,
        DbPool::Sqlite(pool) => Category::find_by_id_sqlite(&pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;
    
    verify_company_access(&auth_user, existing_category.company_id)?;
    
    if let Some(parent_id) = payload.parent_id {
        if parent_id == id {
            return Err(AppError::ValidationError(
                "Category cannot be its own parent".to_string()
            ));
        }
    }
    
    let updated_category = match state.db.as_ref() {
        DbPool::Postgres(pool) => Category::update_pg(&pool, id, payload, auth_user.id).await?,
        DbPool::Sqlite(pool) => Category::update_sqlite(&pool, id, payload, auth_user.id).await?,
    };
    
    tracing::info!(
        category_id = %id,
        "Category updated successfully"
    );
    
    Ok(Json(updated_category))
}

// ===== DELETE CATEGORY ENDPOINT =====
#[utoipa::path(
    delete,
    path = "/api/v1/categories/{id}",
    tag = "categories",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Category ID")
    ),
    responses(
        (status = 204, description = "Category deleted successfully"),
        (status = 404, description = "Category not found"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "Category has items or subcategories")
    )
)]
pub async fn delete_category(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        category_id = %id,
        "Deleting category"
    );
    
    let category = match state.db.as_ref() {
        DbPool::Postgres(pool) => Category::find_by_id_pg(&pool, id).await?,
        DbPool::Sqlite(pool) => Category::find_by_id_sqlite(&pool, id).await?,
    }
    .ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;
    
    verify_company_access(&auth_user, category.company_id)?;
    
    // Check for items in this category
    let items_count = match state.db.as_ref() {
        DbPool::Postgres(pool) => Category::count_items_pg(&pool, id).await?,
        DbPool::Sqlite(pool) => Category::count_items_sqlite(&pool, id).await?,
    };
    
    if items_count > 0 {
        return Err(AppError::OperationNotAllowed(
            format!("Cannot delete category: {} items are in this category. Please reassign or remove items first.", items_count)
        ));
    }
    
    // Reassign subcategories to top-level before deletion
    match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            Category::reassign_subcategories_to_top_pg(&pool, id).await?;
            Category::delete_pg(&pool, id).await?;
        }
        DbPool::Sqlite(pool) => {
            Category::reassign_subcategories_to_top_sqlite(&pool, id).await?;
            Category::delete_sqlite(&pool, id).await?;
        }
    };
    
    tracing::info!(
        category_id = %id,
        name = %category.name,
        "Category deleted successfully"
    );
    
    Ok(no_content())
}

pub fn categories_router() -> axum::Router<Arc<AppState>> {
    use axum::routing::{delete, get, post, put};

    axum::Router::new()
        .route("/", get(list_categories).post(create_category))
        .route("/:id", get(get_category).put(update_category).delete(delete_category))
}