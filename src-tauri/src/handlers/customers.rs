// src/handlers/customers.rs
// Customer management endpoints
// CRUD operations for customers with statistics

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    config::DbPool,
    middleware::auth::{verify_company_access, AuthUser},
    models::customer::{CreateCustomerRequest, Customer, CustomerWithStats, UpdateCustomerRequest},
    utils::{
        error::{AppError, Result},
        response::{created, no_content, paginated, success},
    },
    AppState,
};

use super::users::PaginationParams;

// ===== CUSTOMER SEARCH PARAMETERS =====
#[derive(Debug, Deserialize, ToSchema)]
pub struct CustomerSearchParams {
    pub search: Option<String>,
    
    #[serde(default = "default_true")]
    pub active_only: bool,
    
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

fn default_true() -> bool { true }

// ===== LIST CUSTOMERS ENDPOINT =====
pub async fn list_customers(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Query(params): Query<CustomerSearchParams>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        company_id = %auth_user.company_id,
        search = ?params.search,
        "Listing customers"
    );
    
    // Fetch customers based on search criteria
    let (customers, total_count) = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            let customers = if let Some(ref search_term) = params.search {
                Customer::search_pg(
                    pool,
                    auth_user.company_id,
                    search_term,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            } else {
                Customer::list_by_company_pg(
                    pool,
                    auth_user.company_id,
                    params.active_only,
                    params.pagination.limit(),
                    params.pagination.offset(),
                ).await?
            };
            
            let total_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM customers WHERE company_id = $1 AND ($2 = false OR is_active = true)"
            )
            .bind(auth_user.company_id)
            .bind(params.active_only)
            .fetch_one(pool)
            .await?;

            (customers, total_count)
        }
        _ => {
            return Err(AppError::Internal("SQLite not supported".to_string()));
        }
    };
    
    tracing::debug!(
        count = customers.len(),
        total = total_count,
        "Customers retrieved successfully"
    );
    
    Ok(paginated(
        customers,
        params.pagination.page(),
        params.pagination.per_page(),
        total_count,
    ))
}

// ===== GET CUSTOMER ENDPOINT =====
pub async fn get_customer(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::debug!(
        user_id = %auth_user.id,
        customer_id = %id,
        "Fetching customer"
    );
    
    // Fetch customer with statistics
    let customer_with_stats = match state.db.as_ref() {
        DbPool::Postgres(pool) => Customer::get_with_stats_pg(pool, id).await?,
        _ => return Err(AppError::Internal("SQLite not supported".to_string())),
    }
    .ok_or_else(|| AppError::NotFound("Customer not found".to_string()))?;
    
    // Verify customer belongs to same company
    verify_company_access(&auth_user, customer_with_stats.customer.company_id)?;
    
    tracing::info!(
        customer_id = %id,
        name = %customer_with_stats.customer.name,
        "Customer retrieved successfully"
    );
    
    // Return flattened customer with stats
    Ok(Json(serde_json::json!({
        "id": customer_with_stats.customer.id,
        "company_id": customer_with_stats.customer.company_id,
        "customer_code": customer_with_stats.customer.customer_code,
        "name": customer_with_stats.customer.name,
        "contact_person": customer_with_stats.customer.contact_person,
        "email": customer_with_stats.customer.email,
        "phone": customer_with_stats.customer.phone,
        "mobile": customer_with_stats.customer.mobile,
        "tax_id": customer_with_stats.customer.tax_id,
        "billing_address": customer_with_stats.customer.billing_address,
        "billing_city": customer_with_stats.customer.billing_city,
        "billing_state": customer_with_stats.customer.billing_state,
        "billing_country": customer_with_stats.customer.billing_country,
        "billing_postal_code": customer_with_stats.customer.billing_postal_code,
        "shipping_address": customer_with_stats.customer.shipping_address,
        "shipping_city": customer_with_stats.customer.shipping_city,
        "shipping_state": customer_with_stats.customer.shipping_state,
        "shipping_country": customer_with_stats.customer.shipping_country,
        "shipping_postal_code": customer_with_stats.customer.shipping_postal_code,
        "credit_limit": customer_with_stats.customer.credit_limit,
        "credit_days": customer_with_stats.customer.credit_days,
        "discount_percentage": customer_with_stats.customer.discount_percentage,
        "is_active": customer_with_stats.customer.is_active,
        "tags": customer_with_stats.customer.tags,
        "notes": customer_with_stats.customer.notes,
        "metadata": customer_with_stats.customer.metadata,
        "created_at": customer_with_stats.customer.created_at,
        "updated_at": customer_with_stats.customer.updated_at,
        "created_by": customer_with_stats.customer.created_by,
        "updated_by": customer_with_stats.customer.updated_by,
        "total_invoices": customer_with_stats.total_invoices,
        "total_spent": customer_with_stats.total_sales,
        "current_balance": customer_with_stats.outstanding_balance,
    })))
}

// ===== CREATE CUSTOMER ENDPOINT =====
pub async fn create_customer(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(mut payload): Json<CreateCustomerRequest>,
) -> Result<impl axum::response::IntoResponse> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        customer_code = %payload.customer_code,
        "Creating customer"
    );
    
    // Force company_id to match authenticated user's company
    let company_id = auth_user.company_id;
    
    // Check if customer code already exists
    let code_exists = match state.db.as_ref() {
        DbPool::Postgres(pool) => {
            Customer::find_by_code_pg(pool, company_id, &payload.customer_code)
                .await?
                .is_some()
        }
        _ => return Err(AppError::Internal("SQLite not supported".to_string())),
    };
    
    if code_exists {
        tracing::warn!(
            customer_code = %payload.customer_code,
            "Customer creation failed: code already exists"
        );
        return Err(AppError::DuplicateKey("Customer code already exists".to_string()));
    }
    
    // Create customer
    let customer = match state.db.as_ref() {
        DbPool::Postgres(pool) => Customer::create_pg(pool, payload, company_id, auth_user.id).await?,
        _ => return Err(AppError::Internal("SQLite not supported".to_string())),
    };
    
    tracing::info!(
        customer_id = %customer.id,
        name = %customer.name,
        "Customer created successfully"
    );
    
    Ok(created("Customer created successfully", customer))
}

// ===== UPDATE CUSTOMER ENDPOINT =====
pub async fn update_customer(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCustomerRequest>,
) -> Result<impl axum::response::IntoResponse> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    tracing::info!(
        user_id = %auth_user.id,
        customer_id = %id,
        "Updating customer"
    );
    
    // Fetch existing customer
    let existing = match state.db.as_ref() {
        DbPool::Postgres(pool) => Customer::find_by_id_pg(pool, id).await?,
        _ => return Err(AppError::Internal("SQLite not supported".to_string())),
    }
    .ok_or_else(|| AppError::NotFound("Customer not found".to_string()))?;
    
    // Verify customer belongs to same company
    verify_company_access(&auth_user, existing.company_id)?;
    
    // Update customer
    let customer = match state.db.as_ref() {
        DbPool::Postgres(pool) => Customer::update_pg(pool, id, payload, auth_user.id).await?,
        _ => return Err(AppError::Internal("SQLite not supported".to_string())),
    };
    
    tracing::info!(
        customer_id = %id,
        name = %customer.name,
        "Customer updated successfully"
    );
    
    Ok(success("Customer updated successfully", customer))
}

// ===== DELETE CUSTOMER ENDPOINT =====
pub async fn delete_customer(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    tracing::info!(
        user_id = %auth_user.id,
        customer_id = %id,
        "Deleting customer"
    );
    
    // Fetch existing customer
    let existing = match state.db.as_ref() {
        DbPool::Postgres(pool) => Customer::find_by_id_pg(pool, id).await?,
        _ => return Err(AppError::Internal("SQLite not supported".to_string())),
    }
    .ok_or_else(|| AppError::NotFound("Customer not found".to_string()))?;
    
    // Verify customer belongs to same company
    verify_company_access(&auth_user, existing.company_id)?;
    
    // Soft delete customer
    match state.db.as_ref() {
        DbPool::Postgres(pool) => Customer::delete_pg(pool, id, auth_user.id).await?,
        _ => return Err(AppError::Internal("SQLite not supported".to_string())),
    };
    
    tracing::info!(
        customer_id = %id,
        name = %existing.name,
        "Customer deleted successfully"
    );
    
    Ok(no_content())
}

// ===== ROUTER =====
pub fn customers_router() -> axum::Router<Arc<AppState>> {
    use axum::routing::{get, post, put, delete};
    
    axum::Router::new()
        .route("/", get(list_customers).post(create_customer))
        .route("/:id", get(get_customer).put(update_customer).delete(delete_customer))
}
